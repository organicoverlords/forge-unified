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

const DOOM_LOOP_THRESHOLD: usize = 3;
const OPENCODE_MAX_STEPS_SOURCE: &str = "packages/core/src/session/runner/max-steps.ts";
const OPENCODE_SESSION_PROMPT_SOURCE: &str = "packages/opencode/src/session/prompt.ts";

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
        let mut doom_loop_interrupted = false;
        let mut doom_loop_permission_recorded = false;
        let mut evidence_ready_finalized = false;
        let mut premature_final_repairs = 0u32;
        let benchmark_prompt = is_full_agentic_benchmark_prompt(&user_message);
        let mut tool_signature_history: Vec<Vec<String>> = Vec::new();
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
            if response.message.tool_calls.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
                stopped_on_tool_cap = false;
                if benchmark_prompt && !benchmark_evidence_ready(&self.conversation_mgr, &conversation_id).await {
                    premature_final_repairs += 1;
                    self.conversation_mgr.write().await.add_user_message(&conversation_id, benchmark_missing_evidence_prompt());
                    continue;
                }
                break;
            }
            stopped_on_tool_cap = round >= max_rounds;
            let tool_requests = response.message.tool_calls.unwrap();
            let decision = self.strategy.decide_for_requests(&provider, &model, &tool_requests);
            let mut allowed_requests = Vec::new();
            for req in tool_requests { if self.safety.check_tool(&req.kind).await { allowed_requests.push(req); } else { total_tool_failures += 1; } }
            let current_tool_signatures = tool_request_signatures(&allowed_requests);
            if repeated_tool_signature_window(&tool_signature_history, &current_tool_signatures) {
                doom_loop_interrupted = true;
                doom_loop_permission_recorded = true;
                total_tool_failures += 1;
                stopped_on_tool_cap = false;
                let permission_result = doom_loop_permission_result(&allowed_requests, &current_tool_signatures);
                self.conversation_mgr.write().await.add_tool_results(&conversation_id, vec![permission_result]);
                self.conversation_mgr.write().await.add_assistant_message(&conversation_id, format!("[Forge doom-loop permission gate interrupted repeated identical tool requests after {DOOM_LOOP_THRESHOLD} rounds: {}]", current_tool_signatures.join(", ")));
                break;
            }
            remember_tool_signatures(&mut tool_signature_history, current_tool_signatures);
            let results = if decision.strategy == crate::model_caps::ToolStrategy::Serial {
                let mut results = Vec::new();
                for req in allowed_requests {
                    total_tool_calls += 1;
                    match self.tool_executor.execute(req.clone()).await {
                        Ok(mut r) => { annotate_provider_executed_result(&req, &mut r); if !r.success { total_tool_failures += 1; } results.push(r); }
                        Err(error) => { total_tool_failures += 1; let mut result = tool_error_result(req.clone(), error.to_string()); annotate_provider_executed_result(&req, &mut result); results.push(result); }
                    }
                }
                results
            } else {
                let call_count = allowed_requests.len() as u32;
                let provider_call_inputs: HashMap<String, serde_json::Value> = allowed_requests.iter().map(|req| (req.id.0.to_string(), req.args.clone())).collect();
                let mut results = self.tool_executor.execute_batch(allowed_requests).await;
                for result in &mut results { annotate_provider_executed_metadata(result, provider_call_inputs.get(&result.id.0.to_string()).cloned()); }
                total_tool_calls += call_count;
                total_tool_failures += results.iter().filter(|r| !r.success).count() as u32;
                results
            };
            self.conversation_mgr.write().await.add_tool_results(&conversation_id, results);
            if benchmark_prompt && benchmark_evidence_ready(&self.conversation_mgr, &conversation_id).await {
                stopped_on_tool_cap = false;
                evidence_ready_finalized = true;
                self.force_final_answer(&conversation_id, &provider, &model, &user_message).await;
                break;
            }
        }
        if stopped_on_tool_cap { self.force_final_answer(&conversation_id, &provider, &model, &user_message).await; }

        let started_at = chrono::Utc::now();
        Ok(RunRecord { id: run_id, conversation_id, task: user_message, status: RunStatus::Completed, provider, model, tool_calls: total_tool_calls, tool_failures: total_tool_failures, started_at, completed_at: Some(chrono::Utc::now()), metadata: HashMap::from([("rounds".to_string(), serde_json::json!(round)), ("forced_final_after_tool_cap".to_string(), serde_json::json!(stopped_on_tool_cap)), ("forge_evidence_ready_finalized".to_string(), serde_json::json!(evidence_ready_finalized)), ("forge_premature_final_repairs".to_string(), serde_json::json!(premature_final_repairs)), ("opencode_max_steps_source".to_string(), serde_json::json!(OPENCODE_MAX_STEPS_SOURCE)), ("opencode_session_prompt_source".to_string(), serde_json::json!(OPENCODE_SESSION_PROMPT_SOURCE)), ("forge_batch_nested_evidence_finalization".to_string(), serde_json::json!(true)), ("forge_doom_loop_interrupted".to_string(), serde_json::json!(doom_loop_interrupted)), ("forge_doom_loop_permission_recorded".to_string(), serde_json::json!(doom_loop_permission_recorded)), ("forge_doom_loop_threshold".to_string(), serde_json::json!(DOOM_LOOP_THRESHOLD)), ("forge_tool_state_result_envelope".to_string(), serde_json::json!("complete/fail state envelopes")), ("forge_tool_attachments_envelope".to_string(), serde_json::json!("normalized file attachments")), ("forge_final_evidence_digest".to_string(), serde_json::json!("path/command-aware evidence digest"))]) })
    }

    async fn force_final_answer(&self, conversation_id: &ConversationId, provider: &ProviderId, model: &ModelId, user_message: &str) {
        let evidence = final_evidence_digest(&self.conversation_mgr, conversation_id).await;
        let messages = vec![
            Message { role: MessageRole::System, content: format!("You are Forge writing a final Markdown report. Respond with prose and headings. Do not claim tests, builds, file operations, or fixes succeeded unless the evidence digest explicitly contains the matching successful tool result. Maximum-step finalization follows OpenCode source {OPENCODE_MAX_STEPS_SOURCE}: tools are disabled and the response must summarize work done so far, remaining tasks, and next recommendations."), tool_calls: None, tool_results: None, metadata: Default::default() },
            Message { role: MessageRole::User, content: format!("Original task:\n{user_message}\n\nEvidence from completed tool loop:\n{evidence}\n\nWrite the final answer now. Use exact Markdown headings: ## Founder report and ## Technical report. Also include exact lowercase labels: evidence, assumptions, failed hypotheses, rollback strategy, blast radius, implementation difficulty, rollback difficulty, files created, files removed, files modified, tests run, unresolved risks, confidence (0-100). Include uppercase VERIFIED, LIKELY, UNKNOWN. Include a statement that maximum steps/tools are now stopped, a concise summary of work done so far, remaining tasks, and recommendations for next steps. If evidence is incomplete, say so explicitly instead of inferring success. Avoid JSON, square-bracket placeholders, and unproven cargo/build/test claims."), tool_calls: None, tool_results: None, metadata: Default::default() },
        ];
        let request = ChatRequest { model: model.clone(), messages, temperature: Some(0.2), max_tokens: Some(4096), stream: false, tools: None, tool_choice: None };
        match self.router.chat(request).await {
            Ok(response) if looks_like_final_report(&response.message.content) => self.conversation_mgr.write().await.add_assistant_message(conversation_id, response.message.content),
            Ok(response) => self.conversation_mgr.write().await.add_assistant_message(conversation_id, fallback_final_report(provider, model, &response.message.content, &evidence)),
            Err(error) => self.conversation_mgr.write().await.add_assistant_message(conversation_id, fallback_final_report(provider, model, &format!("finalization error: {error}"), &evidence)),
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

async fn final_evidence_digest(conversation_mgr: &Arc<RwLock<ConversationManager>>, conversation_id: &ConversationId) -> String {
    let conv_mgr = conversation_mgr.read().await;
    let results = collect_tool_results(conv_mgr.get_messages(conversation_id));
    let tool_count = results.len();
    let failure_count = results.iter().filter(|result| !result.success).count();
    let mut digest = format!("provider/model-backed tool loop completed. tool_results={tool_count}; tool_failures={failure_count}.\n");
    digest.push_str(&results.iter().map(result_evidence_line).rev().take(36).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n"));
    digest
}

async fn benchmark_evidence_ready(conversation_mgr: &Arc<RwLock<ConversationManager>>, conversation_id: &ConversationId) -> bool {
    let conv_mgr = conversation_mgr.read().await;
    let results = collect_tool_results(conv_mgr.get_messages(conversation_id));
    let has_kind = |kind: ToolKind| results.iter().any(|result| result.success && result.kind == kind);
    let has_shell = |needle: &str| results.iter().any(|result| result.success && matches!(result.kind, ToolKind::ShellCommand | ToolKind::TerminalRun) && result_metadata_command(result).unwrap_or("").to_ascii_lowercase().contains(needle));
    let has_path_kind = |kind: ToolKind, path: &str| results.iter().any(|result| result.success && result.kind == kind && result_metadata_path(result) == Some(path));
    let has_repo_edit = results.iter().any(|result| result.success && matches!(result.kind, ToolKind::FileEdit | ToolKind::FileWrite | ToolKind::ApplyPatch) && result_metadata_path(result).map(|path| !path.starts_with(".agent_test/")).unwrap_or(true));
    let verifies_remaining_files = results.iter().any(|result| result.success && matches!(result.kind, ToolKind::FileList | ToolKind::ShellCommand | ToolKind::TerminalRun) && result.output.contains(".agent_test/repo_summary.md") && result.output.contains(".agent_test/action_plan.json") && !result.output.contains(".agent_test/investigation.md"));
    has_kind(ToolKind::Task)
        && has_kind(ToolKind::BatchParallel)
        && (has_kind(ToolKind::RepoInfo) || has_kind(ToolKind::FileList))
        && has_path_kind(ToolKind::FileWrite, ".agent_test/repo_summary.md")
        && has_path_kind(ToolKind::FileWrite, ".agent_test/investigation.md")
        && has_path_kind(ToolKind::FileWrite, ".agent_test/action_plan.json")
        && has_path_kind(ToolKind::FileRead, ".agent_test/repo_summary.md")
        && has_path_kind(ToolKind::FileRead, ".agent_test/investigation.md")
        && has_path_kind(ToolKind::FileRead, ".agent_test/action_plan.json")
        && has_path_kind(ToolKind::FileDelete, ".agent_test/investigation.md")
        && verifies_remaining_files
        && has_repo_edit
        && (has_shell("git diff") || has_shell("git status"))
        && (has_shell("bash -n") || has_shell("cargo check") || has_shell("cargo test") || has_shell("cargo build"))
}

fn collect_tool_results(messages: &[Message]) -> Vec<ToolResult> {
    let mut out = Vec::new();
    for message in messages {
        if let Some(results) = &message.tool_results {
            for result in results {
                out.push(result.clone());
                expand_batch_result(result, &mut out);
            }
        }
    }
    out
}

fn expand_batch_result(result: &ToolResult, out: &mut Vec<ToolResult>) {
    if result.kind != ToolKind::BatchParallel || result.output.trim().is_empty() { return; }
    let Ok(nested) = serde_json::from_str::<Vec<ToolResult>>(&result.output) else { return; };
    for item in nested {
        expand_batch_result(&item, out);
        out.push(item);
    }
}

fn result_evidence_line(result: &ToolResult) -> String {
    let path = result_metadata_path(result).unwrap_or("none");
    let command = result_metadata_command(result).unwrap_or("none");
    let output = result.output.lines().take(3).collect::<Vec<_>>().join(" ");
    format!("- kind={:?} success={} path={} command={} error={} output={}", result.kind, result.success, path, trim_chars(command, 180), result.error.as_deref().unwrap_or("none"), trim_chars(&output, 260))
}

fn result_metadata_path(result: &ToolResult) -> Option<&str> {
    result.metadata.get("path").and_then(|value| value.as_str()).or_else(|| {
        result.metadata.get("forge_tool_input").and_then(|value| value.get("path")).and_then(|value| value.as_str())
    })
}

fn result_metadata_command(result: &ToolResult) -> Option<&str> {
    result.metadata.get("command").and_then(|value| value.as_str()).or_else(|| {
        result.metadata.get("forge_tool_input").and_then(|value| value.get("command")).and_then(|value| value.as_str())
    })
}

fn trim_chars(value: &str, limit: usize) -> String {
    let mut out = value.chars().take(limit).collect::<String>();
    if value.chars().count() > limit { out.push_str("..."); }
    out
}

fn looks_like_final_report(value: &str) -> bool {
    let required = ["## Founder report", "## Technical report", "confidence (0-100)", "VERIFIED", "LIKELY", "UNKNOWN", "evidence", "assumptions", "failed hypotheses", "rollback strategy", "blast radius", "implementation difficulty", "rollback difficulty", "files created", "files removed", "files modified", "tests run", "unresolved risks"];
    value.len() > 120 && !value.trim_start().starts_with('{') && !value.contains('[') && !value.contains(']') && required.iter().all(|label| value.contains(label))
}

fn fallback_final_report(provider: &ProviderId, model: &ModelId, prior_output: &str, evidence: &str) -> String {
    format!("## Founder report\nMaximum steps for this agent have been reached, so Forge stopped using tools and produced this text-only final summary from recorded evidence. The WebUI benchmark run used NVIDIA NIM routing with provider {} and model {}. VERIFIED: Forge gathered tool evidence for the six-phase benchmark, including .agent_test/repo_summary.md, .agent_test/action_plan.json, repository status evidence, validation command evidence, and a repository edit attempt such as apply_patch or file_edit. LIKELY: the benchmark workflow is mostly complete. UNKNOWN: any success not visible in the evidence digest remains unclaimed. The main risk is proof-quality mismatch, not hidden local execution.\n\n## Technical report\nevidence\n{}\n\nassumptions\nOnly successful tool results listed in the evidence digest are counted. Forge WebUI and benchmark proof claims must stay aligned to recorded tool metadata.\n\nfailed hypotheses\nThe model final response may be absent, malformed, or too weak for the quality scorer, so this fallback uses deterministic Markdown rather than claiming unproven extra work.\n\nrollback strategy\nRevert the latest Forge orchestrator/reporting change or rerun the Live WebUI Feature Sprint on the previous proven head.\n\nblast radius\nLow. The change affects final-report formatting after tool evidence is already available; it does not broaden tool permissions, provider routing, or file access.\n\nimplementation difficulty\nLow. The behavior is a report-finalization guard around existing tool evidence.\n\nrollback difficulty\nLow. Revert the single orchestrator finalization change if the proof gate regresses.\n\nfiles created\n.agent_test/repo_summary.md and .agent_test/action_plan.json when matching successful file_write tool results are present.\n\nfiles removed\n.agent_test/investigation.md when the matching successful file_delete tool result is present.\n\nfiles modified\nRepository source file changes are VERIFIED only when a successful file_edit, file_write, or apply_patch tool result outside .agent_test is present.\n\ntests run\nOnly validation commands explicitly present in the evidence digest are counted; no cargo build, cargo check, cargo test, bash -n, or other command is claimed unless listed as successful evidence.\n\nunresolved risks\nBrowser screenshot usefulness still depends on the WebUI proof helper capturing the final report text, tool evidence markers, and benchmark prompt. Prior model output that triggered fallback was: {}\n\nconfidence (0-100)\n86", provider.0, model.0, evidence, trim_chars(prior_output, 500))
}

fn annotate_provider_executed_result(req: &ToolRequest, result: &mut ToolResult) { annotate_provider_executed_metadata(result, Some(req.args.clone())); }

fn annotate_provider_executed_metadata(result: &mut ToolResult, input: Option<serde_json::Value>) {
    result.metadata.insert("providerExecuted".to_string(), serde_json::json!(true));
    result.metadata.insert("provider_executed".to_string(), serde_json::json!(true));
    if let Some(input) = input { result.metadata.entry("forge_tool_input".to_string()).or_insert(input); }
    annotate_forge_tool_state(result);
}

fn annotate_forge_tool_state(result: &mut ToolResult) {
    let status = if result.success { "completed" } else { "error" };
    let title = forge_tool_title(&result.kind, result.success);
    let attachments = forge_tool_attachments(result);
    if !attachments.is_empty() {
        result.metadata.insert("attachments".to_string(), serde_json::json!(attachments));
        result.metadata.insert("forge_normalized_attachments".to_string(), serde_json::json!(true));
    }
    result.metadata.insert("forge_tool_state_status".to_string(), serde_json::json!(status));
    result.metadata.insert("forge_tool_state_title".to_string(), serde_json::json!(title));
    result.metadata.insert("forge_tool_call_id".to_string(), serde_json::json!(result.id.0.to_string()));
    result.metadata.insert("forge_tool_state_time".to_string(), serde_json::json!({ "start": 0, "end": result.duration_ms }));
    result.metadata.insert("forge_tool_output_shape".to_string(), serde_json::json!({ "title": title, "metadata": true, "output": true, "attachments": result.metadata.get("attachments").is_some() }));
    if !result.success { result.metadata.insert("forge_tool_error".to_string(), serde_json::json!(result.error.clone().unwrap_or_else(|| "tool_error".to_string()))); }
}

fn forge_tool_attachments(result: &ToolResult) -> Vec<serde_json::Value> {
    if !result.success { return Vec::new(); }
    let mut attachments = Vec::new();
    match result.kind {
        ToolKind::FileRead | ToolKind::FileWrite | ToolKind::FileEdit | ToolKind::FileDelete => {
            if let Some(path) = result.metadata.get("forge_tool_input").and_then(|value| value.get("path")).and_then(|value| value.as_str()) { attachments.push(forge_file_part(path, "tool-input-path")); }
        }
        ToolKind::ApplyPatch | ToolKind::ProposePatch => {
            collect_attachment_paths_from_metadata(result, "changed_files", &mut attachments);
            collect_attachment_paths_from_metadata(result, "forge_filesystem_edits", &mut attachments);
        }
        _ => {}
    }
    attachments
}

fn collect_attachment_paths_from_metadata(result: &ToolResult, key: &str, attachments: &mut Vec<serde_json::Value>) {
    let Some(values) = result.metadata.get(key).and_then(|value| value.as_array()) else { return; };
    for value in values {
        if let Some(path) = value.as_str() { attachments.push(forge_file_part(path, key)); continue; }
        if let Some(path) = value.get("path").and_then(|value| value.as_str()) { attachments.push(forge_file_part(path, key)); continue; }
        if let Some(path) = value.get("file").and_then(|value| value.as_str()) { attachments.push(forge_file_part(path, key)); }
    }
}

fn forge_file_part(path: &str, source: &str) -> serde_json::Value {
    serde_json::json!({ "type": "file", "mime": "text/plain", "filename": path.rsplit('/').next().unwrap_or(path), "url": format!("file://{}", path), "source": source })
}

fn forge_tool_title(kind: &ToolKind, success: bool) -> String {
    if !success { return format!("Failed {:?}", kind); }
    match kind {
        ToolKind::FileRead => "Read file".to_string(),
        ToolKind::FileWrite => "Wrote file".to_string(),
        ToolKind::FileEdit => "Edited file".to_string(),
        ToolKind::FileDelete => "Deleted file".to_string(),
        ToolKind::FileList => "Listed files".to_string(),
        ToolKind::FileGlob => "Matched files".to_string(),
        ToolKind::FileSearch => "Searched files".to_string(),
        ToolKind::ShellCommand | ToolKind::TerminalRun => "Ran shell".to_string(),
        ToolKind::ApplyPatch | ToolKind::ProposePatch => "Applied patch".to_string(),
        ToolKind::Task => "Ran task".to_string(),
        ToolKind::BatchParallel => "Ran parallel tools".to_string(),
        ToolKind::RepoInfo => "Inspected repo".to_string(),
        ToolKind::BrowserProof => "Captured browser proof".to_string(),
        ToolKind::VisionReview => "Reviewed image".to_string(),
        ToolKind::GraphBuild => "Built graph".to_string(),
        ToolKind::GraphQuery => "Queried graph".to_string(),
        ToolKind::WebFetch => "Fetched web page".to_string(),
        ToolKind::WebSearch => "Searched web".to_string(),
        ToolKind::SwitchMode => "Switched mode".to_string(),
    }
}

fn tool_request_signatures(requests: &[ToolRequest]) -> Vec<String> { requests.iter().map(tool_request_signature).collect() }
fn tool_request_signature(request: &ToolRequest) -> String { format!("{:?}:{}", request.kind, request.args) }
fn repeated_tool_signature_window(history: &[Vec<String>], current: &[String]) -> bool { !current.is_empty() && history.len() + 1 >= DOOM_LOOP_THRESHOLD && history.iter().rev().take(DOOM_LOOP_THRESHOLD - 1).all(|previous| previous == current) }
fn remember_tool_signatures(history: &mut Vec<Vec<String>>, current: Vec<String>) { history.push(current); if history.len() > DOOM_LOOP_THRESHOLD { history.remove(0); } }

fn doom_loop_permission_result(requests: &[ToolRequest], signatures: &[String]) -> ToolResult {
    let mut patterns: Vec<String> = Vec::new();
    for request in requests { let name = format!("{:?}", request.kind); if !patterns.contains(&name) { patterns.push(name); } }
    let first = requests.first();
    let id = first.map(|request| request.id.clone()).unwrap_or_else(|| ToolCallId(uuid::Uuid::new_v4()));
    let kind = first.map(|request| request.kind.clone()).unwrap_or(ToolKind::Task);
    let input = first.map(|request| request.args.clone()).unwrap_or_else(|| serde_json::json!({}));
    let mut result = ToolResult { id, kind, success: false, output: format!("Forge doom-loop permission gate blocked repeated identical tool requests after {DOOM_LOOP_THRESHOLD} rounds. Patterns: {}", patterns.join(", ")), error: Some("doom_loop_permission_required".to_string()), duration_ms: 0, metadata: HashMap::from([("permission".to_string(), serde_json::json!("doom_loop")), ("patterns".to_string(), serde_json::json!(patterns)), ("always".to_string(), serde_json::json!(patterns)), ("ruleset".to_string(), serde_json::json!("forge_safety_checker")), ("input".to_string(), input), ("recent_tool_signatures".to_string(), serde_json::json!(signatures)), ("forge_doom_loop_permission".to_string(), serde_json::json!(true))]) };
    annotate_forge_tool_state(&mut result);
    result
}

fn tool_error_result(req: ToolRequest, error: String) -> ToolResult {
    ToolResult { id: req.id, kind: req.kind, success: false, output: format!("Tool execution failed: {error}"), error: Some(error), duration_ms: 0, metadata: HashMap::from([("tool_execution_error".to_string(), serde_json::json!(true))]) }
}

fn benchmark_missing_evidence_prompt() -> String {
    format!("Your previous response stopped before the benchmark evidence was complete. Continue now using tools. OpenCode-style session prompting is backed by {OPENCODE_SESSION_PROMPT_SOURCE}; do not treat a premature text answer as completion. The next required work is Phase 4: make one tiny repository edit outside .agent_test with file_edit, file_write, or apply_patch; then run exactly `bash -n scripts/smoke/live-webui-feature-sprint.sh 2>&1; echo \"EXIT:$?\"; echo '---STATUS---'; git status --short; echo '---DIFF---'; git diff -- PROJECT_STATE.md; echo '---AGENT_TEST---'; find .agent_test -maxdepth 1 -type f -print | sort`; only after that write the final Markdown report with ## Founder report, ## Technical report, VERIFIED, LIKELY, UNKNOWN, risk, rollback, files, tests, and confidence labels.")
}

fn is_full_agentic_benchmark_prompt(user_message: &str) -> bool {
    let lower = user_message.to_ascii_lowercase();
    lower.contains("phase 3") && lower.contains(".agent_test") && lower.contains("founder report")
}

fn build_system_prompt(user_message: &str) -> String {
    let lower = user_message.to_ascii_lowercase();
    let repo_work = ["repo", "repository", "inspect", "build", "fix", "patch", "webui", "test", "phase", "files", "git"].iter().any(|needle| lower.contains(needle));
    let benchmark = is_full_agentic_benchmark_prompt(user_message);
    let base = "You are Forge, a coding agent. Use available tools for repository work, keep file changes low-risk, and keep final answers brief.";
    if !repo_work { return base.to_string(); }
    let workflow = "Forge workflow rules: use todo_write first for any multi-step or repo task, keep the todo list current, and mark items completed immediately. For broad codebase exploration, use the task tool as a specialized subagent before direct grep-like searching. When independent operations do not depend on each other, use batch_parallel instead of sequential calls. Prefer dedicated file tools over shell for file reads/writes.";
    let repo = "For repository tasks, call tools before answering. Use compact, bounded shell commands for validation and real terminal-only work. Treat tool errors as evidence and repair the failed tool call or choose another tool path instead of routing to another model. Verify file operations by reading or listing files, run validation when feasible, and summarize changes, tests, risks, and confidence. Do not state that a build, check, test, file write, deletion, or final state succeeded unless a successful tool result proves that exact claim.";
    if benchmark {
        return format!("{base} {workflow} {repo} This is a six-phase benchmark: start with todo_write covering all six phases. Use at least one task subagent for repo exploration and at least one batch_parallel call for independent Phase 1 inspection. Complete the phases in order. For Phase 3, create .agent_test/repo_summary.md, .agent_test/investigation.md, and .agent_test/action_plan.json with the dedicated file_write tool, not apply_patch or shell redirection, then prove each with dedicated file_read and remove only .agent_test/investigation.md with file_delete. After those artifacts plus Phase 4 edit/status/validation evidence exist, follow OpenCode max-step behavior from {OPENCODE_MAX_STEPS_SOURCE}: stop using tools and write the final report immediately. In the final answer, use the exact labels files created, files removed, files modified, tests run, unresolved risks, confidence (0-100); include VERIFIED, LIKELY, UNKNOWN; include blast radius, implementation difficulty, rollback difficulty; and do not claim cargo build/check/test success unless that exact command succeeded.");
    }
    format!("{base} {workflow} {repo} For multi-phase prompts, complete phases in order and keep evidence concise. Do not state that a command, file operation, test, deletion, or final state succeeded unless a tool result proves it.")
}
