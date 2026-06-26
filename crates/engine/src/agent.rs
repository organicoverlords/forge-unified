//! Agent — high-level interface wrapping orchestrator, router, and conversation.

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
            for id in ids {
                if let Ok(conv) = snapshots.load(&id) {
                    manager.insert(id, conv);
                }
            }
        }

        let conversations = Arc::new(RwLock::new(manager));
        let orchestrator = Arc::new(Orchestrator::new(config.clone(), conversations.clone()));

        Self {
            orchestrator,
            conversations,
            snapshots,
            config,
        }
    }

    pub async fn chat(&self, id: &ConversationId, message: String) -> Result<RunRecord> {
        let record = self.orchestrator.run(id.clone(), message, 10).await?;
        self.save_snapshot(id).await?;
        Ok(record)
    }

    pub async fn execute_tool(&self, request: ToolRequest) -> Result<ToolResult> {
        self.orchestrator.execute_tool(request).await
    }

    pub async fn new_conversation(&self, title: String) -> ConversationId {
        let id = self.conversations.write().await.create(title);
        let _ = self.save_snapshot(&id).await;
        id
    }

    pub async fn list_conversations(&self) -> Vec<ConversationSummary> {
        self.conversations.read().await.list().iter().map(|c| ConversationSummary {
            id: c.id.clone(),
            title: c.title.clone(),
            message_count: c.messages.len(),
            mode: c.mode.clone(),
            updated_at: c.updated_at,
        }).collect()
    }

    pub async fn get_conversation(&self, id: &ConversationId) -> Option<Conversation> {
        self.conversations.read().await.get(id).cloned()
    }

    pub async fn delete_conversation(&self, id: &ConversationId) -> Option<Conversation> {
        let removed = self.conversations.write().await.delete(id);
        if removed.is_some() {
            let _ = self.snapshots.delete(id);
        }
        removed
    }

    pub async fn save_snapshot(&self, id: &ConversationId) -> Result<()> {
        if let Some(conv) = self.conversations.read().await.get(id).cloned() {
            self.snapshots.save(&conv)?;
        }
        Ok(())
    }

    pub async fn load_snapshot(&self, id: &ConversationId) -> Result<()> {
        let conv = self.snapshots.load(id)?;
        let _title = conv.title.clone();
        self.conversations.write().await.delete(id);
        self.conversations.write().await.insert(id.clone(), conv);
        Ok(())
    }

    pub async fn cancel(&self, id: &ConversationId) -> Result<()> {
        self.orchestrator.cancel(id).await
    }

    pub async fn pause(&self, id: &ConversationId) -> Result<()> {
        self.orchestrator.pause(id).await
    }

    pub async fn resume(&self, id: &ConversationId) -> Result<()> {
        self.orchestrator.resume(id).await
    }

    pub async fn browser_proof(&self, url: &str, width: u32, height: u32, capture_dom: bool) -> Result<BrowserProofResult> {
        let req = ToolRequest {
            id: ToolCallId(uuid::Uuid::new_v4()),
            kind: ToolKind::BrowserProof,
            args: serde_json::json!({
                "url": url,
                "width": width,
                "height": height,
                "capture_dom": capture_dom,
            }),
            parallel_group: None,
        };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success {
            anyhow::bail!("Browser proof failed: {}", result.error.unwrap_or_default());
        }
        serde_json::from_str(&result.output).map_err(|e| anyhow::anyhow!("Failed to parse browser proof result: {}", e))
    }

    pub async fn vision_review(&self, image_base64: &str, prompt: Option<&str>, provider_id: Option<ProviderId>, model_id: Option<ModelId>) -> Result<VisionReviewResult> {
        let req = ToolRequest {
            id: ToolCallId(uuid::Uuid::new_v4()),
            kind: ToolKind::VisionReview,
            args: serde_json::json!({
                "image_base64": image_base64,
                "prompt": prompt,
                "provider_id": provider_id,
                "model_id": model_id,
            }),
            parallel_group: None,
        };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success {
            anyhow::bail!("Vision review failed: {}", result.error.unwrap_or_default());
        }
        serde_json::from_str(&result.output).map_err(|e| anyhow::anyhow!("Failed to parse vision review result: {}", e))
    }

    pub async fn graph_build(&self, pattern: Option<String>) -> Result<Value> {
        let req = ToolRequest {
            id: ToolCallId(Uuid::new_v4()),
            kind: ToolKind::GraphBuild,
            args: serde_json::json!({
                "pattern": pattern.unwrap_or_else(|| "**/crates/**/*.rs".to_string()),
            }),
            parallel_group: None,
        };
        let result = self.orchestrator.execute_tool(req).await?;
        if !result.success {
            anyhow::bail!("Graph build failed: {}", result.error.unwrap_or_default());
        }
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
