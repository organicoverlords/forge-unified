//! Provider abstraction for LLM API routing.
//! Supports NVIDIA NIM, OpenAI-compatible, and local providers.

use crate::types::{Message, ProviderConfig, ProviderId, ModelId, ModelConfig, ToolConfig, ToolKind};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub mod nvidia_nim;
pub mod openai;

pub use nvidia_nim::NvidiaNimProvider;
pub use openai::OpenAiProvider;

#[async_trait]
pub trait Provider: Send + Sync {
    fn id(&self) -> &ProviderId;
    fn config(&self) -> &ProviderConfig;
    
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream>;
    async fn health_check(&self) -> Result<bool>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: ModelId,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
    pub tools: Option<Vec<ToolConfig>>,
    pub tool_choice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: Message,
    pub usage: Option<TokenUsage>,
    pub provider: ProviderId,
    pub model: ModelId,
}

pub struct ChatStream {
    pub provider: ProviderId,
    pub model: ModelId,
    pub receiver: tokio::sync::mpsc::Receiver<StreamEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    Token(String),
    ToolCall(ToolCallDelta),
    Done(TokenUsage),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub fn create_provider(config: ProviderConfig) -> Box<dyn Provider> {
    match config.id.0.as_str() {
        "nvidia_nim" => Box::new(NvidiaNimProvider::new(config)),
        _ => Box::new(OpenAiProvider::new(config)),
    }
}

pub fn default_nim_config() -> ProviderConfig {
    ProviderConfig {
        id: ProviderId("nvidia_nim".to_string()),
        name: "NVIDIA NIM".to_string(),
        api_base: "https://integrate.api.nvidia.com/v1".to_string(),
        api_key_env: "NVIDIA_NIM_API_KEY".to_string(),
        models: vec![
            ModelConfig {
                id: ModelId("openai/gpt-oss-120b".to_string()),
                name: "GPT-OSS 120B".to_string(),
                context_window: 128000,
                supports_streaming: true,
                supports_tools: true,
                supports_parallel_tools: false,
                max_output_tokens: 8192,
            },
            ModelConfig {
                id: ModelId("meta/llama-3.1-405b-instruct".to_string()),
                name: "Llama 3.1 405B".to_string(),
                context_window: 128000,
                supports_streaming: true,
                supports_tools: true,
                supports_parallel_tools: false,
                max_output_tokens: 8192,
            },
            ModelConfig {
                id: ModelId("meta/llama-3.1-70b-instruct".to_string()),
                name: "Llama 3.1 70B".to_string(),
                context_window: 128000,
                supports_streaming: true,
                supports_tools: true,
                supports_parallel_tools: true,
                max_output_tokens: 8192,
            },
            ModelConfig {
                id: ModelId("mistralai/mistral-nemo-12b-instruct".to_string()),
                name: "Mistral NeMo 12B".to_string(),
                context_window: 128000,
                supports_streaming: true,
                supports_tools: true,
                supports_parallel_tools: true,
                max_output_tokens: 8192,
            },
        ],
        enabled: std::env::var("NVIDIA_NIM_API_KEY").is_ok(),
        priority: 0,
        max_retries: 3,
        timeout_ms: 120000,
    }
}