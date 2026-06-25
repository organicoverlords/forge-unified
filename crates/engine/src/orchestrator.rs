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
    snapshot: Arc<SnapshotManager>,
    config: Config,
}

impl Orchestrator {
    pub fn new(config: Config) -> Self {
        let router = Arc::new(Router::from_config(&config));
        let tool_executor = Arc::new(ToolExecutor::new(
            config.workspace_root.clone(),
            config.tool_timeout_ms,
            config.max_parallel_tools,
        ));
        let conversation_mgr = Arc::new(RwLock::new(ConversationManager::new()));
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
            let conv_mgr = self.conversation_mgr.read().await;
            let messages = conv_mgr.get_messages(&conversation_id);

            let request = ChatRequest {
                model: model.clone(),
                messages: messages.to_vec(),
                temperature: Some(0.7),
                max_tokens: Some(4096),
                stream: false,
                tools: None,
                tool_choice: None,
            };

            let response = match self.router.chat(request).await {
                Ok(r) => r,
                Err(e) => {
                    self.conversation_mgr.write().await.add_assistant_message(
                        &conversation_id,
                        format!("[Provider error: {}]", e),
                    );
                    break;
                }
            };

            let assistant_content = response.message.content.clone();
            self.conversation_mgr.write().await.add_assistant_message(
                &conversation_id,
                assistant_content.clone(),
            );

            if response.message.tool_calls.is_none() || response.message.tool_calls.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
                break;
            }

            let tool_requests = response.message.tool_calls.unwrap();
            let decision = self.strategy.decide_for_requests(&provider, &model, &tool_requests);

            // Filter allowed tools
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
                        Err(e) => {
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
}
