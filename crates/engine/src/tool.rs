//! Tool execution system with parallel batch support.

use crate::change_bus::{ChangeBus, ChangeBusStatus, ChangeEvent};
use crate::types::{ToolKind, ToolRequest, ToolResult, ToolConfig};
use anyhow::Result;
use std::time::Instant;
use serde_json::{json, Value};

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
        Self { workspace_root, timeout_ms, max_parallel, change_bus: ChangeBus::new() }
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
        receipts.extend(self.publish_metadata_array(result, "opencode_filesystem_edits", "filesystem.edited"));
        receipts.extend(self.publish_metadata_array(result, "opencode_watcher_updates", "watcher.updated"));
        receipts.extend(self.publish_metadata_array(result, "opencode_lsp_warmups", "lsp.warmup.contained"));
        receipts.extend(self.publish_metadata_array(result, "opencode_lsp_diagnostics", "lsp.diagnostics"));
        receipts
    }

    fn publish_metadata_array(&self, result: &ToolResult, key: &str, event_type: &str) -> Vec<ChangeEvent> {
        let Some(values) = result.metadata.get(key).and_then(Value::as_array) else { return Vec::new(); };
        values.iter().map(|value| {
            let mut payload = value.clone();
            if let Some(object) = payload.as_object_mut() {
                object.insert("tool_id".to_string(), json!(result.id.0.to_string()));
                object.insert("tool_kind".to_string(), json!(format!("{:?}", &result.kind)));
                object.insert("metadata_key".to_string(), json!(key));
            }
            self.change_bus.publish(event_type, "opencode.apply_patch", payload)
        }).collect()
    }

    pub async fn execute_batch(&self, requests: Vec<ToolRequest>) -> Vec<ToolResult> {
        use futures::stream::{self, StreamExt};

        stream::iter(requests)
            .map(|req| self.execute(req))
            .buffer_unordered(self.max_parallel)
            .filter_map(|r| async move { r.ok() })
            .collect()
            .await
    }
}

pub fn tool_definitions() -> Vec<ToolConfig> {
    vec![
        ToolConfig {
            name: "file_read".to_string(),
            description: "Read the contents of a file at the given path".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path relative to workspace root" }
                },
                "required": ["path"]
            }),
        },
        ToolConfig {
            name: "file_write".to_string(),
            description: "Write content to a file (creates or overwrites)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path relative to workspace root" },
                    "content": { "type": "string", "description": "Content to write" }
                },
                "required": ["path", "content"]
            }),
        },
        ToolConfig {
            name: "file_edit".to_string(),
            description: "Edit a file by replacing exact text with new text".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path relative to workspace root" },
                    "old_string": { "type": "string", "description": "Exact text to replace" },
                    "new_string": { "type": "string", "description": "Replacement text" }
                },
                "required": ["path", "old_string", "new_string"]
            }),
        },
        ToolConfig {
            name: "file_delete".to_string(),
            description: "Delete a file at the given path".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path relative to workspace root" }
                },
                "required": ["path"]
            }),
        },
        ToolConfig {
            name: "file_list".to_string(),
            description: "List files and directories in the given directory".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Directory path relative to workspace root" }
                },
                "required": ["path"]
            }),
        },
        ToolConfig {
            name: "file_glob".to_string(),
            description: "Find files matching a glob pattern".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "pattern": { "type": "string", "description": "Glob pattern" }
                },
                "required": ["pattern"]
            }),
        },
        ToolConfig {
            name: "file_search".to_string(),
            description: "Search for text in files".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" }
                },
                "required": ["query"]
            }),
        },
        ToolConfig {
            name: "web_fetch".to_string(),
            description: "Fetch content from a URL".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "URL to fetch" }
                },
                "required": ["url"]
            }),
        },
        ToolConfig {
            name: "shell_command".to_string(),
            description: "Execute a shell command".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Command to execute" }
                },
                "required": ["command"]
            }),
        },
        ToolConfig {
            name: "batch_parallel".to_string(),
            description: "Execute multiple tool calls in parallel",
            parameters: json!({
                "type": "object",
                "properties": {
                    "requests": { "type": "array", "description": "Tool requests to execute" }
                },
                "required": ["requests"]
            }),
        },
    ]
}
