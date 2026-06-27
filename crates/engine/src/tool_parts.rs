//! OpenCode-style durable text, reasoning, snapshot, compaction, file, tool, and patch part helpers.
//!
//! Upstream references:
//! - `packages/schema/src/v1/session.ts`: session part schemas.
//! - `packages/opencode/src/session/processor.ts`: `completeToolCall` / `failToolCall`.
//! - `packages/opencode/src/session/compaction.ts`: creates compaction parts with auto/overflow/tail metadata.

use crate::types::{ToolKind, ToolRequest, ToolResult};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn text_part(text: &str, synthetic: bool) -> serde_json::Value {
    serde_json::json!({"type": "text", "text": text, "synthetic": synthetic, "time": {"start": 0, "end": 0}, "metadata": {"opencode_source": opencode_text_part_source()}})
}

pub fn text_parts(text: &str, synthetic: bool) -> Vec<serde_json::Value> {
    if text.trim().is_empty() { Vec::new() } else { vec![text_part(text, synthetic)] }
}

pub fn reasoning_part(text: &str) -> serde_json::Value {
    serde_json::json!({"type": "reasoning", "text": text, "time": {"start": 0, "end": 0}, "metadata": {"visibility": "public_progress_summary", "private_chain_of_thought": false, "opencode_source": opencode_reasoning_part_source()}})
}

pub fn reasoning_parts(public_text: &str) -> Vec<serde_json::Value> {
    let summary = public_progress_summary(public_text);
    if summary.is_empty() { Vec::new() } else { vec![reasoning_part(&summary)] }
}

fn public_progress_summary(public_text: &str) -> String {
    let compact = public_text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() { return compact; }
    let prefix = "Public progress summary: ";
    let limit = 180usize.saturating_sub(prefix.len());
    let visible = if compact.chars().count() > limit { format!("{}…", compact.chars().take(limit.saturating_sub(1)).collect::<String>()) } else { compact };
    format!("{prefix}{visible}")
}

pub fn snapshot_part(snapshot: &str) -> serde_json::Value {
    serde_json::json!({"type": "snapshot", "snapshot": snapshot, "metadata": {"opencode_source": opencode_snapshot_part_source()}})
}

pub fn compaction_part(auto: bool, overflow: Option<bool>, tail_start_id: Option<String>) -> serde_json::Value {
    let mut part = serde_json::json!({"type": "compaction", "auto": auto, "metadata": {"opencode_source": opencode_compaction_part_source()}});
    if let Some(overflow) = overflow { part["overflow"] = serde_json::json!(overflow); }
    if let Some(tail_start_id) = tail_start_id { part["tail_start_id"] = serde_json::json!(tail_start_id); }
    part
}

pub fn file_parts(results: &[ToolResult]) -> Vec<serde_json::Value> { results.iter().flat_map(file_parts_for_result).collect() }

fn file_parts_for_result(result: &ToolResult) -> Vec<serde_json::Value> {
    if !result.success || !matches!(result.kind, ToolKind::ApplyPatch) || !apply_patch_applied(result) { return Vec::new(); }
    patch_file_values(result).into_iter().map(|file| {
        let path = file.get("relativePath").or_else(|| file.get("path")).and_then(serde_json::Value::as_str).unwrap_or("unknown");
        let text = file.get("diff").and_then(serde_json::Value::as_str).unwrap_or("");
        serde_json::json!({
            "type": "file", "mime": mime_for(path), "filename": path.rsplit('/').next().unwrap_or(path),
            "url": format!("workspace://{}", path),
            "source": {"type": "file", "path": path, "text": {"value": text, "start": 0, "end": text.len()}},
            "metadata": {"opencode_source": opencode_file_part_source()}
        })
    }).collect()
}

fn apply_patch_applied(result: &ToolResult) -> bool {
    result.metadata.get("applied").and_then(serde_json::Value::as_bool).unwrap_or(true)
}

fn mime_for(path: &str) -> &'static str {
    if path.ends_with(".md") { "text/markdown" } else if path.ends_with(".json") { "application/json" } else if path.ends_with(".rs") { "text/rust" } else { "text/plain" }
}

pub fn running_tool_part(call: &ToolRequest) -> serde_json::Value {
    serde_json::json!({"type": "tool", "callID": call.id.0.to_string(), "tool": tool_name(&call.kind), "state": {"status": "running", "input": call.args.clone(), "metadata": {}, "time": {"start": 0}}, "metadata": {"opencode_source": opencode_tool_part_source()}})
}

pub fn finished_tool_part(result: &ToolResult) -> serde_json::Value { if result.success { completed_tool_part(result) } else { error_tool_part(result) } }

pub fn completed_tool_part(result: &ToolResult) -> serde_json::Value {
    let title = result.metadata.get("title").and_then(serde_json::Value::as_str).unwrap_or_else(|| tool_name(&result.kind));
    serde_json::json!({"type": "tool", "callID": result.id.0.to_string(), "tool": tool_name(&result.kind), "state": {"status": "completed", "input": {}, "output": result.output.clone(), "title": title, "metadata": result.metadata.clone(), "time": {"start": 0, "end": result.duration_ms}}, "metadata": {"opencode_source": opencode_tool_part_source()}})
}

pub fn error_tool_part(result: &ToolResult) -> serde_json::Value {
    serde_json::json!({"type": "tool", "callID": result.id.0.to_string(), "tool": tool_name(&result.kind), "state": {"status": "error", "input": {}, "error": result.error.clone().unwrap_or_else(|| result.output.clone()), "metadata": result.metadata.clone(), "time": {"start": 0, "end": result.duration_ms}}, "metadata": {"opencode_source": opencode_tool_part_source()}})
}

pub fn patch_part(result: &ToolResult) -> Option<serde_json::Value> {
    if !result.success || !matches!(result.kind, ToolKind::ApplyPatch) || !apply_patch_applied(result) { return None; }
    let files = patch_files(result);
    if files.is_empty() { return None; }
    let hash = patch_hash(result, &files);
    Some(serde_json::json!({"type": "patch", "hash": hash, "files": files, "metadata": {"opencode_source": opencode_patch_part_source()}}))
}

pub fn patch_parts(results: &[ToolResult]) -> Vec<serde_json::Value> { results.iter().filter_map(patch_part).collect() }

fn patch_file_values(result: &ToolResult) -> Vec<&serde_json::Value> {
    result.metadata.get("files").and_then(serde_json::Value::as_array).map(|files| files.iter().collect()).unwrap_or_default()
}

fn patch_files(result: &ToolResult) -> Vec<String> {
    let mut out = Vec::new();
    for file in patch_file_values(result) {
        if let Some(path) = file.get("relativePath").or_else(|| file.get("path")).and_then(serde_json::Value::as_str) { out.push(path.to_string()); }
    }
    out.sort(); out.dedup(); out
}

fn patch_hash(result: &ToolResult, files: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    result.id.0.to_string().hash(&mut hasher); result.output.hash(&mut hasher);
    for file in files { file.hash(&mut hasher); }
    format!("patch_{:016x}", hasher.finish())
}

pub fn opencode_text_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "TextPart", "shape": {"type": "text", "text": "String", "synthetic": "optional Boolean"}})
}

pub fn opencode_reasoning_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "ReasoningPart", "shape": {"type": "reasoning", "text": "String", "metadata": "optional Record<String, Any>", "time": {"start": "NonNegativeInt", "end": "optional NonNegativeInt"}}})
}

pub fn opencode_snapshot_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "SnapshotPart", "shape": {"type": "snapshot", "snapshot": "String"}})
}

pub fn opencode_compaction_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "CompactionPart", "shape": {"type": "compaction", "auto": "Boolean", "overflow": "optional Boolean", "tail_start_id": "optional String"}, "runtime_source": "packages/opencode/src/session/compaction.ts:create/process"})
}

pub fn opencode_file_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "FilePart", "shape": {"type": "file", "mime": "String", "filename": "optional String", "url": "String", "source": "optional FilePartSource"}})
}

pub fn opencode_patch_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "PatchPart", "shape": {"type": "patch", "hash": "String", "files": "Array<String>"}})
}

pub fn opencode_tool_part_source() -> serde_json::Value {
    serde_json::json!(["packages/schema/src/v1/session.ts:ToolPart/ToolStateRunning/ToolStateCompleted/ToolStateError", "packages/opencode/src/session/processor.ts:completeToolCall/failToolCall"])
}

pub fn tool_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read", ToolKind::FileWrite => "file_write", ToolKind::FileEdit => "file_edit",
        ToolKind::FileDelete => "file_delete", ToolKind::FileList => "file_list", ToolKind::FileGlob => "file_glob",
        ToolKind::FileSearch => "file_search", ToolKind::WebFetch => "web_fetch", ToolKind::WebSearch => "web_search",
        ToolKind::ShellCommand => "shell_command", ToolKind::TerminalRun => "terminal_run", ToolKind::Task => "task",
        ToolKind::BatchParallel => "batch_parallel", ToolKind::RepoInfo => "repo_info", ToolKind::ProposePatch => "propose_patch",
        ToolKind::ApplyPatch => "apply_patch", ToolKind::SwitchMode => "switch_mode", ToolKind::BrowserProof => "browser_proof",
        ToolKind::VisionReview => "vision_review", ToolKind::GraphBuild => "graph_build", ToolKind::GraphQuery => "graph_query",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ToolCallId;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn result_with_applied(applied: bool) -> ToolResult {
        ToolResult { id: ToolCallId(Uuid::nil()), kind: ToolKind::ApplyPatch, success: true, output: "Success".into(), error: None, duration_ms: 7,
            metadata: HashMap::from([("title".into(), serde_json::json!("apply_patch")), ("applied".into(), serde_json::json!(applied)), ("files".into(), serde_json::json!([{"relativePath": "proof.txt", "diff": "+ok"}]))]), }
    }

    fn result() -> ToolResult { result_with_applied(true) }

    #[test]
    fn builds_text_part() { assert_eq!(text_part("hello", false)["metadata"]["opencode_source"]["identifier"], "TextPart"); }

    #[test]
    fn builds_reasoning_part() {
        let part = reasoning_parts("Created a public summary for the turn.");
        assert_eq!(part[0]["type"], "reasoning");
        assert_eq!(part[0]["metadata"]["opencode_source"]["identifier"], "ReasoningPart");
        assert_eq!(part[0]["metadata"]["visibility"], "public_progress_summary");
    }

    #[test]
    fn builds_snapshot_part() { assert_eq!(snapshot_part("snapshot saved")["metadata"]["opencode_source"]["identifier"], "SnapshotPart"); }

    #[test]
    fn builds_compaction_part() {
        let part = compaction_part(true, Some(false), Some("message-index-2".to_string()));
        assert_eq!(part["type"], "compaction");
        assert_eq!(part["auto"], true);
        assert_eq!(part["tail_start_id"], "message-index-2");
        assert_eq!(part["metadata"]["opencode_source"]["identifier"], "CompactionPart");
    }

    #[test]
    fn builds_file_part_for_applied_apply_patch() {
        let parts = file_parts(&[result()]);
        assert_eq!(parts[0]["type"], "file");
        assert_eq!(parts[0]["filename"], "proof.txt");
        assert_eq!(parts[0]["metadata"]["opencode_source"]["identifier"], "FilePart");
    }

    #[test]
    fn skips_file_and_patch_parts_for_pending_approval() {
        assert!(file_parts(&[result_with_applied(false)]).is_empty());
        assert!(patch_parts(&[result_with_applied(false)]).is_empty());
    }

    #[test]
    fn builds_completed_tool_part() { assert_eq!(completed_tool_part(&result())["state"]["status"], "completed"); }

    #[test]
    fn builds_patch_part_for_apply_patch() {
        let part = patch_part(&result()).unwrap();
        assert_eq!(part["type"], "patch");
        assert!(part["hash"].as_str().unwrap().starts_with("patch_"));
    }
}
