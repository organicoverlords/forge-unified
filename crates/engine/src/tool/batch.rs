//! Batch parallel tool execution.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolCallId, ToolKind};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

impl ToolExecutor {
    pub async fn execute_batch_parallel(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { calls: Option<Vec<BatchCall>>, requests: Option<Vec<BatchCall>> }
        #[derive(serde::Deserialize)]
        struct BatchCall { tool: String, args: serde_json::Value }
        let args: Args = serde_json::from_value(request.args)?;
        let calls = args.calls.or(args.requests).unwrap_or_default();

        let mut sub_requests = Vec::new();
        let mut skipped = Vec::new();
        for call in calls {
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
                "shell" | "shell_command" => ToolKind::ShellCommand,
                "terminal" | "terminal_run" => ToolKind::TerminalRun,
                "task" | "todo" | "todo_write" => ToolKind::Task,
                "repo_info" => ToolKind::RepoInfo,
                _ => { skipped.push(call.tool); continue; }
            };

            sub_requests.push(ToolRequest {
                id: ToolCallId(Uuid::new_v4()),
                kind,
                args: call.args,
                parallel_group: Some(request.id.0.to_string()),
            });
        }

        let planned_count = sub_requests.len();
        let results = self.execute_batch(sub_requests).await;
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();
        let failed_count = total_count.saturating_sub(success_count) + skipped.len();

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::BatchParallel,
            success: failed_count == 0 && planned_count > 0,
            output: serde_json::to_string_pretty(&results)?,
            error: if failed_count == 0 { None } else { Some(format!("{failed_count} failed_or_skipped")) },
            duration_ms: 0,
            metadata: HashMap::from([
                ("total_calls".to_string(), serde_json::json!(total_count)),
                ("successful".to_string(), serde_json::json!(success_count)),
                ("failed".to_string(), serde_json::json!(failed_count)),
                ("skipped_tools".to_string(), serde_json::json!(skipped)),
                ("opencode_parallel_source".to_string(), serde_json::json!("packages/opencode/src/session/prompt/anthropic.txt:parallel tool calls policy")),
            ]),
        })
    }
}
