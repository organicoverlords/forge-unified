//! Snapshot manager — conversation checkpoint and restore.

use crate::types::{Conversation, ConversationId};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct SnapshotManager {
    snapshot_dir: PathBuf,
}

impl SnapshotManager {
    pub fn new(data_dir: &str) -> Self {
        let snapshot_dir = PathBuf::from(data_dir).join("snapshots");
        Self { snapshot_dir }
    }

    pub fn save(&self, conversation: &Conversation) -> Result<()> {
        std::fs::create_dir_all(&self.snapshot_dir)?;
        let path = self.snapshot_path(&conversation.id);
        let json = serde_json::to_string_pretty(conversation)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub fn load(&self, id: &ConversationId) -> Result<Conversation> {
        let path = self.snapshot_path(id);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Snapshot not found for conversation {}", id.0))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn list(&self) -> Result<Vec<ConversationId>> {
        if !self.snapshot_dir.exists() {
            return Ok(Vec::new());
        }
        let mut ids = Vec::new();
        for entry in std::fs::read_dir(&self.snapshot_dir)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    if let Ok(uuid) = name.trim_end_matches(".json").parse::<uuid::Uuid>() {
                        ids.push(ConversationId(uuid));
                    }
                }
            }
        }
        Ok(ids)
    }

    pub fn delete(&self, id: &ConversationId) -> Result<()> {
        let path = self.snapshot_path(id);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    fn snapshot_path(&self, id: &ConversationId) -> PathBuf {
        self.snapshot_dir.join(format!("{}.json", id.0))
    }
}
