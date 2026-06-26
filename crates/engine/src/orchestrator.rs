//! Run orchestrator — manages the agent loop: prompt → LLM → tools → observe → repeat.

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
    state: Arc<RwLock<EngineState>>,
    router: Arc<Router>,
    tool_executor: Arc<ToolExecutor>,
    conversation_mgr: Arc<RwLock<ConversationManager>>,
    strategy: Arc<StrategyEngine>,
    safety: Arc<SafetyChecker>,
    #[allow(dead_code)]
    snapshot: Arc<SnapshotManager>,
    #[allow(dead_code)]
    config: Config,
}

impl Orchestrator {
    pub fn new(config: Config, conversation_mgr: Arc<RwLock<ConversationManager>>) -> Self {
        let router = Arc::new(Router::from_config(&config));
        let tool_executor = Arc::new(ToolExecutor::new(
            config.workspace_root.clone(),
            config.tool_timeout_ms,
            config.max_parallel_tools,
        ));
        let strategy = Arc::new(StrategyEngine::with_nim_catalog());
        let safety = Arc::new(SafetyChecker::new(config.approval_mode.clone()));
        let snapshot = Arc::new(SnapshotManager::new(&config.data_dir));
        let state = Arc::new(RwLock::new(EngineState::new(&config)));

        Self {
            state,
            router,
            tool_executor,
            conversation_mgr,
            strategy,
            safety,
            snapshot,
            config,
        }
    }

    pub async fn run(
        &self,
        conversation_id: ConversationId,
        user_message: String,
        max_rounds: u32,
    ) -> Result<RunRecord> {
        let run_id = RunId(uuid::Uuid::new_v4());
        let provider = self.state.read().await.current_provider();
        let model = self.state.read().await.current_model();

        let mut round = 0u32;
        let mut total_tool_calls = 0u32;
        let mut total_tool_failures = 0u32;

        self.conversation_mgr.write().await.add_user_message(
            &conversation_id,
            user_message.clone(),
        );

        while round < max_rounds {
            round += 1;

            let messages = {
                let conv_mgr = self.conversation_mgr.read().await;
                let mut messages = conv_mgr.get_messages(&conversation_id).to_vec();
                messages.insert(0, Message {
                    role: MessageRole::System,
                    content: build_system_prompt(&user_message),
                    tool_calls: None,
                    tool_results: None,
                    metadata: Default::default(),
                });
                messages
            };

            let tools = crate::tool::tool_definitions();
            let request = ChatRequest {
                model: model.clone(),
                messages,
                temperature: Some(0.2),
                max_tokens: Some(4096),
                stream: false,
                tools: Some(tools),
                tool_choice: Some("auto".to_string()),
            };

            let response = match self.router.chat(request).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!("LLM provider error: {}", e);
                    self.conversation_mgr.write().await.add_assistant_message(
                        &conversation_id,
                        format!("[Provider error: {}]", e),
                    );
                    break;
                }
            };

            let assistant_content = response.message.content.clone();
            self.conversation_mgr.write().await.add_assistant_message_with_tools(
                &conversation_id,
                assistant_content.clone(),
                response.message.tool_calls.clone(),
            );

            if response.message.tool_calls.is_none() || response.message.tool_calls.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
                break;
            }

            let tool_requests = response.message.tool_calls.unwrap();
            let decision = self.strategy.decide_for_requests(&provider, &model, &tool_requests);

            let mut allowed_requests = Vec::new();
            for req in tool_requests {
                if self.safety.check_tool(&req.kind).await {
                    allowed_requests.push(req);
                } else {
                    total_tool_failures += 1;
                }
            }

            let results = if decision.strategy == crate::model_caps::ToolStrategy::Serial {
                let mut results = Vec::new();
                for req in allowed_requests {
                    match self.tool_executor.execute(req).await {
                        Ok(r) => {
                            total_tool_calls += 1;
                            if !r.success { total_tool_failures += 1; }
                            results.push(r);
                        }
                        Err(_e) => {
                            total_tool_failures += 1;
                        }
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

        let started_at = chrono::Utc::now();
        Ok(RunRecord {
            id: run_id,
            conversation_id,
            task: user_message,
            status: RunStatus::Completed,
            provider,
            model,
            tool_calls: total_tool_calls,
            tool_failures: total_tool_failures,
            started_at,
            completed_at: Some(chrono::Utc::now()),
            metadata: HashMap::from([
                ("rounds".to_string(), serde_json::json!(round)),
            ]),
        })
    }

    pub async fn cancel(&self, conversation_id: &ConversationId) -> Result<()> {
        self.state.write().await.cancel_run(conversation_id);
        Ok(())
    }

    pub async fn pause(&self, conversation_id: &ConversationId) -> Result<()> {
        self.state.write().await.pause_run(conversation_id);
        Ok(())
    }

    pub async fn resume(&self, conversation_id: &ConversationId) -> Result<()> {
        self.state.write().await.resume_run(conversation_id);
        Ok(())
    }

    pub async fn execute_tool(&self, request: crate::types::ToolRequest) -> Result<crate::types::ToolResult> {
        self.tool_executor.execute(request).await
    }
}

fn build_system_prompt(user_message: &str) -> String {
    let lower = user_message.to_ascii_lowercase();
    let self_build = lower.contains("build")
        || lower.contains("fix")
        || lower.contains("inspect")
        || lower.contains("repo_info")
        || lower.contains("file_list")
        || lower.contains("webui");

    let base = "You are Forge, an OpenCode-style coding agent. Use the provided tools to inspect and change the real workspace. Do not claim to inspect files without tool calls. When tool calls are needed, call tools first, then answer after observing tool results.";

    if self_build {
        format!("{} For this request, you must call repo_info and file_list with path '.' before the final answer. Keep the final answer short and state the smallest next build step.", base)
    } else {
        base.to_string()
    }
}
