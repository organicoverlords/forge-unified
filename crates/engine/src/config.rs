//! Configuration management for the Forge engine.

use crate::types::{ModelConfig, ModelId, ProviderConfig, ProviderId};
use serde::{Deserialize, Serialize};

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
            providers: default_provider_configs(),
            default_provider: None,
            default_model: None,
            approval_mode: ApprovalMode::Ask,
            max_parallel_tools: 10,
            tool_timeout_ms: 60000,
            context_window_limit: 128000,
            auto_compact: true,
            workspace_root: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
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

fn default_provider_configs() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig {
            id: ProviderId("nvidia_nim".to_string()),
            name: "NVIDIA NIM".to_string(),
            api_base: "https://integrate.api.nvidia.com/v1".to_string(),
            api_key_env: "NVIDIA_NIM_API_KEY".to_string(),
            enabled: std::env::var("NVIDIA_NIM_API_KEY").is_ok(),
            priority: 0,
            max_retries: 3,
            timeout_ms: 60000,
            models: vec![
                model("mistralai/mistral-small-4-119b-2603", "Mistral Small 4", 128000, true),
                model("deepseek-ai/deepseek-v4-flash", "DeepSeek V4 Flash", 128000, true),
                model("openai/gpt-oss-120b", "GPT-OSS 120B", 128000, true),
                model("meta/llama-3.1-70b-instruct", "Llama 3.1 70B", 128000, true),
            ],
        },
        ProviderConfig {
            id: ProviderId("groq".to_string()),
            name: "Groq".to_string(),
            api_base: "https://api.groq.com/openai/v1".to_string(),
            api_key_env: "GROQ_API_KEY".to_string(),
            enabled: std::env::var("GROQ_API_KEY").is_ok(),
            priority: 10,
            max_retries: 2,
            timeout_ms: 45000,
            models: vec![
                model("llama-3.3-70b-versatile", "Llama 3.3 70B Versatile", 128000, true),
            ],
        },
        ProviderConfig {
            id: ProviderId("openrouter".to_string()),
            name: "OpenRouter".to_string(),
            api_base: "https://openrouter.ai/api/v1".to_string(),
            api_key_env: "OPENROUTER_API_KEY".to_string(),
            enabled: std::env::var("OPENROUTER_API_KEY").is_ok(),
            priority: 20,
            max_retries: 2,
            timeout_ms: 60000,
            models: vec![
                model("mistralai/mistral-small-3.2-24b-instruct:free", "Mistral Small Free", 128000, true),
            ],
        },
    ]
}

fn model(id: &str, name: &str, context_window: u32, supports_tools: bool) -> ModelConfig {
    ModelConfig {
        id: ModelId(id.to_string()),
        name: name.to_string(),
        context_window,
        supports_streaming: true,
        supports_tools,
        supports_parallel_tools: supports_tools,
        max_output_tokens: 8192,
    }
}
