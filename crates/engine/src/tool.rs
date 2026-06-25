//! Tool execution system with parallel batch support.

use crate::types::{ToolKind, ToolRequest, ToolResult, ToolConfig};
use anyhow::Result;
use std::time::Instant;
use serde_json::json;

pub mod batch;
pub mod browser;
pub mod file_ops;
pub mod graph;
pub mod shell_ops;
pub mod task_ops;
pub mod web_ops;


#[derive(Debug, Clone)]
pub struct ToolExecutor {
    workspace_root: String,
    timeout_ms: u64,
    max_parallel: usize,
}

impl ToolExecutor {
    pub fn new(workspace_root: String, timeout_ms: u64, max_parallel: usize) -> Self {
        Self { workspace_root, timeout_ms, max_parallel }
    }

    pub async fn execute(&self, request: ToolRequest) -> Result<ToolResult> {
        let start = Instant::now();
        let result = match request.kind {
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
            ToolKind::SwitchMode => self.execute_switch_mode(request).await,
            ToolKind::BrowserProof => self.execute_browser_proof(request).await,
            ToolKind::VisionReview => self.execute_vision_review(request).await,
            ToolKind::GraphBuild => self.execute_graph_build(request).await,
            ToolKind::GraphQuery => self.execute_graph_query(request).await,
        };
        
        let duration = start.elapsed().as_millis() as u64;
        result.map(|mut r| {
            r.duration_ms = duration;
            r
        })
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
                    "pattern": { "type": "string", "description": "Glob pattern (e.g. **/*.rs)" }
                },
                "required": ["pattern"]
            }),
        },
        ToolConfig {
            name: "file_search".to_string(),
            description: "Search for text in files using regex".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "pattern": { "type": "string", "description": "Regex pattern to search" },
                    "path": { "type": "string", "description": "Directory to search (optional)" }
                },
                "required": ["pattern"]
            }),
        },
        ToolConfig {
            name: "web_fetch".to_string(),
            description: "Fetch the contents of a URL and return the text".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "URL to fetch" }
                },
                "required": ["url"]
            }),
        },
        ToolConfig {
            name: "web_search".to_string(),
            description: "Search the web for information".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" }
                },
                "required": ["query"]
            }),
        },
        ToolConfig {
            name: "shell_command".to_string(),
            description: "Run a shell command and return its output".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Shell command to run" },
                    "timeout": { "type": "number", "description": "Timeout in seconds (optional)" }
                },
                "required": ["command"]
            }),
        },
        ToolConfig {
            name: "terminal_run".to_string(),
            description: "Run a command in an interactive terminal".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Command to run" },
                    "cwd": { "type": "string", "description": "Working directory (optional)" }
                },
                "required": ["command"]
            }),
        },
        ToolConfig {
            name: "task".to_string(),
            description: "Create a subagent task for delegated work".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "description": { "type": "string", "description": "Task description" },
                    "background": { "type": "boolean", "description": "Run in background" }
                },
                "required": ["description"]
            }),
        },
        ToolConfig {
            name: "repo_info".to_string(),
            description: "Get repository information (git root, branch, HEAD)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        ToolConfig {
            name: "propose_patch".to_string(),
            description: "Propose a patch/summary of changes".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "summary": { "type": "string", "description": "Summary of changes" },
                    "diff": { "type": "string", "description": "Diff content" }
                },
                "required": ["summary", "diff"]
            }),
        },
        ToolConfig {
            name: "switch_mode".to_string(),
            description: "Switch the agent mode (chat, explore, plan, build)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "mode": { "type": "string", "enum": ["chat", "explore", "plan", "build"], "description": "Mode to switch to" }
                },
                "required": ["mode"]
            }),
        },
        ToolConfig {
            name: "browser_proof".to_string(),
            description: "Open a headless browser, take a screenshot, and optionally dump the DOM. Use this to debug UI issues, verify that pages load correctly, or inspect visual output.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "URL to open in the browser" },
                    "width": { "type": "integer", "description": "Viewport width in pixels (default 1280)" },
                    "height": { "type": "integer", "description": "Viewport height in pixels (default 720)" },
                    "capture_dom": { "type": "boolean", "description": "Whether to capture DOM snapshot (default true)" }
                },
                "required": ["url"]
            }),
        },
        ToolConfig {
            name: "vision_review".to_string(),
            description: "Send a screenshot image to a vision-capable AI model for analysis. Use this to detect UI bugs, visual errors, or layout issues in screenshots.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "image_base64": { "type": "string", "description": "Base64-encoded PNG screenshot" },
                    "prompt": { "type": "string", "description": "Custom analysis prompt (optional)" }
                },
                "required": ["image_base64"]
            }),
        },
        ToolConfig {
            name: "graph_build".to_string(),
            description: "Build a knowledge graph from code files in the workspace. Extracts imports, dependencies, and file structure into a queryable graph.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "pattern": { "type": "string", "description": "Glob pattern for files to include (default **/crates/**/*.rs)", "default": "**/crates/**/*.rs" }
                },
                "required": []
            }),
        },
        ToolConfig {
            name: "graph_query".to_string(),
            description: "Query a previously built knowledge graph. Search for files, functions, or dependencies by keyword.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "graph_json": { "type": "string", "description": "JSON output from a previous graph_build call" },
                    "query": { "type": "string", "description": "Search query (file name, function, import, etc.)" }
                },
                "required": ["graph_json", "query"]
            }),
        },
    ]
}