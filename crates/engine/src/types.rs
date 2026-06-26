//! Core type definitions shared across the engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModelId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolCallId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolKind {
    FileRead,
    FileWrite,
    FileEdit,
    FileDelete,
    FileList,
    FileGlob,
    FileSearch,
    WebFetch,
    WebSearch,
    ShellCommand,
    TerminalRun,
    Task,
    BatchParallel,
    RepoInfo,
    ProposePatch,
    ApplyPatch,
    SwitchMode,
    BrowserProof,
    VisionReview,
    GraphBuild,
    GraphQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub id: ToolCallId,
    pub kind: ToolKind,
    pub args: serde_json::Value,
    pub parallel_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub id: ToolCallId,
    pub kind: ToolKind,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub tool_calls: Option<Vec<ToolRequest>>,
    pub tool_results: Option<Vec<ToolResult>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: ConversationId,
    pub title: String,
    pub messages: Vec<Message>,
    pub provider: Option<ProviderId>,
    pub model: Option<ModelId>,
    pub mode: AgentMode,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AgentMode {
    #[default]
    Chat,
    Explore,
    Plan,
    Build,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: ProviderId,
    pub name: String,
    pub api_base: String,
    pub api_key_env: String,
    pub models: Vec<ModelConfig>,
    pub enabled: bool,
    pub priority: u32,
    pub max_retries: u32,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: ModelId,
    pub name: String,
    pub context_window: u32,
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_parallel_tools: bool,
    pub max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub id: RunId,
    pub conversation_id: ConversationId,
    pub task: String,
    pub status: RunStatus,
    pub provider: ProviderId,
    pub model: ModelId,
    pub tool_calls: u32,
    pub tool_failures: u32,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
    Paused,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProofRequest {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub capture_dom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProofResult {
    pub screenshot_base64: String,
    pub console_logs: Vec<String>,
    pub dom_snapshot: Option<String>,
    pub url: String,
    pub page_title: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionReviewRequest {
    pub image_base64: String,
    pub prompt: Option<String>,
    pub provider_id: Option<ProviderId>,
    pub model_id: Option<ModelId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionReviewResult {
    pub analysis: String,
    pub verdict: Option<String>,
    pub provider: ProviderId,
    pub model: ModelId,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkCapabilities {
    pub chat: bool,
    pub natural_tasks: bool,
    pub repo_read: bool,
    pub repo_write: bool,
    pub shell: bool,
    pub tool_calls: bool,
    pub native_tool_traces: bool,
    pub parallel_tool_calls: bool,
    pub browser_self_report: bool,
    pub browser_artifact_proof: bool,
    pub screenshot: bool,
    pub dom_snapshot: bool,
    pub console_logs: bool,
    pub network_logs: bool,
    pub cancel: bool,
    pub pause: bool,
    pub resume: bool,
    pub context_compaction: bool,
    pub session_export: bool,
    pub repo_root_inventory: bool,
}
