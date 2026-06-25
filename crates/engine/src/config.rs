//! Configuration management for the Forge engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::{ProviderConfig, ModelConfig, ProviderId, ModelId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
    pub default_provider: Option<ProviderId>,
    pub default_model: Option<ModelId>,
    pub approval_mode: ApprovalMode,
    pub max_parallel_tools: usize,
    pub tool_timeout_ms: u64,
    pub context_window_limit: usize,
    pub auto_compact: bool,
    pub workspace_root: String,
    pub data_dir: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ApprovalMode {
    #[default]
    Ask,
    Full,
    ReadOnly,
    Blocked,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            providers: vec![],
            default_provider: None,
            default_model: None,
            approval_mode: ApprovalMode::Ask,
            max_parallel_tools: 10,
            tool_timeout_ms: 60000,
            context_window_limit: 128000,
            auto_compact: true,
            workspace_root: std::env::current_dir().unwrap_or_default().to_string_lossy().to_string(),
            data_dir: dirs::data_local_dir()
                .unwrap_or_default()
                .join("forge")
                .to_string_lossy()
                .to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("forge")
            .join("config.json");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("forge")
            .join("config.json");
        
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        std::fs::write(config_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn get_provider(&self, id: &ProviderId) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| &p.id == id)
    }

    pub fn get_model(&self, provider_id: &ProviderId, model_id: &ModelId) -> Option<&ModelConfig> {
        self.get_provider(provider_id)?
            .models
            .iter()
            .find(|m| &m.id == model_id)
    }

    pub fn enabled_providers(&self) -> Vec<&ProviderConfig> {
        self.providers.iter().filter(|p| p.enabled).collect()
    }

    pub fn provider_priority_order(&self) -> Vec<&ProviderConfig> {
        let mut providers = self.enabled_providers();
        providers.sort_by_key(|p| p.priority);
        providers
    }
}