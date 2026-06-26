//! OpenCode-style durable tool part helpers.
//!
//! Upstream reference:
//! - `packages/schema/src/v1/session.ts`: `ToolPart`, `ToolStateCompleted`, `ToolStateError`.
//! - `packages/opencode/src/session/processor.ts`: `completeToolCall` / `failToolCall`.

use crate::types::{ToolKind, ToolRequest, ToolResult};

pub fn running_tool_part(call: &ToolRequest) -> serde_json::Value {
    serde_json::json!({
        "type": "tool",
        "callID": call.id.0.to_string(),
        "tool": tool_name(&call.kind),
        "state": {
            "status": "running",
            "input": call.args,
            "metadata": {},
            "time": {"start": 0}
        },
        "metadata": {
            "opencode_source": opencode_tool_part_source()
        }
    })
}

pub fn finished_tool_part(result: &ToolResult) -> serde_json::Value {
    if result.success {
        completed_tool_part(result)
    } else {
        error_tool_part(result)
    }
}

pub fn completed_tool_part(result: &ToolResult) -> serde_json::Value {
    let title = result.metadata.get("title")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| tool_name(&result.kind));
    serde_json::json!({
        "type": "tool",
        "callID": result.id.0.to_string(),
        "tool": tool_name(&result.kind),
        "state": {
            "status": "completed",
            "input": {},
            "output": result.output,
            "title": title,
            "metadata": result.metadata,
            "time": {"start": 0, "end": result.duration_ms}
        },
        "metadata": {
            "opencode_source": opencode_tool_part_source()
        }
    })
}

pub fn error_tool_part(result: &ToolResult) -> serde_json::Value {
    serde_json::json!({
        "type": "tool",
        "callID": result.id.0.to_string(),
        "tool": tool_name(&result.kind),
        "state": {
            "status": "error",
            "input": {},
            "error": result.error.clone().unwrap_or_else(|| result.output.clone()),
            "metadata": result.metadata,
            "time": {"start": 0, "end": result.duration_ms}
        },
        "metadata": {
            "opencode_source": opencode_tool_part_source()
        }
    })
}

pub fn opencode_tool_part_source() -> serde_json::Value {
    serde_json::json!([
        "packages/schema/src/v1/session.ts:ToolPart/ToolStateRunning/ToolStateCompleted/ToolStateError",
        "packages/opencode/src/session/processor.ts:completeToolCall/failToolCall"
    ])
}

pub fn tool_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read",
        ToolKind::FileWrite => "file_write",
        ToolKind::FileEdit => "file_edit",
        ToolKind::FileDelete => "file_delete",
        ToolKind::FileList => "file_list",
        ToolKind::FileGlob => "file_glob",
        ToolKind::FileSearch => "file_search",
        ToolKind::WebFetch => "web_fetch",
        ToolKind::WebSearch => "web_search",
        ToolKind::ShellCommand => "shell_command",
        ToolKind::TerminalRun => "terminal_run",
        ToolKind::Task => "task",
        ToolKind::BatchParallel => "batch_parallel",
        ToolKind::RepoInfo => "repo_info",
        ToolKind::ProposePatch => "propose_patch",
        ToolKind::ApplyPatch => "apply_patch",
        ToolKind::SwitchMode => "switch_mode",
        ToolKind::BrowserProof => "browser_proof",
        ToolKind::VisionReview => "vision_review",
        ToolKind::GraphBuild => "graph_build",
        ToolKind::GraphQuery => "graph_query",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ToolCallId;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn builds_completed_tool_part() {
        let result = ToolResult {
            id: ToolCallId(Uuid::nil()),
            kind: ToolKind::ApplyPatch,
            success: true,
            output: "Success".into(),
            error: None,
            duration_ms: 7,
            metadata: HashMap::from([("title".into(), serde_json::json!("apply_patch"))]),
        };
        let part = completed_tool_part(&result);
        assert_eq!(part["type"], "tool");
        assert_eq!(part["tool"], "apply_patch");
        assert_eq!(part["state"]["status"], "completed");
        assert_eq!(part["state"]["output"], "Success");
        assert_eq!(part["state"]["title"], "apply_patch");
    }
}
