use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{atomic::{AtomicU64, Ordering}, mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use tokio::sync::broadcast;

const HISTORY_LIMIT: usize = 200;
const OPENCODE_APPLY_SOURCE: &str = "opencode.apply_patch";
const OPENCODE_NATIVE_WATCHER_SOURCE: &str = "opencode.native_filewatcher";
const DURABLE_LOG_PATH: &str = ".forge/change-events.jsonl";
const WATCHER_SUBSCRIBE_TIMEOUT_MS: usize = 10_000;
const OPENCODE_WATCHER_SOURCE: &str = "packages/core/src/filesystem/watcher.ts";

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
    pub recent_count: usize,
    pub last_seq: u64,
    pub durable_log_path: String,
    pub durable_log_writable: bool,
    pub watcher_backend: String,
    pub watcher_native_binding: String,
    pub watcher_running: bool,
    pub watcher_root: String,
    pub watcher_subscribe_timeout_ms: usize,
    pub watcher_ignore_patterns: Vec<String>,
    pub latest_files: Vec<String>,
    pub by_type: BTreeMap<String, usize>,
    pub by_source: BTreeMap<String, usize>,
    pub opencode_sources: Vec<String>,
}

#[derive(Debug, Clone)]
struct WatcherHandle {
    _thread: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[derive(Debug)]
struct ChangeBusInner {
    seq: AtomicU64,
    history: Mutex<VecDeque<ChangeEvent>>,
    tx: broadcast::Sender<ChangeEvent>,
    durable_log_path: PathBuf,
    watcher: Mutex<Option<WatcherHandle>>,
    watcher_root: PathBuf,
    watcher_ignore_patterns: Vec<String>,
}

#[derive(Clone)]
pub struct ChangeBus { inner: Arc<ChangeBusInner> }

impl fmt::Debug for ChangeBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChangeBus").field("last_seq", &self.inner.seq.load(Ordering::SeqCst)).finish()
    }
}

impl ChangeBus {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        let workspace_root = workspace_root.into();
        let durable_log_path = workspace_root.join(DURABLE_LOG_PATH);
        let (tx, _) = broadcast::channel(HISTORY_LIMIT);
        let bus = Self {
            inner: Arc::new(ChangeBusInner {
                seq: AtomicU64::new(0),
                history: Mutex::new(VecDeque::with_capacity(HISTORY_LIMIT)),
                tx,
                durable_log_path,
                watcher: Mutex::new(None),
                watcher_root: workspace_root.clone(),
                watcher_ignore_patterns: vec![".git".into(), "target".into(), ".forge".into(), "forge-proof".into()],
            }),
        };
        bus.start_native_watcher();
        bus
    }

    pub fn publish(&self, event_type: &str, source: &str, payload: serde_json::Value) -> ChangeEvent {
        let event = ChangeEvent { seq: self.inner.seq.fetch_add(1, Ordering::SeqCst) + 1, event_type: event_type.to_string(), source: source.to_string(), payload, created_at: chrono::Utc::now() };
        if let Ok(mut history) = self.inner.history.lock() {
            history.push_back(event.clone());
            while history.len() > HISTORY_LIMIT { history.pop_front(); }
        }
        let _ = self.inner.tx.send(event.clone());
        self.append_durable(&event);
        event
    }

    pub fn recent(&self) -> Vec<ChangeEvent> {
        self.inner.history.lock().map(|history| history.iter().cloned().collect()).unwrap_or_default()
    }

    pub fn status(&self) -> ChangeBusStatus {
        let recent = self.recent();
        let mut latest_files = Vec::new();
        let mut by_type = BTreeMap::new();
        let mut by_source = BTreeMap::new();
        for event in &recent {
            *by_type.entry(event.event_type.clone()).or_insert(0) += 1;
            *by_source.entry(event.source.clone()).or_insert(0) += 1;
            if let Some(file) = event.payload.get("file").or_else(|| event.payload.get("path")).and_then(|value| value.as_str()) {
                if !latest_files.iter().any(|known| known == file) { latest_files.push(file.to_string()); }
            }
        }
        let watcher_running = self.inner.watcher.lock().map(|guard| guard.is_some()).unwrap_or(false);
        ChangeBusStatus {
            event_bus: "change_bus".into(),
            bridge_shape: "opencode_event_v2_bridge_status".into(),
            history_limit: HISTORY_LIMIT,
            recent_count: recent.len(),
            last_seq: self.inner.seq.load(Ordering::SeqCst),
            durable_log_path: DURABLE_LOG_PATH.into(),
            durable_log_writable: self.inner.durable_log_path.parent().map(|path| path.exists()).unwrap_or(false),
            watcher_backend: "notify".into(),
            watcher_native_binding: OPENCODE_NATIVE_WATCHER_SOURCE.into(),
            watcher_running,
            watcher_root: self.inner.watcher_root.display().to_string(),
            watcher_subscribe_timeout_ms: WATCHER_SUBSCRIBE_TIMEOUT_MS,
            watcher_ignore_patterns: self.inner.watcher_ignore_patterns.clone(),
            latest_files,
            by_type,
            by_source,
            opencode_sources: vec![OPENCODE_WATCHER_SOURCE.into(), OPENCODE_APPLY_SOURCE.into()],
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> { self.inner.tx.subscribe() }

    fn append_durable(&self, event: &ChangeEvent) {
        if let Some(parent) = self.inner.durable_log_path.parent() { let _ = std::fs::create_dir_all(parent); }
        if let Ok(line) = serde_json::to_string(event) {
            use std::io::Write;
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&self.inner.durable_log_path) { let _ = writeln!(file, "{line}"); }
        }
    }

    fn should_ignore(&self, path: &Path) -> bool {
        let text = path.to_string_lossy();
        self.inner.watcher_ignore_patterns.iter().any(|pattern| text.contains(pattern))
    }

    fn start_native_watcher(&self) {
        let bus = self.clone();
        let root = self.inner.watcher_root.clone();
        let (ready_tx, ready_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            let watcher_result = RecommendedWatcher::new(tx, notify::Config::default());
            let mut watcher = match watcher_result {
                Ok(watcher) => watcher,
                Err(error) => { let _ = ready_tx.send(false); bus.publish("watcher.error", OPENCODE_NATIVE_WATCHER_SOURCE, json!({"error": error.to_string(), "opencode_source": OPENCODE_WATCHER_SOURCE})); return; }
            };
            if watcher.watch(&root, RecursiveMode::Recursive).is_err() { let _ = ready_tx.send(false); return; }
            let _ = ready_tx.send(true);
            while let Ok(event) = rx.recv() {
                if let Ok(event) = event {
                    if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)) { continue; }
                    for path in event.paths {
                        if bus.should_ignore(&path) { continue; }
                        bus.publish("watcher.updated", OPENCODE_NATIVE_WATCHER_SOURCE, json!({"file": path.display().to_string(), "event": format!("{:?}", event.kind), "opencode_source": OPENCODE_WATCHER_SOURCE}));
                    }
                }
            }
        });
        if ready_rx.recv_timeout(std::time::Duration::from_millis(WATCHER_SUBSCRIBE_TIMEOUT_MS as u64)).unwrap_or(false) {
            if let Ok(mut slot) = self.inner.watcher.lock() { *slot = Some(WatcherHandle { _thread: Arc::new(Mutex::new(Some(handle))) }); }
        }
    }
}
