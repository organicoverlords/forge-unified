//! Run orchestrator.

use crate::change_bus::{ChangeBusStatus, ChangeEvent};
use crate::config::Config;
use crate::conversation::ConversationManager;
use crate::provider::ChatRequest;
use crate::router::Router;
use crate::safety::SafetyChecker;
use crate::snapshot::SnapshotManager;
use crate::state::EngineState;
use crate::strategy::StrategyEngine;
use crate::tool::ToolExecutor;
use crate::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Orchestrator {
    state: Arc<RwLock<EngineState>>, router: Arc<Router>, tool_executor: Arc<ToolExecutor>,
    conversation_mgr: Arc<RwLock<ConversationManager>>, strategy: Arc<StrategyEngine>, safety: Arc<SafetyChecker>,
    #[allow(dead_code)] snapshot: Arc<SnapshotManager>, #[allow(dead_code)] config: Config,
}

impl Orchestrator {
    pub fn new(config: Config, conversation_mgr: Arc<RwLock<ConversationManager>>) -> Self {
        let router = Arc::new(Router::from_config(&config));
        let tool_executor = Arc::new(ToolExecutor::new(config.workspace_root.clone(), config.tool_timeout_ms, config.max_parallel_tools));
        let strategy = Arc::new(StrategyEngine::with_nim_catalog());
        let safety = Arc::new(SafetyChecker::new(config.approval_mode.clone()));
        let snapshot = Arc::new(SnapshotManager::new(&config.data_dir));
        let state = Arc::new(RwLock::new(EngineState::new(&config)));
        Self { state, router, tool_executor, conversation_mgr, strategy, safety, snapshot, config }
    }

    pub async fn run(&self, conversation_id: ConversationId, user_message: String, max_rounds: u32) -> Result<RunRecord> {
        let run_id = RunId(uuid::Uuid::new_v4());
        let provider = self.state.read().await.current_provider();
        let model = self.state.read().await.current_model();
        let mut round = 0u32;
        let mut total_tool_calls = 0u32;
        let mut total_tool_failures = 0u32;
        let mut stopped_on_tool_cap = false;
        {
            let mut mgr = self.conversation_mgr.write().await;
            if let Some(conv) = mgr.get_mut(&conversation_id) { conv.provider = Some(provider.clone()); conv.model = Some(model.clone()); conv.updated_at = chrono::Utc::now(); }
            mgr.add_user_message(&conversation_id, user_message.clone());
        }

        while round < max_rounds {
            round += 1;
            let messages = model_messages(&self.conversation_mgr, &conversation_id, build_system_prompt(&user_message)).await;
            let request = ChatRequest { model: model.clone(), messages, temperature: Some(0.2), max_tokens: Some(4096), stream: false, tools: Some(crate::tool::tool_definitions()), tool_choice: Some("auto".to_string()) };
            let response = match self.router.chat(request).await {
                Ok(r) => r,
                Err(e) => { tracing::warn!("LLM route failed: {}", e); self.conversation_mgr.write().await.add_assistant_message(&conversation_id, format!("[Provider route failed: {}]", e)); break; }
            };
            self.conversation_mgr.write().await.add_assistant_message_with_tools(&conversation_id, response.message.content.clone(), response.message.tool_calls.clone());
            if response.message.tool_calls.as_ref().map(|t| t.is_empty()).unwrap_or(true) { stopped_on_tool_cap = false; break; }
            stopped_on_tool_cap = round >= max_rounds;
            let tool_requests = response.message.tool_calls.unwrap();
            let decision = self.strategy.decide_for_requests(&provider, &model, &tool_requests);
            let mut allowed_requests = Vec::new();
            for req in tool_requests { if self.safety.check_tool(&req.kind).await { allowed_requests.push(req); } else { total_tool_failures += 1; } }
            let results = if decision.strategy == crate::model_caps::ToolStrategy::Serial {
                let mut results = Vec::new();
                for req in allowed_requests {
                    total_tool_calls += 1;
                    match self.tool_executor.execute(req.clone()).await {
                        Ok(r) => { if !r.success { total_tool_failures += 1; } results.push(r); }
                        Err(error) => { total_tool_failures += 1; results.push(tool_error_result(req, error.to_string())); }
                    }
                }
                results
            } else {
                let call_count = allowed_requests.len() as u32;
                let results = self.tool_executor.execute_batch(allowed_requests).await;
                total_tool_calls += call_count;
                total_tool_failures += results.iter().filter(|r| !r.success).count() as u32;
                results
            };
            self.conversation_mgr.write().await.add_tool_results(&conversation_id, results);
        }
        if stopped_on_tool_cap { self.force_final_answer(&conversation_id, &provider, &model, &user_message).await; }

        let started_at = chrono::Utc::now();
        Ok(RunRecord { id: run_id, conversation_id, task: user_message, status: RunStatus::Completed, provider, model, tool_calls: total_tool_calls, tool_failures: total_tool_failures, started_at, completed_at: Some(chrono::Utc::now()), metadata: HashMap::from([("rounds".to_string(), serde_json::json!(round)), ("forced_final_after_tool_cap".to_string(), serde_json::json!(stopped_on_tool_cap))]) })
    }

    async fn force_final_answer(&self, conversation_id: &ConversationId, provider: &ProviderId, model: &ModelId, user_message: &str) {
        let final_prompt = format!("{} No more tool calls are available. Write the final answer now. Include the requested Founder report, Technical report, files created/removed/modified, validation commands, remaining risks, and confidence score. Do not call tools.", build_system_prompt(user_message));
        let messages = model_messages(&self.conversation_mgr, conversation_id, final_prompt).await;
        let request = ChatRequest { model: model.clone(), messages, temperature: Some(0.2), max_tokens: Some(4096), stream: false, tools: None, tool_choice: Some("none".to_string()) };
        match self.router.chat(request).await {
            Ok(response) => self.conversation_mgr.write().await.add_assistant_message(conversation_id, response.message.content),
            Err(error) => self.conversation_mgr.write().await.add_assistant_message(conversation_id, format!("Founder report: tool rounds completed, but finalization failed. Technical report: provider={}; model={}; finalization error={}; remaining risk: final report was generated by fallback error handling, not the model.", provider.0, model.0, error)),
        }
    }

    pub async fn cancel(&self, conversation_id: &ConversationId) -> Result<()> { self.state.write().await.cancel_run(conversation_id); Ok(()) }
    pub async fn pause(&self, conversation_id: &ConversationId) -> Result<()> { self.state.write().await.pause_run(conversation_id); Ok(()) }
    pub async fn resume(&self, conversation_id: &ConversationId) -> Result<()> { self.state.write().await.resume_run(conversation_id); Ok(()) }
    pub async fn execute_tool(&self, request: ToolRequest) -> Result<ToolResult> { self.tool_executor.execute(request).await }
    pub fn publish_change_event(&self, event_type: &str, source: &str, payload: serde_json::Value) -> ChangeEvent { self.tool_executor.change_bus().publish(event_type, source, payload) }
    pub fn recent_change_events(&self) -> Vec<ChangeEvent> { self.tool_executor.recent_change_events() }
    pub fn change_bus_status(&self) -> ChangeBusStatus { self.tool_executor.change_bus_status() }
    pub fn subscribe_change_events(&self) -> tokio::sync::broadcast::Receiver<ChangeEvent> { self.tool_executor.subscribe_change_events() }
}

async fn model_messages(conversation_mgr: &Arc<RwLock<ConversationManager>>, conversation_id: &ConversationId, system: String) -> Vec<Message> {
    let conv_mgr = conversation_mgr.read().await;
    let mut messages = conv_mgr.get_messages(conversation_id).to_vec();
    messages.insert(0, Message { role: MessageRole::System, content: system, tool_calls: None, tool_results: None, metadata: Default::default() });
    messages
}

fn tool_error_result(req: ToolRequest, error: String) -> ToolResult {
    ToolResult { id: req.id, kind: req.kind, success: false, output: format!("Tool execution failed: {error}"), error: Some(error), duration_ms: 0, metadata: HashMap::from([("tool_execution_error".to_string(), serde_json::json!(true))]) }
}

fn build_system_prompt(user_message: &str) -> String {
    let lower = user_message.to_ascii_lowercase();
    let repo_work = ["repo", "repository", "inspect", "build", "fix", "patch", "webui", "test", "phase", "files", "git"].iter().any(|needle| lower.contains(needle));
    let base = "You are Forge, an OpenCode-style coding agent. Use available tools for repository work, keep file changes low-risk, and keep final answers brief.";
    if !repo_work { return base.to_string(); }
    format!("{base} For repository tasks, call tools before answering. Inspect the repo first with repo_info, file_list, file_search, file_read, and bounded shell_command as needed. Treat tool errors as evidence and choose another tool or path instead of repeating the same failing call. For multi-phase prompts, complete phases in order, but once you have enough evidence, stop searching and write the requested final reports. Verify file operations by reading or listing files, run validation when feasible, and summarize changes, tests, risks, and confidence. Do not pretend to have inspected files or run commands without tool results.")
}
