use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};
use tokio::sync::broadcast;

const HISTORY_LIMIT: usize = 200;
const OPENCODE_APPLY_SOURCE: &str = "opencode.apply_patch";

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
    pub opencode_sources: Vec<String>,
}

#[derive(Clone)]
pub struct ChangeBus { inner: Arc<Inner> }

struct Inner {
    next_seq: AtomicU64,
    history: Mutex<VecDeque<ChangeEvent>>,
    tx: broadcast::Sender<ChangeEvent>,
}

impl fmt::Debug for ChangeBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("ChangeBus").finish_non_exhaustive() }
}

impl ChangeBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(HISTORY_LIMIT);
        Self { inner: Arc::new(Inner { next_seq: AtomicU64::new(1), history: Mutex::new(VecDeque::new()), tx }) }
    }

    pub fn publish(&self, event_type: impl Into<String>, source: impl Into<String>, payload: serde_json::Value) -> ChangeEvent {
        let event = ChangeEvent { seq: self.inner.next_seq.fetch_add(1, Ordering::SeqCst), event_type: event_type.into(), source: source.into(), payload, created_at: chrono::Utc::now() };
        if let Ok(mut history) = self.inner.history.lock() {
            history.push_back(event.clone());
            while history.len() > HISTORY_LIMIT { history.pop_front(); }
        }
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
            opencode_sources: opencode_event_sources(),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> { self.inner.tx.subscribe() }
}

impl Default for ChangeBus { fn default() -> Self { Self::new() } }

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
}
