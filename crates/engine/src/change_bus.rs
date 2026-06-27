use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc, Mutex};
use tokio::sync::broadcast;

const HISTORY_LIMIT: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub seq: u64,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct ChangeBus {
    inner: Arc<Inner>,
}

struct Inner {
    next_seq: AtomicU64,
    history: Mutex<VecDeque<ChangeEvent>>,
    tx: broadcast::Sender<ChangeEvent>,
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

    pub fn recent(&self) -> Vec<ChangeEvent> { self.inner.history.lock().map(|h| h.iter().cloned().collect()).unwrap_or_default() }

    pub fn subscribe(&self) -> broadcast::Receiver<ChangeEvent> { self.inner.tx.subscribe() }
}

impl Default for ChangeBus {
    fn default() -> Self { Self::new() }
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
}
