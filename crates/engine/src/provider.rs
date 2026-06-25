//! Provider abstraction for LLM API routing.
//! Supports NVIDIA NIM, OpenAI-compatible, and local providers.

use crate::types::{Message, ProviderConfig, ProviderId, ModelId, ModelConfig, ToolConfig, ToolKind, ToolRequest, ToolCallId};
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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

/// Convert a ToolKind to its snake_case name.
pub fn tool_kind_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read",
        ToolKind::FileWrite => "file_write",
        ToolKind::FileEdit => "file_edit",
        ToolKind::FileDelete => "file_delete",
        ToolKind::FileList => "file_list",
        ToolKind::FileGlob => "file_glob",
        ToolKind::FileSearch => "file_search",
        ToolKind::WebFetch => "web_fetch",
        ToolKind::WebSearch => "web_search",
        ToolKind::ShellCommand => "shell_command",
        ToolKind::TerminalRun => "terminal_run",
        ToolKind::Task => "task",
        ToolKind::BatchParallel => "batch_parallel",
        ToolKind::RepoInfo => "repo_info",
        ToolKind::ProposePatch => "propose_patch",
        ToolKind::SwitchMode => "switch_mode",
        ToolKind::BrowserProof => "browser_proof",
        ToolKind::VisionReview => "vision_review",
        ToolKind::GraphBuild => "graph_build",
        ToolKind::GraphQuery => "graph_query",
    }
}

/// Convert a raw tool call name to a ToolKind.
pub fn tool_kind_from_name(name: &str) -> Option<ToolKind> {
    match name {
        "file_read" => Some(ToolKind::FileRead),
        "file_write" => Some(ToolKind::FileWrite),
        "file_edit" => Some(ToolKind::FileEdit),
        "file_delete" => Some(ToolKind::FileDelete),
        "file_list" => Some(ToolKind::FileList),
        "file_glob" => Some(ToolKind::FileGlob),
        "file_search" => Some(ToolKind::FileSearch),
        "web_fetch" => Some(ToolKind::WebFetch),
        "web_search" => Some(ToolKind::WebSearch),
        "shell_command" => Some(ToolKind::ShellCommand),
        "terminal_run" => Some(ToolKind::TerminalRun),
        "task" => Some(ToolKind::Task),
        "repo_info" => Some(ToolKind::RepoInfo),
        "propose_patch" => Some(ToolKind::ProposePatch),
        "switch_mode" => Some(ToolKind::SwitchMode),
        "browser_proof" => Some(ToolKind::BrowserProof),
        "vision_review" => Some(ToolKind::VisionReview),
        "graph_build" => Some(ToolKind::GraphBuild),
        "graph_query" => Some(ToolKind::GraphQuery),
        "batch_parallel" | "parallel_tool_calls" => Some(ToolKind::BatchParallel),
        _ => None,
    }
}

/// Convert provider tool call deltas into ToolRequest vec.
pub fn tool_calls_from_deltas(deltas: Vec<ToolCallDelta>) -> Vec<ToolRequest> {
    deltas.into_iter().filter_map(|d| {
        let kind = tool_kind_from_name(&d.name)?;
        let args: serde_json::Value = serde_json::from_str(&d.arguments).unwrap_or(serde_json::Value::Null);
        Some(ToolRequest {
            id: ToolCallId(Uuid::parse_str(&d.id).unwrap_or_else(|_| Uuid::new_v4())),
            kind,
            args,
            parallel_group: None,
        })
    }).collect()
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
        timeout_ms: 60000,
    }
}