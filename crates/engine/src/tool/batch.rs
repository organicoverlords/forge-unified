//! Batch parallel tool execution.

use crate::tool::ToolExecutor;
use crate::types::{ToolCallId, ToolKind, ToolRequest, ToolResult};
use anyhow::Result;
use serde_json::{Map, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
struct NormalizedBatchCall {
    tool: String,
    args: Value,
}

impl ToolExecutor {
    pub async fn execute_batch_parallel(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args {
            calls: Option<Vec<Value>>,
            requests: Option<Vec<Value>>,
        }

        let args: Args = serde_json::from_value(request.args)?;
        let calls = args.calls.or(args.requests).unwrap_or_default();

        let mut sub_requests = Vec::new();
        let mut skipped = Vec::new();
        for raw_call in calls {
            let call = match normalize_batch_call(raw_call) {
                Some(call) => call,
                None => {
                    skipped.push("invalid_batch_call".to_string());
                    continue;
                }
            };
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
                _ => {
                    skipped.push(call.tool);
                    continue;
                }
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
                ("forge_parallel_policy".to_string(), serde_json::json!("independent tool requests are executed concurrently up to the configured parallelism limit")),
            ]),
        })
    }
}

fn normalize_batch_call(raw: Value) -> Option<NormalizedBatchCall> {
    let object = raw.as_object()?.clone();
    if let Some(tool) = object.get("tool").and_then(Value::as_str) {
        let args = object.get("args").cloned().unwrap_or_else(|| args_without_tool_fields(&object));
        return Some(NormalizedBatchCall { tool: tool.to_string(), args });
    }

    if object.len() == 1 {
        let (tool, args) = object.into_iter().next()?;
        return Some(NormalizedBatchCall { tool, args });
    }

    None
}

fn args_without_tool_fields(object: &Map<String, Value>) -> Value {
    let mut args = Map::new();
    for (key, value) in object {
        if key != "tool" && key != "args" {
            args.insert(key.clone(), value.clone());
        }
    }
    Value::Object(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_explicit_tool_args_shape() {
        let call = normalize_batch_call(serde_json::json!({"tool":"file_read","args":{"path":"Cargo.toml"}})).unwrap();
        assert_eq!(call.tool, "file_read");
        assert_eq!(call.args["path"], "Cargo.toml");
    }

    #[test]
    fn normalizes_explicit_tool_inline_args_shape() {
        let call = normalize_batch_call(serde_json::json!({"tool":"shell_command","command":"git status -sb"})).unwrap();
        assert_eq!(call.tool, "shell_command");
        assert_eq!(call.args["command"], "git status -sb");
    }

    #[test]
    fn normalizes_single_key_shorthand_shape() {
        let call = normalize_batch_call(serde_json::json!({"repo_info":{}})).unwrap();
        assert_eq!(call.tool, "repo_info");
        assert_eq!(call.args, serde_json::json!({}));
    }
}
