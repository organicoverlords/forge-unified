//! Tool execution system with parallel batch support.

use crate::types::{ToolKind, ToolRequest, ToolResult, ToolCallId};
use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

pub mod file_ops;
pub mod web_ops;
pub mod shell_ops;
pub mod batch;
pub mod task_ops;

pub use file_ops::*;
pub use web_ops::*;
pub use shell_ops::*;
pub use batch::*;
pub use task_ops::*;

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