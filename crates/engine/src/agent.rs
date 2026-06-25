//! Agent — high-level interface wrapping orchestrator, router, and conversation.

use crate::config::Config;
use crate::orchestrator::Orchestrator;
use crate::conversation::ConversationManager;
use crate::snapshot::SnapshotManager;
use crate::types::*;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Agent {
    orchestrator: Arc<Orchestrator>,
    conversations: Arc<RwLock<ConversationManager>>,
    snapshots: Arc<SnapshotManager>,
    config: Config,
}

impl Agent {
    pub fn new(config: Config) -> Self {
        let snapshots = Arc::new(SnapshotManager::new(&config.data_dir));
        let orchestrator = Arc::new(Orchestrator::new(config.clone()));
        let conversations = Arc::new(RwLock::new(ConversationManager::new()));

        Self {
            orchestrator,
            conversations,
            snapshots,
            config,
        }
    }

    pub async fn chat(&self, id: &ConversationId, message: String) -> Result<RunRecord> {
        self.orchestrator.run(id.clone(), message, 10).await
    }

    pub async fn new_conversation(&self, title: String) -> ConversationId {
        self.conversations.write().await.create(title)
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
        self.conversations.write().await.delete(id)
    }

    pub async fn save_snapshot(&self, id: &ConversationId) -> Result<()> {
        if let Some(conv) = self.conversations.read().await.get(id).cloned() {
            self.snapshots.save(&conv)?;
        }
        Ok(())
    }

    pub async fn load_snapshot(&self, id: &ConversationId) -> Result<()> {
        let conv = self.snapshots.load(id)?;
        let title = conv.title.clone();
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub id: ConversationId,
    pub title: String,
    pub message_count: usize,
    pub mode: AgentMode,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

use serde::{Serialize, Deserialize};
