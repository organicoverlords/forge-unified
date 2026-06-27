//! OpenCode-style durable tool and patch part helpers.
//!
//! Upstream references:
//! - `packages/schema/src/v1/session.ts`: `PatchPart`, `ToolPart`, `ToolState*`.
//! - `packages/opencode/src/session/processor.ts`: `completeToolCall` / `failToolCall`.

use crate::types::{ToolKind, ToolRequest, ToolResult};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn running_tool_part(call: &ToolRequest) -> serde_json::Value {
    serde_json::json!({
        "type": "tool",
        "callID": call.id.0.to_string(),
        "tool": tool_name(&call.kind),
        "state": {
            "status": "running",
            "input": call.args.clone(),
            "metadata": {},
            "time": {"start": 0}
        },
        "metadata": {"opencode_source": opencode_tool_part_source()}
    })
}

pub fn finished_tool_part(result: &ToolResult) -> serde_json::Value {
    if result.success { completed_tool_part(result) } else { error_tool_part(result) }
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
            "output": result.output.clone(),
            "title": title,
            "metadata": result.metadata.clone(),
            "time": {"start": 0, "end": result.duration_ms}
        },
        "metadata": {"opencode_source": opencode_tool_part_source()}
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
            "metadata": result.metadata.clone(),
            "time": {"start": 0, "end": result.duration_ms}
        },
        "metadata": {"opencode_source": opencode_tool_part_source()}
    })
}

pub fn patch_part(result: &ToolResult) -> Option<serde_json::Value> {
    if !result.success || !matches!(result.kind, ToolKind::ApplyPatch) { return None; }
    let files = patch_files(result);
    if files.is_empty() { return None; }
    let hash = patch_hash(result, &files);
    Some(serde_json::json!({
        "type": "patch",
        "hash": hash,
        "files": files,
        "metadata": {"opencode_source": opencode_patch_part_source()}
    }))
}

pub fn patch_parts(results: &[ToolResult]) -> Vec<serde_json::Value> {
    results.iter().filter_map(patch_part).collect()
}

fn patch_files(result: &ToolResult) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(files) = result.metadata.get("files").and_then(serde_json::Value::as_array) {
        for file in files {
            let path = file.get("relativePath").or_else(|| file.get("path"))
                .and_then(serde_json::Value::as_str);
            if let Some(path) = path { out.push(path.to_string()); }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn patch_hash(result: &ToolResult, files: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    result.id.0.to_string().hash(&mut hasher);
    result.output.hash(&mut hasher);
    for file in files { file.hash(&mut hasher); }
    format!("patch_{:016x}", hasher.finish())
}

pub fn opencode_patch_part_source() -> serde_json::Value {
    serde_json::json!({
        "path": "packages/schema/src/v1/session.ts",
        "identifier": "PatchPart",
        "shape": {"type": "patch", "hash": "String", "files": "Array<String>"}
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

    fn result() -> ToolResult {
        ToolResult {
            id: ToolCallId(Uuid::nil()), kind: ToolKind::ApplyPatch, success: true,
            output: "Success".into(), error: None, duration_ms: 7,
            metadata: HashMap::from([
                ("title".into(), serde_json::json!("apply_patch")),
                ("files".into(), serde_json::json!([{"relativePath": "proof.txt"}])),
            ]),
        }
    }

    #[test]
    fn builds_completed_tool_part() {
        let part = completed_tool_part(&result());
        assert_eq!(part["type"], "tool");
        assert_eq!(part["tool"], "apply_patch");
        assert_eq!(part["state"]["status"], "completed");
        assert_eq!(part["state"]["title"], "apply_patch");
    }

    #[test]
    fn builds_patch_part_for_apply_patch() {
        let part = patch_part(&result()).unwrap();
        assert_eq!(part["type"], "patch");
        assert!(part["hash"].as_str().unwrap().starts_with("patch_"));
        assert_eq!(part["files"][0], "proof.txt");
        assert_eq!(part["metadata"]["opencode_source"]["identifier"], "PatchPart");
    }
}
