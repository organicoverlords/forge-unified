//! OpenCode-style durable text, reasoning, snapshot, compaction, file, tool, and patch part helpers.
//!
//! Upstream references:
//! - `packages/schema/src/v1/session.ts`: session part schemas.
//! - `packages/schema/src/v1/session.ts`: ToolStateCompleted carries optional FilePart attachments.
//! - `packages/opencode/src/session/processor.ts`: pending/running/completed/error tool part lifecycle.
//! - `packages/opencode/src/session/compaction.ts`: creates compaction parts with auto/overflow/tail metadata.

use crate::types::{ToolKind, ToolRequest, ToolResult};
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn text_part(text: &str, synthetic: bool) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({"type": "text", "text": text, "synthetic": synthetic, "time": {"start": 0, "end": 0}, "metadata": {"opencode_source": opencode_text_part_source()}}),
        format!("text:{synthetic}:{text}"),
    )
}

pub fn text_parts(text: &str, synthetic: bool) -> Vec<serde_json::Value> {
    if text.trim().is_empty() { Vec::new() } else { vec![text_part(text, synthetic)] }
}

pub fn reasoning_part(text: &str) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({"type": "reasoning", "text": text, "time": {"start": 0, "end": 0}, "metadata": {"visibility": "public_progress_summary", "private_chain_of_thought": false, "opencode_source": opencode_reasoning_part_source()}}),
        format!("reasoning:{text}"),
    )
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
    with_opencode_part_base(
        serde_json::json!({"type": "snapshot", "snapshot": snapshot, "metadata": {"opencode_source": opencode_snapshot_part_source()}}),
        format!("snapshot:{snapshot}"),
    )
}

pub fn compaction_part(auto: bool, overflow: Option<bool>, tail_start_id: Option<String>) -> serde_json::Value {
    let seed_tail = tail_start_id.clone().unwrap_or_else(|| "none".to_string());
    let mut part = serde_json::json!({"type": "compaction", "auto": auto, "metadata": {"opencode_source": opencode_compaction_part_source()}});
    if let Some(overflow) = overflow { part["overflow"] = serde_json::json!(overflow); }
    if let Some(tail_start_id) = tail_start_id { part["tail_start_id"] = serde_json::json!(tail_start_id); }
    with_opencode_part_base(part, format!("compaction:{auto}:{seed_tail}"))
}

pub fn file_parts(results: &[ToolResult]) -> Vec<serde_json::Value> { results.iter().flat_map(file_parts_for_result).collect() }

fn file_parts_for_result(result: &ToolResult) -> Vec<serde_json::Value> {
    if !result.success || !result_can_attach_files(result) { return Vec::new(); }
    file_values(result).into_iter().map(|file| {
        let path = file.get("relativePath").or_else(|| file.get("path")).and_then(serde_json::Value::as_str).unwrap_or("unknown");
        let text = file.get("diff").and_then(serde_json::Value::as_str).unwrap_or_else(|| result.output.as_str());
        let kind = file.get("type").and_then(serde_json::Value::as_str).unwrap_or("update");
        with_opencode_part_base(
            serde_json::json!({
                "type": "file", "mime": mime_for(path), "filename": path.rsplit('/').next().unwrap_or(path),
                "url": format!("workspace://{}", path),
                "source": {"type": "file", "path": path, "text": {"value": text, "start": 0, "end": text.len()}},
                "metadata": {"opencode_source": opencode_file_part_source(), "tool": tool_name(&result.kind), "change_type": kind}
            }),
            format!("file:{}:{}:{}", result.id.clone().0, kind, path),
        )
    }).collect()
}

fn result_can_attach_files(result: &ToolResult) -> bool {
    match &result.kind {
        ToolKind::ApplyPatch => apply_patch_applied(result),
        ToolKind::FileWrite | ToolKind::FileEdit | ToolKind::FileDelete => true,
        _ => false,
    }
}

fn apply_patch_applied(result: &ToolResult) -> bool {
    result.metadata.get("applied").and_then(serde_json::Value::as_bool).unwrap_or(true)
}

fn file_values(result: &ToolResult) -> Vec<&serde_json::Value> {
    result.metadata.get("files").and_then(serde_json::Value::as_array).map(|files| files.iter().collect()).unwrap_or_default()
}

fn mime_for(path: &str) -> &'static str {
    if path.ends_with(".md") { "text/markdown" } else if path.ends_with(".json") { "application/json" } else if path.ends_with(".rs") { "text/rust" } else { "text/plain" }
}

pub fn pending_tool_part(call: &ToolRequest) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": call.id.clone().0.to_string(), "tool": tool_name(&call.kind),
            "state": {"status": "pending", "input": {}, "raw": call.args.to_string()},
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "pending"}
        }),
        tool_part_seed(&call.id.clone().0.to_string(), tool_name(&call.kind), "pending"),
    )
}

pub fn running_tool_part(call: &ToolRequest) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": call.id.clone().0.to_string(), "tool": tool_name(&call.kind),
            "state": {"status": "running", "input": call.args.clone(), "title": tool_name(&call.kind), "metadata": {}, "time": {"start": 0}},
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "running"}
        }),
        tool_part_seed(&call.id.clone().0.to_string(), tool_name(&call.kind), "running"),
    )
}

pub fn started_tool_lifecycle_parts(call: &ToolRequest) -> Vec<serde_json::Value> {
    vec![pending_tool_part(call), running_tool_part(call)]
}

pub fn finished_tool_part(result: &ToolResult) -> serde_json::Value { if result.success { completed_tool_part(result) } else { error_tool_part(result) } }

pub fn finished_tool_lifecycle_parts(result: &ToolResult) -> Vec<serde_json::Value> {
    vec![pending_tool_part_from_result(result), running_tool_part_from_result(result), finished_tool_part(result)]
}

pub fn completed_tool_part(result: &ToolResult) -> serde_json::Value {
    let title = result.metadata.get("title").and_then(serde_json::Value::as_str).unwrap_or_else(|| tool_name(&result.kind));
    let mut state = serde_json::json!({"status": "completed", "input": tool_input(result), "output": result.output.clone(), "title": title, "metadata": result.metadata.clone(), "time": {"start": 0, "end": result.duration_ms}});
    let attachments = file_parts_for_result(result);
    if !attachments.is_empty() { state["attachments"] = serde_json::json!(attachments); }
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": result.id.clone().0.to_string(), "tool": tool_name(&result.kind),
            "state": state,
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "completed"}
        }),
        tool_part_seed(&result.id.clone().0.to_string(), tool_name(&result.kind), "completed"),
    )
}

pub fn error_tool_part(result: &ToolResult) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": result.id.clone().0.to_string(), "tool": tool_name(&result.kind),
            "state": {"status": "error", "input": tool_input(result), "error": result.error.clone().unwrap_or_else(|| result.output.clone()), "metadata": result.metadata.clone(), "time": {"start": 0, "end": result.duration_ms}},
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "error"}
        }),
        tool_part_seed(&result.id.clone().0.to_string(), tool_name(&result.kind), "error"),
    )
}

fn pending_tool_part_from_result(result: &ToolResult) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": result.id.clone().0.to_string(), "tool": tool_name(&result.kind),
            "state": {"status": "pending", "input": {}, "raw": tool_input(result).to_string()},
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "pending", "derived_from_result": true}
        }),
        tool_part_seed(&result.id.clone().0.to_string(), tool_name(&result.kind), "pending"),
    )
}

fn running_tool_part_from_result(result: &ToolResult) -> serde_json::Value {
    with_opencode_part_base(
        serde_json::json!({
            "type": "tool", "callID": result.id.clone().0.to_string(), "tool": tool_name(&result.kind),
            "state": {"status": "running", "input": tool_input(result), "title": tool_name(&result.kind), "metadata": {}, "time": {"start": 0}},
            "metadata": {"opencode_source": opencode_tool_part_source(), "lifecycle_stage": "running", "derived_from_result": true}
        }),
        tool_part_seed(&result.id.clone().0.to_string(), tool_name(&result.kind), "running"),
    )
}

fn tool_input(result: &ToolResult) -> serde_json::Value {
    result.metadata.get("opencode_tool_input").cloned().unwrap_or_else(|| serde_json::json!({}))
}

pub fn patch_part(result: &ToolResult) -> Option<serde_json::Value> {
    if !result.success || !matches!(&result.kind, ToolKind::ApplyPatch) || !apply_patch_applied(result) { return None; }
    let files = patch_files(result);
    if files.is_empty() { return None; }
    let hash = patch_hash(result, &files);
    Some(with_opencode_part_base(
        serde_json::json!({"type": "patch", "hash": hash, "files": files, "metadata": {"opencode_source": opencode_patch_part_source()}}),
        format!("patch:{}", result.id.clone().0),
    ))
}

pub fn patch_parts(results: &[ToolResult]) -> Vec<serde_json::Value> { results.iter().filter_map(patch_part).collect() }

fn patch_files(result: &ToolResult) -> Vec<String> {
    let mut out = Vec::new();
    for file in file_values(result) {
        if let Some(path) = file.get("relativePath").or_else(|| file.get("path")).and_then(serde_json::Value::as_str) { out.push(path.to_string()); }
    }
    out.sort(); out.dedup(); out
}

fn patch_hash(result: &ToolResult, files: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    result.id.clone().0.to_string().hash(&mut hasher); result.output.hash(&mut hasher);
    for file in files { file.hash(&mut hasher); }
    format!("patch_{:016x}", hasher.finish())
}

fn tool_part_seed(call_id: &str, tool: &str, stage: &str) -> String { format!("tool:{call_id}:{tool}:{stage}") }

fn with_opencode_part_base(mut part: Value, seed: impl AsRef<str>) -> Value {
    let seed = seed.as_ref();
    part["id"] = serde_json::json!(durable_schema_id("prt", seed));
    part["sessionID"] = serde_json::json!(durable_schema_id("ses", "forge-unified-session"));
    part["messageID"] = serde_json::json!(durable_schema_id("msg", seed));
    let source = serde_json::json!({
        "schema_path": "packages/schema/src/v1/session.ts",
        "copied_fields": ["id", "sessionID", "messageID"],
        "part_base_lines": "partBase requires PartID, SessionID, and MessageID on session parts"
    });
    if let Some(metadata) = part.get_mut("metadata").and_then(Value::as_object_mut) {
        metadata.insert("opencode_part_base_source".to_string(), source);
    } else {
        part["metadata"] = serde_json::json!({"opencode_part_base_source": source});
    }
    part
}

fn durable_schema_id(prefix: &str, seed: &str) -> String {
    let mut hasher = DefaultHasher::new();
    prefix.hash(&mut hasher);
    seed.hash(&mut hasher);
    format!("{prefix}_forge_{:016x}", hasher.finish())
}

pub fn opencode_text_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "TextPart", "shape": {"type": "text", "text": "String", "synthetic": "optional Boolean", "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}})
}

pub fn opencode_reasoning_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "ReasoningPart", "shape": {"type": "reasoning", "text": "String", "metadata": "optional Record<String, Any>", "time": {"start": "NonNegativeInt", "end": "optional NonNegativeInt"}, "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}})
}

pub fn opencode_snapshot_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "SnapshotPart", "shape": {"type": "snapshot", "snapshot": "String", "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}})
}

pub fn opencode_compaction_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "CompactionPart", "shape": {"type": "compaction", "auto": "Boolean", "overflow": "optional Boolean", "tail_start_id": "optional String", "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}, "runtime_source": "packages/opencode/src/session/compaction.ts:create/process"})
}

pub fn opencode_file_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "FilePart", "shape": {"type": "file", "mime": "String", "filename": "optional String", "url": "String", "source": "optional FilePartSource", "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}})
}

pub fn opencode_patch_part_source() -> serde_json::Value {
    serde_json::json!({"path": "packages/schema/src/v1/session.ts", "identifier": "PatchPart", "shape": {"type": "patch", "hash": "String", "files": "Array<String>", "id": "PartID", "sessionID": "SessionID", "messageID": "MessageID"}})
}

pub fn opencode_tool_part_source() -> serde_json::Value {
    serde_json::json!({
        "schema": "packages/schema/src/v1/session.ts:partBase + ToolPart/ToolStatePending/ToolStateRunning/ToolStateCompleted/ToolStateError",
        "attachments_schema": "packages/schema/src/v1/session.ts:ToolStateCompleted.attachments -> Array<FilePart>",
        "processor": "packages/opencode/src/session/processor.ts:ensureToolCall/updateToolCall/completeToolCall/failToolCall"
    })
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
            metadata: HashMap::from([("title".into(), serde_json::json!("apply_patch")), ("applied".into(), serde_json::json!(applied)), ("opencode_tool_input".into(), serde_json::json!({"patchText": "*** Begin Patch"})), ("files".into(), serde_json::json!([{"relativePath": "proof.txt", "diff": "+ok"}]))]), }
    }

    fn result() -> ToolResult { result_with_applied(true) }

    fn file_write_result() -> ToolResult {
        ToolResult { id: ToolCallId(Uuid::nil()), kind: ToolKind::FileWrite, success: true, output: "Written 12 bytes to proof.txt".into(), error: None, duration_ms: 5,
            metadata: HashMap::from([("title".into(), serde_json::json!("file_write")), ("files".into(), serde_json::json!([{"type":"add", "relativePath": "proof.txt", "path":"proof.txt"}]))]), }
    }

    fn assert_part_base(part: &serde_json::Value) {
        assert!(part["id"].as_str().unwrap().starts_with("prt_"));
        assert!(part["sessionID"].as_str().unwrap().starts_with("ses_"));
        assert!(part["messageID"].as_str().unwrap().starts_with("msg_"));
        assert_eq!(part["metadata"]["opencode_part_base_source"]["schema_path"], "packages/schema/src/v1/session.ts");
    }

    #[test]
    fn builds_text_part() {
        let part = text_part("hello", false);
        assert_eq!(part["metadata"]["opencode_source"]["identifier"], "TextPart");
        assert_part_base(&part);
    }

    #[test]
    fn builds_reasoning_part() {
        let part = reasoning_parts("Created a public summary for the turn.");
        assert_eq!(part[0]["type"], "reasoning");
        assert_eq!(part[0]["metadata"]["opencode_source"]["identifier"], "ReasoningPart");
        assert_eq!(part[0]["metadata"]["visibility"], "public_progress_summary");
        assert_part_base(&part[0]);
    }

    #[test]
    fn builds_snapshot_part() {
        let part = snapshot_part("snapshot saved");
        assert_eq!(part["metadata"]["opencode_source"]["identifier"], "SnapshotPart");
        assert_part_base(&part);
    }

    #[test]
    fn builds_compaction_part() {
        let part = compaction_part(true, Some(false), Some("message-index-2".to_string()));
        assert_eq!(part["type"], "compaction");
        assert_eq!(part["auto"], true);
        assert_eq!(part["tail_start_id"], "message-index-2");
        assert_eq!(part["metadata"]["opencode_source"]["identifier"], "CompactionPart");
        assert_part_base(&part);
    }

    #[test]
    fn builds_file_part_for_applied_apply_patch() {
        let parts = file_parts(&[result()]);
        assert_eq!(parts[0]["type"], "file");
        assert_eq!(parts[0]["filename"], "proof.txt");
        assert_eq!(parts[0]["metadata"]["opencode_source"]["identifier"], "FilePart");
        assert_part_base(&parts[0]);
    }

    #[test]
    fn builds_file_part_for_normal_file_tools() {
        let parts = file_parts(&[file_write_result()]);
        assert_eq!(parts[0]["type"], "file");
        assert_eq!(parts[0]["metadata"]["tool"], "file_write");
        assert_eq!(parts[0]["metadata"]["change_type"], "add");
        assert_part_base(&parts[0]);
    }

    #[test]
    fn completed_file_tool_part_carries_file_attachments() {
        let part = completed_tool_part(&file_write_result());
        assert_eq!(part["state"]["status"], "completed");
        assert_eq!(part["state"]["attachments"][0]["filename"], "proof.txt");
        assert!(part["metadata"]["opencode_source"]["attachments_schema"].as_str().unwrap().contains("ToolStateCompleted.attachments"));
        assert_part_base(&part);
        assert_part_base(&part["state"]["attachments"][0]);
    }

    #[test]
    fn skips_file_and_patch_parts_for_pending_approval() {
        assert!(file_parts(&[result_with_applied(false)]).is_empty());
        assert!(patch_parts(&[result_with_applied(false)]).is_empty());
    }

    #[test]
    fn builds_full_tool_lifecycle_parts() {
        let parts = finished_tool_lifecycle_parts(&result());
        let statuses: Vec<_> = parts.iter().map(|p| p["state"]["status"].as_str().unwrap()).collect();
        assert_eq!(statuses, vec!["pending", "running", "completed"]);
        assert_eq!(parts[2]["state"]["input"]["patchText"], "*** Begin Patch");
        assert!(parts[2]["metadata"]["opencode_source"]["schema"].as_str().unwrap().contains("ToolStatePending"));
        for part in parts { assert_part_base(&part); }
    }

    #[test]
    fn builds_patch_part_for_apply_patch() {
        let part = patch_part(&result()).unwrap();
        assert_eq!(part["type"], "patch");
        assert!(part["hash"].as_str().unwrap().starts_with("patch_"));
        assert_part_base(&part);
    }

    #[test]
    fn durable_part_base_is_stable_for_same_seed() {
        let first = text_part("same", false);
        let second = text_part("same", false);
        assert_eq!(first["id"], second["id"]);
        assert_eq!(first["sessionID"], second["sessionID"]);
        assert_eq!(first["messageID"], second["messageID"]);
    }
}