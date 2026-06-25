//! Batch parallel tool execution.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolCallId, ToolKind};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

impl ToolExecutor {
    pub async fn execute_batch_parallel(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { calls: Vec<BatchCall> }
        #[derive(serde::Deserialize)]
        struct BatchCall { tool: String, args: serde_json::Value }
        
        let args: Args = serde_json::from_value(request.args)?;
        
        let mut sub_requests = Vec::new();
        for (i, call) in args.calls.into_iter().enumerate() {
            let kind = match call.tool.as_str() {
                "file_read" => ToolKind::FileRead,
                "file_write" => ToolKind::FileWrite,
                "file_edit" => ToolKind::FileEdit,
                "file_delete" => ToolKind::FileDelete,
                "file_list" => ToolKind::FileList,
                "file_glob" => ToolKind::FileGlob,
                "file_search" => ToolKind::FileSearch,
                "web_fetch" => ToolKind::WebFetch,
                "web_search" => ToolKind::WebSearch,
                "shell" => ToolKind::ShellCommand,
                "terminal" => ToolKind::TerminalRun,
                "repo_info" => ToolKind::RepoInfo,
                _ => continue,
            };
            
            sub_requests.push(ToolRequest {
                id: ToolCallId(Uuid::new_v4()),
                kind,
                args: call.args,
                parallel_group: Some(request.id.0.to_string()),
            });
        }
        
        let results = self.execute_batch(sub_requests).await;
        
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::BatchParallel,
            success: success_count == total_count,
            output: serde_json::to_string_pretty(&results)?,
            error: if success_count == total_count { None } else { Some(format!("{}/{} failed", total_count - success_count, total_count)) },
            duration_ms: 0,
            metadata: HashMap::from([
                ("total_calls".to_string(), serde_json::json!(total_count)),
                ("successful".to_string(), serde_json::json!(success_count)),
                ("failed".to_string(), serde_json::json!(total_count - success_count)),
            ]),
        })
    }
}