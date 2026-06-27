//! Agent — high-level interface wrapping orchestrator, router, and conversation.

use crate::change_bus::{ChangeBusStatus, ChangeEvent};
use crate::config::Config;
use crate::conversation::ConversationManager;
use crate::orchestrator::Orchestrator;
use crate::snapshot::SnapshotManager;
use crate::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

const OPENCODE_COMPACTION_RUNTIME_SOURCE: &str = "packages/core/src/session/compaction.ts";
const OPENCODE_COMPACTION_SCHEMA_SOURCE: &str = "packages/schema/src/session-event.ts";
const OPENCODE_COMPACTION_STARTED: &str = "session.next.compaction.started";
const OPENCODE_COMPACTION_ENDED: &str = "session.next.compaction.ended";

pub struct Agent {
    orchestrator: Arc<Orchestrator>,
    conversations: Arc<RwLock<ConversationManager>>,
    snapshots: Arc<SnapshotManager>,
    #[allow(dead_code)]
    config: Config,
}

impl Agent {
    pub fn new(config: Config) -> Self {
        let snapshots = Arc::new(SnapshotManager::new(&config.data_dir));
        let mut manager = ConversationManager::new();
        if let Ok(ids) = snapshots.list() {
            for id in ids { if let Ok(conv) = snapshots.load(&id) { manager.insert(id, conv); } }
        }
        let conversations = Arc::new(RwLock::new(manager));
        let orchestrator = Arc::new(Orchestrator::new(config.clone(), conversations.clone()));
        Self { orchestrator, conversations, snapshots, config }
    }

    pub async fn chat(&self, id: &ConversationId, message: String) -> Result<RunRecord> {
        let record = self.orchestrator.run(id.clone(), message, 10).await?;
        self.save_snapshot(id).await?;
        Ok(record)
    }

    pub async fn execute_tool(&self, request: ToolRequest) -> Result<ToolResult> { self.orchestrator.execute_tool(request).await }
    pub fn recent_change_events(&self) -> Vec<ChangeEvent> { self.orchestrator.recent_change_events() }
    pub fn change_bus_status(&self) -> ChangeBusStatus { self.orchestrator.change_bus_status() }
    pub fn subscribe_change_events(&self) -> tokio::sync::broadcast::Receiver<ChangeEvent> { self.orchestrator.subscribe_change_events() }

    pub async fn record_user_message(&self, id: &ConversationId, content: String) -> Result<()> {
        self.conversations.write().await.add_user_message(id, content);
        self.save_snapshot(id).await
    }

    pub async fn record_assistant_tool_call(&self, id: &ConversationId, content: String, call: ToolRequest) -> Result<()> {
        self.conversations.write().await.add_assistant_message_with_tools(id, content, Some(vec![call]));
        self.save_snapshot(id).await
    }

    pub async fn record_tool_results(&self, id: &ConversationId, results: Vec<ToolResult>) -> Result<()> {
        self.conversations.write().await.add_tool_results(id, results);
        self.save_snapshot(id).await
    }

    pub async fn record_assistant_summary(&self, id: &ConversationId, summary: String) -> Result<()> {
        self.conversations.write().await.add_assistant_message(id, summary);
        self.save_snapshot(id).await
    }

    pub async fn new_conversation(&self, title: String) -> ConversationId {
        let id = self.conversations.write().await.create(title);
        let _ = self.save_snapshot(&id).await;
        id
    }

    pub async fn list_conversations(&self) -> Vec<ConversationSummary> {
        self.conversations.read().await.list().iter().map(|c| ConversationSummary { id: c.id.clone(), title: c.title.clone(), message_count: c.messages.len(), mode: c.mode.clone(), updated_at: c.updated_at }).collect()
    }

    pub async fn get_conversation(&self, id: &ConversationId) -> Option<Conversation> { self.conversations.read().await.get(id).cloned() }

    pub async fn delete_conversation(&self, id: &ConversationId) -> Option<Conversation> {
        let removed = self.conversations.write().await.delete(id);
        if removed.is_some() { let _ = self.snapshots.delete(id); }
        removed
    }

    pub async fn save_snapshot(&self, id: &ConversationId) -> Result<()> {
        if let Some(conv) = self.conversations.read().await.get(id).cloned() { self.snapshots.save(&conv)?; }
        Ok(())
    }

    pub async fn save_snapshot_with_part(&self, id: &ConversationId) -> Result<String> {
        let label = format!("Snapshot saved at {}", chrono::Utc::now().to_rfc3339());
        self.conversations.write().await.add_snapshot_part(id, label.clone());
        self.save_snapshot(id).await?;
        Ok(label)
    }

    pub async fn compact_with_part(&self, id: &ConversationId, keep_last: usize, auto: bool, overflow: bool) -> Result<Value> {
        let started = self.orchestrator.publish_change_event(OPENCODE_COMPACTION_STARTED, "opencode.session.compaction", serde_json::json!({
            "sessionID": id.0.to_string(),
            "conversation_id": id.0.to_string(),
            "messageID": format!("compaction-{}", id.0),
            "reason": if auto { "auto" } else { "manual" },
            "keep_last": keep_last,
            "auto": auto,
            "overflow": overflow,
            "durable": true,
            "opencode_source": OPENCODE_COMPACTION_SCHEMA_SOURCE,
            "opencode_runtime_source": OPENCODE_COMPACTION_RUNTIME_SOURCE,
            "copied_behavior": "publish SessionEvent.Compaction.Started before summarizing old context"
        }));
        let mut result = self.conversations.write().await.add_compaction_part(id, keep_last, auto, overflow).ok_or_else(|| anyhow::anyhow!("Conversation not found"))?;
        self.save_snapshot(id).await?;
        let finished = self.orchestrator.publish_change_event(OPENCODE_COMPACTION_ENDED, "opencode.session.compaction", serde_json::json!({
            "sessionID": id.0.to_string(),
            "conversation_id": id.0.to_string(),
            "messageID": format!("compaction-{}", id.0),
            "reason": if auto { "auto" } else { "manual" },
            "text": result.get("summary").cloned().unwrap_or(Value::String(String::new())),
            "recent": result.get("recent").cloned().unwrap_or(Value::String(String::new())),
            "compacted": result.get("compacted").and_then(Value::as_bool).unwrap_or(false),
            "before": result.get("before").and_then(Value::as_u64),
            "after": result.get("after").and_then(Value::as_u64),
            "tail_start_id": result.get("tail_start_id").cloned().unwrap_or(Value::Null),
            "started_seq": started.seq,
            "durable": true,
            "opencode_source": OPENCODE_COMPACTION_SCHEMA_SOURCE,
            "opencode_runtime_source": OPENCODE_COMPACTION_RUNTIME_SOURCE,
            "copied_behavior": "publish SessionEvent.Compaction.Ended with summary text and recent tail after compaction"
        }));
        if let Some(object) = result.as_object_mut() {
            object.insert("event_bus_receipts".to_string(), serde_json::json!([started, finished]));
            object.insert("event_bus_status".to_string(), serde_json::json!(self.change_bus_status()));
            object.insert("opencode_compaction_event_source".to_string(), serde_json::json!({"schema_path":OPENCODE_COMPACTION_SCHEMA_SOURCE,"runtime_path":OPENCODE_COMPACTION_RUNTIME_SOURCE,"events":[OPENCODE_COMPACTION_STARTED,OPENCODE_COMPACTION_ENDED]}));
        }
        Ok(result)
    }

    pub async fn load_snapshot(&self, id: &ConversationId) -> Result<()> {
        let conv = self.snapshots.load(id)?;
        self.conversations.write().await.delete(id);
        self.conversations.write().await.insert(id.clone(), conv);
        Ok(())
    }

    pub async fn cancel(&self, id: &ConversationId) -> Result<()> { self.orchestrator.cancel(id).await }
    pub async fn pause(&self, id: &ConversationId) -> Result<()> { self.orchestrator.pause(id).await }
    pub async fn resume(&self, id: &ConversationId) -> Result<()> { self.orchestrator.resume(id).await }

    pub async fn browser_proof(&self, url: &str, width: u32, height: u32, capture_dom: bool) -> Result<BrowserProofResult> {
        let req = ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::BrowserProof, args: serde_json::json!({"url": url, "width": width, "height": height, "capture_dom": capture_dom}), parallel_group: None };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success { anyhow::bail!("Browser proof failed: {}", result.error.unwrap_or_default()); }
        serde_json::from_str(&result.output).map_err(|e| anyhow::anyhow!("Failed to parse browser proof result: {}", e))
    }

    pub async fn vision_review(&self, image_base64: &str, prompt: Option<&str>, provider_id: Option<ProviderId>, model_id: Option<ModelId>) -> Result<VisionReviewResult> {
        let req = ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::VisionReview, args: serde_json::json!({"image_base64": image_base64, "prompt": prompt, "provider_id": provider_id, "model_id": model_id}), parallel_group: None };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success { anyhow::bail!("Vision review failed: {}", result.error.unwrap_or_default()); }
        serde_json::from_str(&result.output).map_err(|e| anyhow::anyhow!("Failed to parse vision review result: {}", e))
    }

    pub async fn graph_build(&self, pattern: Option<String>) -> Result<Value> {
        let req = ToolRequest { id: ToolCallId(Uuid::new_v4()), kind: ToolKind::GraphBuild, args: serde_json::json!({"pattern": pattern.unwrap_or_else(|| "**/crates/**/*.rs".to_string())}), parallel_group: None };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success { anyhow::bail!("Graph build failed: {}", result.error.unwrap_or_default()); }
        serde_json::from_str(&result.output).map_err(|e| anyhow::anyhow!("Failed to parse graph result: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub id: ConversationId,
    pub title: String,
    pub message_count: usize,
    pub mode: AgentMode,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
