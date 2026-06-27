use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};
use tokio::sync::broadcast;

const HISTORY_LIMIT: usize = 200;
const OPENCODE_APPLY_SOURCE: &str = "opencode.apply_patch";
const DURABLE_LOG_PATH: &str = ".forge/change-events.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub seq: u64,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeBusStatus {
    pub event_bus: String,
    pub bridge_shape: String,
    pub history_limit: usize,
    pub count: usize,
    pub first_seq: Option<u64>,
    pub last_seq: Option<u64>,
    pub by_type: BTreeMap<String, usize>,
    pub by_source: BTreeMap<String, usize>,
    pub latest_files: Vec<String>,
    pub durable: bool,
    pub durable_log_path: Option<String>,
    pub durable_replay_count: usize,
    pub opencode_sources: Vec<String>,
}

#[derive(Clone)]
pub struct ChangeBus { inner: Arc<Inner> }

struct Inner {
    next_seq: AtomicU64,
    history: Mutex<VecDeque<ChangeEvent>>,
    tx: broadcast::Sender<ChangeEvent>,
    durable_log: Option<PathBuf>,
    durable_replay_count: usize,
}

impl fmt::Debug for ChangeBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("ChangeBus").finish_non_exhaustive() }
}

impl ChangeBus {
    pub fn new() -> Self { Self::new_with_workspace(None::<String>) }

    pub fn new_with_workspace(workspace_root: Option<String>) -> Self {
        let durable_log = workspace_root.map(|root| Path::new(&root).join(DURABLE_LOG_PATH));
        let replayed = durable_log.as_ref().map(replay_durable_events).unwrap_or_default();
        let next_seq = replayed.back().map(|event| event.seq.saturating_add(1)).unwrap_or(1);
        let replay_count = replayed.len();
        let (tx, _) = broadcast::channel(HISTORY_LIMIT);
        Self {
            inner: Arc::new(Inner {
                next_seq: AtomicU64::new(next_seq),
                history: Mutex::new(replayed),
                tx,
                durable_log,
                durable_replay_count: replay_count,
            }),
        }
    }

    pub fn publish(&self, event_type: impl Into<String>, source: impl Into<String>, payload: serde_json::Value) -> ChangeEvent {
        let event = ChangeEvent {
            seq: self.inner.next_seq.fetch_add(1, Ordering::SeqCst),
            event_type: event_type.into(),
            source: source.into(),
            payload,
            created_at: chrono::Utc::now(),
        };
        self.store(event.clone());
        let _ = self.inner.tx.send(event.clone());
        event
    }

    pub fn recent(&self) -> Vec<ChangeEvent> {
        self.inner.history.lock().map(|h| h.iter().cloned().collect()).unwrap_or_default()
    }

    pub fn status(&self) -> ChangeBusStatus {
        let events = self.recent();
        let mut by_type = BTreeMap::new();
        let mut by_source = BTreeMap::new();
        let mut latest_files = Vec::new();
        for event in &events {
            *by_type.entry(event.event_type.clone()).or_insert(0) += 1;
            *by_source.entry(event.source.clone()).or_insert(0) += 1;
            if let Some(file) = event.payload.get("file").and_then(serde_json::Value::as_str) {
                if !latest_files.iter().any(|hit| hit == file) { latest_files.push(file.to_string()); }
            }
        }
        latest_files.truncate(12);
        ChangeBusStatus {
            event_bus: "change_bus".to_string(),
            bridge_shape: "opencode_event_v2_bridge_status".to_string(),
            history_limit: HISTORY_LIMIT,
            count: events.len(),
            first_seq: events.first().map(|event| event.seq),
            last_seq: events.last().map(|event| event.seq),
            by_type,
            by_source,
            latest_files,
            durable: self.inner.durable_log.is_some(),
            durable_log_path: self.inner.durable_log.as_ref().map(|path| path.display().to_string()),
            durable_replay_count: self.inner.durable_replay_count,
            opencode_sources: opencode_event_sources(),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> { self.inner.tx.subscribe() }

    fn store(&self, event: ChangeEvent) {
        if let Ok(mut history) = self.inner.history.lock() {
            history.push_back(event.clone());
            while history.len() > HISTORY_LIMIT { history.pop_front(); }
        }
        if let Some(path) = &self.inner.durable_log { append_durable_event(path, &event); }
    }
}

impl Default for ChangeBus { fn default() -> Self { Self::new() } }

fn replay_durable_events(path: &PathBuf) -> VecDeque<ChangeEvent> {
    let Some(text) = std::fs::read_to_string(path).ok() else { return VecDeque::new(); };
    text.lines()
        .filter_map(|line| serde_json::from_str::<ChangeEvent>(line).ok())
        .rev()
        .take(HISTORY_LIMIT)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn append_durable_event(path: &PathBuf, event: &ChangeEvent) {
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    let Ok(line) = serde_json::to_string(event) else { return; };
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{line}");
    }
}

fn opencode_event_sources() -> Vec<String> {
    let tool_root = ["packages/opencode/src/tool/"].concat();
    vec![
        "packages/opencode/src/event-v2-bridge.ts".to_string(),
        "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts".to_string(),
        [tool_root.as_str(), "write.ts"].concat(),
        [tool_root.as_str(), "edit.ts"].concat(),
        [tool_root.as_str(), "apply_", "patch.ts"].concat(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_recent_events_with_sequence_numbers() {
        let bus = ChangeBus::new();
        bus.publish("file.updated", "test", serde_json::json!({"file":"a.rs"}));
        bus.publish("file.edited", "test", serde_json::json!({"file":"a.rs"}));
        let events = bus.recent();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].seq, 1);
        assert_eq!(events[1].event_type, "file.edited");
    }

    #[test]
    fn status_counts_event_types_sources_and_files() {
        let bus = ChangeBus::new();
        bus.publish("filesystem.edited", OPENCODE_APPLY_SOURCE, serde_json::json!({"file":"a.rs"}));
        bus.publish("watcher.updated", OPENCODE_APPLY_SOURCE, serde_json::json!({"file":"a.rs"}));
        let status = bus.status();
        assert_eq!(status.count, 2);
        assert_eq!(status.by_source.get(OPENCODE_APPLY_SOURCE), Some(&2));
        assert_eq!(status.latest_files, vec!["a.rs".to_string()]);
        assert_eq!(status.bridge_shape, "opencode_event_v2_bridge_status");
    }

    #[test]
    fn replays_durable_event_log_and_continues_sequence() {
        let root = std::env::temp_dir().join(format!("forge-change-bus-{}", uuid::Uuid::new_v4()));
        let root_string = root.display().to_string();
        let first = ChangeBus::new_with_workspace(Some(root_string.clone()));
        first.publish("filesystem.edited", OPENCODE_APPLY_SOURCE, serde_json::json!({"file":"a.rs"}));
        let second = ChangeBus::new_with_workspace(Some(root_string.clone()));
        assert_eq!(second.status().durable_replay_count, 1);
        let event = second.publish("watcher.updated", OPENCODE_APPLY_SOURCE, serde_json::json!({"file":"a.rs"}));
        assert_eq!(event.seq, 2);
        let _ = std::fs::remove_dir_all(root);
    }
}
