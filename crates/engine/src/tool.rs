//! Tool execution system with parallel batch support.

use crate::change_bus::{ChangeBus, ChangeBusStatus, ChangeEvent};
use crate::types::{ToolConfig, ToolKind, ToolRequest, ToolResult};
use anyhow::Result;
use serde_json::{json, Value};
use std::time::Instant;

pub mod batch;
pub mod browser;
pub mod file_ops;
pub mod graph;
pub mod patch_apply;
pub mod patch_events;
pub mod patch_ops;
pub mod shell_ops;
pub mod task_ops;
pub mod web_ops;

#[derive(Debug, Clone)]
pub struct ToolExecutor {
    pub(crate) workspace_root: String,
    timeout_ms: u64,
    max_parallel: usize,
    change_bus: ChangeBus,
}

impl ToolExecutor {
    pub fn new(workspace_root: String, timeout_ms: u64, max_parallel: usize) -> Self {
        let change_bus = ChangeBus::new(workspace_root.clone());
        Self { workspace_root, timeout_ms, max_parallel, change_bus }
    }

    pub fn change_bus(&self) -> ChangeBus { self.change_bus.clone() }
    pub fn recent_change_events(&self) -> Vec<ChangeEvent> { self.change_bus.recent() }
    pub fn change_bus_status(&self) -> ChangeBusStatus { self.change_bus.status() }
    pub fn subscribe_change_events(&self) -> tokio::sync::broadcast::Receiver<ChangeEvent> { self.change_bus.subscribe() }

    pub async fn execute(&self, request: ToolRequest) -> Result<ToolResult> {
        let start = Instant::now();
        let kind = request.kind.clone();
        let result = match kind {
            ToolKind::FileRead => self.execute_file_read(request).await,
            ToolKind::FileWrite => self.execute_file_write(request).await,
            ToolKind::FileEdit => self.execute_file_edit(request).await,
            ToolKind::FileDelete => self.execute_file_delete(request).await,
            ToolKind::FileList => self.execute_file_list(request).await,
            ToolKind::FileGlob => self.execute_file_glob(request).await,
            ToolKind::FileSearch => self.execute_file_search(request).await,
            ToolKind::WebFetch => self.execute_web_fetch(request).await,
            ToolKind::WebSearch => self.execute_web_search(request).await,
            ToolKind::ShellCommand => self.execute_shell(request).await,
            ToolKind::TerminalRun => self.execute_terminal(request).await,
            ToolKind::Task => self.execute_task(request).await,
            ToolKind::BatchParallel => self.execute_batch_parallel(request).await,
            ToolKind::RepoInfo => self.execute_repo_info(request).await,
            ToolKind::ProposePatch => self.execute_propose_patch(request).await,
            ToolKind::ApplyPatch => self.execute_apply_patch(request).await,
            ToolKind::SwitchMode => self.execute_switch_mode(request).await,
            ToolKind::BrowserProof => self.execute_browser_proof(request).await,
            ToolKind::VisionReview => self.execute_vision_review(request).await,
            ToolKind::GraphBuild => self.execute_graph_build(request).await,
            ToolKind::GraphQuery => self.execute_graph_query(request).await,
        };
        let duration = start.elapsed().as_millis() as u64;
        result.map(|mut r| {
            r.duration_ms = duration;
            let receipts = self.publish_change_events(&r);
            if !receipts.is_empty() {
                r.metadata.insert("event_bus_receipts".to_string(), json!(receipts));
                r.metadata.insert("event_bus_status".to_string(), json!(self.change_bus.status()));
            }
            r
        })
    }

    fn publish_change_events(&self, result: &ToolResult) -> Vec<ChangeEvent> {
        if !result.success { return Vec::new(); }
        let mut receipts = Vec::new();
        receipts.extend(self.publish_metadata_array(result, "forge_filesystem_edits", "filesystem.edited"));
        receipts.extend(self.publish_metadata_array(result, "forge_watcher_updates", "watcher.updated"));
        receipts.extend(self.publish_metadata_array(result, "forge_lsp_warmups", "lsp.warmup.contained"));
        receipts.extend(self.publish_metadata_array(result, "forge_lsp_diagnostics", "lsp.diagnostics"));
        receipts
    }

    fn publish_metadata_array(&self, result: &ToolResult, key: &str, event_type: &str) -> Vec<ChangeEvent> {
        let Some(values) = result.metadata.get(key).and_then(Value::as_array) else { return Vec::new(); };
        let tool_id = result.id.clone().0.to_string();
        let tool_kind = format!("{:?}", &result.kind);
        let source = result.metadata.get("forge_event_publisher").and_then(Value::as_str).unwrap_or("forge.tool").to_string();
        values.iter().map(|value| {
            let mut payload = value.clone();
            if let Some(object) = payload.as_object_mut() {
                object.insert("tool_id".to_string(), json!(tool_id.clone()));
                object.insert("tool_kind".to_string(), json!(tool_kind.clone()));
                object.insert("metadata_key".to_string(), json!(key));
            }
            self.change_bus.publish(event_type, &source, payload)
        }).collect()
    }

    pub async fn execute_batch(&self, requests: Vec<ToolRequest>) -> Vec<ToolResult> {
        use futures::stream::{self, StreamExt};
        stream::iter(requests).map(|req| self.execute(req)).buffer_unordered(self.max_parallel).filter_map(|r| async move { r.ok() }).collect().await
    }
}

fn schema(properties: serde_json::Value, required: &[&str]) -> serde_json::Value {
    json!({"type":"object","properties":properties,"required":required})
}

fn tool(name: &str, description: &str, properties: serde_json::Value, required: &[&str]) -> ToolConfig {
    ToolConfig { name: name.to_string(), description: description.to_string(), parameters: schema(properties, required) }
}

pub fn tool_definitions() -> Vec<ToolConfig> {
    vec![
        tool("repo_info", "Inspect repository identity, branch, remotes, dirty state, and workspace summary before changing files.", json!({}), &[]),
        tool("todo_write", "Create or update a visible checklist before and during multi-step work; mark items in_progress/completed immediately.", json!({"todos":{"type":"array","items":{"type":"object","properties":{"content":{"type":"string"},"status":{"type":"string","enum":["pending","in_progress","completed"]},"priority":{"type":"string","enum":["high","medium","low"]}},"required":["content","status"]}}}), &["todos"]),
        tool("file_list", "List files under a workspace-relative directory with optional depth.", json!({"path":{"type":"string"},"depth":{"type":"integer","minimum":1,"maximum":5}}), &[]),
        tool("file_glob", "Find files by glob pattern under an optional workspace-relative directory.", json!({"pattern":{"type":"string"},"path":{"type":"string"}}), &["pattern"]),
        tool("file_search", "Search text in workspace files by literal pattern and optional file glob.", json!({"pattern":{"type":"string"},"path":{"type":"string"},"file_pattern":{"type":"string"}}), &["pattern"]),
        tool("file_read", "Read a workspace-relative UTF-8 file.", json!({"path":{"type":"string"}}), &["path"]),
        tool("file_write", "Write content to a workspace-relative file, then emit formatter/watcher/LSP receipts when available.", json!({"path":{"type":"string"},"content":{"type":"string"}}), &["path","content"]),
        tool("file_edit", "Replace exact file text in a workspace-relative file with mutation metadata.", json!({"path":{"type":"string"},"old_string":{"type":"string"},"new_string":{"type":"string"},"replace_all":{"type":"boolean"}}), &["path","old_string","new_string"]),
        tool("file_delete", "Remove a workspace-relative file and emit watcher/LSP event receipts.", json!({"path":{"type":"string"}}), &["path"]),
        tool("apply_patch", "Apply a multi-file patch using an explicit patchText payload, permission metadata, formatter hooks, watcher updates, and LSP diagnostics.", json!({"patchText":{"type":"string","description":"The full patch text that describes all changes to be made"}}), &["patchText"]),
        tool("propose_patch", "Prepare a patch for review before approval and application.", json!({"patchText":{"type":"string"},"reason":{"type":"string"}}), &["patchText"]),
        tool("shell_command", "Run a bounded shell command for repo inspection, validation, tests, or diagnostics.", json!({"command":{"type":"string"},"cwd":{"type":"string"},"timeout_ms":{"type":"integer","minimum":1000,"maximum":180000}}), &["command"]),
        tool("terminal_run", "Run an interactive-terminal style bounded command when the workflow needs terminal semantics.", json!({"command":{"type":"string"},"cwd":{"type":"string"},"timeout_ms":{"type":"integer","minimum":1000,"maximum":180000}}), &["command"]),
        tool("task", "Launch a bounded subagent for delegated repo exploration; use focused agents for broad codebase search before direct grep.", json!({"description":{"type":"string"},"prompt":{"type":"string"},"agent":{"type":"string"},"tools":{"type":"array","items":{"type":"string"}},"background":{"type":"boolean"}}), &["prompt"]),
        tool("batch_parallel", "Run independent tool requests concurrently. Each item should be an object with a tool name and args, or a one-key shorthand object whose key is the tool name and value is that tool's args.", json!({"requests":{"type":"array","items":{"type":"object","properties":{"tool":{"type":"string"},"args":{"type":"object"}}}}}), &["requests"]),
        tool("web_fetch", "Fetch a public URL for source inspection when network is allowed.", json!({"url":{"type":"string"}}), &["url"]),
        tool("web_search", "Search the web for source-backed current information when network is allowed.", json!({"query":{"type":"string"}}), &["query"]),
        tool("browser_proof", "Capture browser screenshot and optional DOM proof for visible WebUI validation.", json!({"url":{"type":"string"},"width":{"type":"integer"},"height":{"type":"integer"},"capture_dom":{"type":"boolean"}}), &["url"]),
        tool("vision_review", "Review an image proof with the configured vision provider.", json!({"image_base64":{"type":"string"},"prompt":{"type":"string"}}), &["image_base64"]),
        tool("graph_build", "Build a workspace code graph for repo understanding.", json!({"path":{"type":"string"}}), &[]),
        tool("graph_query", "Query the workspace code graph.", json!({"query":{"type":"string"}}), &["query"]),
        tool("switch_mode", "Switch the conversation mode between chat, explore, plan, and build.", json!({"mode":{"type":"string","enum":["chat","explore","plan","build"]}}), &["mode"]),
    ]
}
