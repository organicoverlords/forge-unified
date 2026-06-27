//! OpenCode-compatible apply_patch parser, edit approval, and mutation entrypoint.

use crate::tool::patch_apply::{apply_file_changes, file_change_metadata, file_change_summary_line, prepare_file_changes, total_diff};
use crate::tool::patch_events;
use crate::tool::ToolExecutor;
use crate::types::{ToolCallId, ToolKind, ToolRequest, ToolResult};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Component, Path};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub(crate) enum PatchHunk {
    Add { path: String, contents: String },
    Delete { path: String },
    Update { path: String, move_path: Option<String>, chunks: Vec<UpdateChunk> },
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct UpdateChunk {
    pub(crate) old_lines: Vec<String>,
    pub(crate) new_lines: Vec<String>,
    pub(crate) change_context: Option<String>,
    pub(crate) is_end_of_file: bool,
}

impl ToolExecutor {
    pub async fn execute_apply_patch(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        #[allow(non_snake_case)]
        struct Args { patchText: String, approved: Option<bool>, approval_id: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        let patch_text = args.patchText;
        let patch_len = patch_text.len();
        let approved = args.approved.unwrap_or(false);
        let approval_id = args.approval_id.unwrap_or_else(|| request.id.0.to_string());
        if patch_text.trim().is_empty() { anyhow::bail!("patchText cannot be empty"); }

        let hunks = match parse_opencode_patch(&patch_text) {
            Ok(hunks) => hunks,
            Err(err) => return Ok(apply_patch_failure(request.id, patch_len, err.to_string())),
        };
        if hunks.is_empty() {
            let normalized = patch_text.replace("\r\n", "\n").replace('\r', "\n");
            let empty_doc = [patch_marker("Begin Patch"), patch_marker("End Patch")].join("\n");
            let error = if normalized.trim() == empty_doc { "patch rejected: empty patch" } else { "apply_patch verification failed: no hunks found" };
            return Ok(apply_patch_failure(request.id, patch_len, error));
        }

        let validated_paths = match validate_patch_paths(&hunks, &self.workspace_root) {
            Ok(paths) => paths,
            Err(err) => return Ok(apply_patch_failure(request.id, patch_len, err.to_string())),
        };
        let changes = match prepare_file_changes(&hunks, &self.workspace_root).await {
            Ok(changes) => changes,
            Err(err) => return Ok(apply_patch_failure(request.id, patch_len, err.to_string())),
        };

        let files: Vec<_> = changes.iter().map(file_change_metadata).collect();
        let summary_lines = changes.iter().map(file_change_summary_line).collect::<Vec<_>>();
        let diff = total_diff(&changes);
        let permission_request = edit_permission_request(&hunks, &files, diff.clone(), approved, &approval_id);

        if !approved {
            return Ok(apply_patch_pending(request.id, patch_len, hunks.len(), files, summary_lines, validated_paths, permission_request, patch_text, diff, approval_id));
        }

        if let Err(err) = apply_file_changes(&changes).await {
            return Ok(apply_patch_failure(request.id, patch_len, err.to_string()));
        }

        let file_events = patch_events::file_change_events(&files);
        let watcher_updates = patch_events::watcher_updates(&files);
        let filesystem_edits = patch_events::filesystem_edits(&files);
        let lsp_touches = patch_events::lsp_touches(&files);
        let lsp_warmups = patch_events::lsp_warmups(&files);
        let diagnostics = patch_events::diagnostics_metadata(&files);
        let diagnostic_reports = patch_events::diagnostic_reports(&files);
        let output = format!("Success. Updated the following files:\n{}\n{}", change_count_summary(summary_lines.len()), summary_lines.join("\n"));
        Ok(ToolResult {
            id: request.id, kind: ToolKind::ApplyPatch, success: true, output, error: None, duration_ms: 0,
            metadata: HashMap::from([
                ("title".to_string(), serde_json::json!("apply_patch")),
                ("opencode_source".to_string(), opencode_source()),
                ("opencode_permission_source".to_string(), opencode_permission_source()),
                ("opencode_tool_state_source".to_string(), opencode_tool_state_source()),
                ("opencode_event_source".to_string(), patch_events::opencode_event_source()),
                ("patch_length".to_string(), serde_json::json!(patch_len)),
                ("hunk_count".to_string(), serde_json::json!(hunks.len())),
                ("files".to_string(), serde_json::json!(files)),
                ("file_events".to_string(), serde_json::json!(file_events)),
                ("opencode_watcher_updates".to_string(), serde_json::json!(watcher_updates)),
                ("opencode_filesystem_edits".to_string(), serde_json::json!(filesystem_edits)),
                ("opencode_lsp_warmups".to_string(), serde_json::json!(lsp_warmups)),
                ("opencode_lsp_diagnostics".to_string(), serde_json::json!(diagnostic_reports)),
                ("lsp_touches".to_string(), serde_json::json!(lsp_touches)),
                ("summary_lines".to_string(), serde_json::json!(summary_lines)),
                ("validated_paths".to_string(), serde_json::json!(validated_paths)),
                ("permission".to_string(), permission_request.clone()),
                ("permission_request".to_string(), permission_request),
                ("approval_state".to_string(), serde_json::json!({"status":"approved", "approval_id": approval_id, "required_before_apply": true})),
                ("parsed_hunks".to_string(), serde_json::json!(hunks)),
                ("diff".to_string(), serde_json::json!(diff)),
                ("diagnostics".to_string(), diagnostics),
                ("applied".to_string(), serde_json::json!(true)),
            ]),
        })
    }
}

fn change_count_summary(count: usize) -> String {
    if count == 1 { "Updated 1 file".to_string() } else { format!("Updated {count} files") }
}

#[allow(clippy::too_many_arguments)]
fn apply_patch_pending(id: ToolCallId, patch_length: usize, hunk_count: usize, files: Vec<serde_json::Value>, summary_lines: Vec<String>, validated_paths: Vec<serde_json::Value>, permission_request: serde_json::Value, patch_text: String, diff: String, approval_id: String) -> ToolResult {
    ToolResult {
        id, kind: ToolKind::ApplyPatch, success: true,
        output: format!("Edit approval required before applying patch.\nPending files:\n{}", summary_lines.join("\n")),
        error: None, duration_ms: 0,
        metadata: HashMap::from([
            ("title".to_string(), serde_json::json!("apply_patch approval pending")),
            ("opencode_source".to_string(), opencode_source()),
            ("opencode_permission_source".to_string(), opencode_permission_source()),
            ("opencode_tool_state_source".to_string(), opencode_tool_state_source()),
            ("patch_length".to_string(), serde_json::json!(patch_length)),
            ("hunk_count".to_string(), serde_json::json!(hunk_count)),
            ("files".to_string(), serde_json::json!(files)),
            ("summary_lines".to_string(), serde_json::json!(summary_lines)),
            ("validated_paths".to_string(), serde_json::json!(validated_paths)),
            ("permission".to_string(), permission_request.clone()),
            ("permission_request".to_string(), permission_request),
            ("pending_edit_approval".to_string(), serde_json::json!({"approval_id": approval_id.clone(), "status":"pending", "patchText": patch_text, "diff": diff})),
            ("approval_state".to_string(), serde_json::json!({"status":"pending", "approval_id": approval_id, "required_before_apply": true})),
            ("applied".to_string(), serde_json::json!(false)),
        ]),
    }
}

fn apply_patch_failure(id: ToolCallId, patch_length: usize, error: impl Into<String>) -> ToolResult {
    ToolResult {
        id, kind: ToolKind::ApplyPatch, success: false, output: "Patch validation failed".to_string(),
        error: Some(error.into()), duration_ms: 0,
        metadata: HashMap::from([
            ("title".to_string(), serde_json::json!("apply_patch")),
            ("opencode_source".to_string(), opencode_source()),
            ("opencode_permission_source".to_string(), opencode_permission_source()),
            ("opencode_tool_state_source".to_string(), opencode_tool_state_source()),
            ("patch_length".to_string(), serde_json::json!(patch_length)),
            ("applied".to_string(), serde_json::json!(false)),
        ]),
    }
}

fn opencode_source() -> serde_json::Value {
    serde_json::json!(["packages/opencode/src/tool/apply_patch.ts", "packages/opencode/src/tool/read.ts", "packages/opencode/src/patch/index.ts"])
}

fn opencode_permission_source() -> serde_json::Value {
    serde_json::json!({"path":"packages/opencode/src/tool/apply_patch.ts", "behavior":"ctx.ask edit permission with patterns, always, and metadata.filepath/diff/files before applying changes"})
}

fn opencode_tool_state_source() -> serde_json::Value {
    serde_json::json!(["packages/schema/src/v1/session.ts:ToolStateCompleted/ToolStateError", "packages/opencode/src/session/processor.ts:toolResultOutput/completeToolCall/failToolCall"])
}

fn edit_permission_request(hunks: &[PatchHunk], files: &[serde_json::Value], diff: String, approved: bool, approval_id: &str) -> serde_json::Value {
    let patterns = patch_relative_paths(hunks);
    let filepath = patterns.join(", ");
    serde_json::json!({
        "permission":"edit", "required":"edit", "patterns": patterns, "always":["*"],
        "metadata_ready": true, "interactive": true, "approved": approved,
        "approval_id": approval_id, "status": if approved { "approved" } else { "pending" },
        "metadata": {"filepath": filepath, "diff": diff, "files": files},
        "note": if approved { "Forge applied this patch only after edit approval." } else { "OpenCode asks before applying; Forge now pauses apply_patch until this edit request is approved." }
    })
}

fn parse_opencode_patch(patch_text: &str) -> Result<Vec<PatchHunk>> {
    let cleaned = strip_heredoc(patch_text.trim());
    let lines: Vec<&str> = cleaned.lines().collect();
    let begin_marker = patch_marker("Begin Patch");
    let end_marker = patch_marker("End Patch");
    let begin_idx = lines.iter().position(|line| line.trim() == begin_marker);
    let end_idx = lines.iter().position(|line| line.trim() == end_marker);
    let (Some(begin), Some(end)) = (begin_idx, end_idx) else { return Err(anyhow!("Invalid patch format: missing Begin/End markers")); };
    if begin >= end { return Err(anyhow!("Invalid patch format: missing Begin/End markers")); }

    let add_marker = patch_marker("Add File:");
    let delete_marker = patch_marker("Delete File:");
    let update_marker = patch_marker("Update File:");
    let move_marker = patch_marker("Move to:");
    let mut hunks = Vec::new();
    let mut i = begin + 1;
    while i < end {
        let line = lines[i];
        if let Some(path) = line.strip_prefix(add_marker.as_str()).map(str::trim).filter(|value| !value.is_empty()) {
            let (contents, next_idx) = parse_add_file_content(&lines, i + 1, end);
            hunks.push(PatchHunk::Add { path: path.to_string(), contents });
            i = next_idx; continue;
        }
        if let Some(path) = line.strip_prefix(delete_marker.as_str()).map(str::trim).filter(|value| !value.is_empty()) {
            hunks.push(PatchHunk::Delete { path: path.to_string() });
            i += 1; continue;
        }
        if let Some(path) = line.strip_prefix(update_marker.as_str()).map(str::trim).filter(|value| !value.is_empty()) {
            let mut next_idx = i + 1;
            let mut move_path = None;
            if next_idx < end {
                if let Some(target) = lines[next_idx].strip_prefix(move_marker.as_str()).map(str::trim).filter(|value| !value.is_empty()) {
                    move_path = Some(target.to_string()); next_idx += 1;
                }
            }
            let (chunks, after_chunks) = parse_update_chunks(&lines, next_idx, end);
            hunks.push(PatchHunk::Update { path: path.to_string(), move_path, chunks });
            i = after_chunks; continue;
        }
        i += 1;
    }
    Ok(hunks)
}

fn patch_marker(label: &str) -> String { ["*** ", label].concat() }

fn strip_heredoc(input: &str) -> String {
    let mut lines = input.lines();
    let Some(first) = lines.next() else { return input.to_string(); };
    let prefix = ["cat ", "<<"].concat();
    let marker = first.trim().strip_prefix(prefix.as_str()).or_else(|| first.trim().strip_prefix("<<"))
        .map(|value| value.trim_matches('\'').trim_matches('"').trim().to_string());
    let Some(marker) = marker.filter(|value| !value.is_empty()) else { return input.to_string(); };
    let body: Vec<&str> = lines.collect();
    if body.last().map(|line| line.trim()) == Some(marker.as_str()) { body[..body.len() - 1].join("\n") } else { input.to_string() }
}

fn parse_add_file_content(lines: &[&str], start: usize, end: usize) -> (String, usize) {
    let mut content = Vec::new();
    let mut i = start;
    while i < end && !lines[i].starts_with("***") {
        let raw = lines[i];
        content.push(raw.strip_prefix('+').unwrap_or(raw));
        i += 1;
    }
    (content.join("\n"), i)
}

fn parse_update_chunks(lines: &[&str], start: usize, end: usize) -> (Vec<UpdateChunk>, usize) {
    let mut chunks = Vec::new();
    let mut old_lines = Vec::new();
    let mut new_lines = Vec::new();
    let mut context = None;
    let mut i = start;
    while i < end && !lines[i].starts_with("***") {
        let line = lines[i];
        match line.chars().next() {
            Some('@') => { flush_update_chunk(&mut chunks, &mut old_lines, &mut new_lines, &mut context, false); context = Some(line.trim_start_matches('@').trim().to_string()); }
            Some('-') => old_lines.push(line[1..].to_string()),
            Some('+') => new_lines.push(line[1..].to_string()),
            Some(' ') => { old_lines.push(line[1..].to_string()); new_lines.push(line[1..].to_string()); }
            _ => {}
        }
        i += 1;
    }
    flush_update_chunk(&mut chunks, &mut old_lines, &mut new_lines, &mut context, true);
    (chunks, i)
}

fn flush_update_chunk(chunks: &mut Vec<UpdateChunk>, old_lines: &mut Vec<String>, new_lines: &mut Vec<String>, context: &mut Option<String>, eof: bool) {
    if old_lines.is_empty() && new_lines.is_empty() { return; }
    chunks.push(UpdateChunk { old_lines: std::mem::take(old_lines), new_lines: std::mem::take(new_lines), change_context: context.take(), is_end_of_file: eof });
}

fn patch_relative_paths(hunks: &[PatchHunk]) -> Vec<String> {
    let mut paths = Vec::new();
    for hunk in hunks {
        match hunk {
            PatchHunk::Add { path, .. } | PatchHunk::Delete { path } => paths.push(path.clone()),
            PatchHunk::Update { path, move_path, .. } => { paths.push(path.clone()); if let Some(move_path) = move_path { paths.push(move_path.clone()); } }
        }
    }
    paths.sort(); paths.dedup(); paths
}

fn validate_patch_paths(hunks: &[PatchHunk], workspace_root: &str) -> Result<Vec<serde_json::Value>> {
    patch_relative_paths(hunks).into_iter().map(|path| {
        validate_relative_patch_path(&path)?;
        let full_path = Path::new(workspace_root).join(&path).display().to_string();
        Ok(serde_json::json!({"path": path, "fullPath": full_path}))
    }).collect()
}

pub(crate) fn validate_relative_patch_path(path: &str) -> Result<()> {
    let raw = Path::new(path);
    if raw.is_absolute() { anyhow::bail!("apply_patch path must be relative to the workspace: {path}"); }
    if path.trim().is_empty() || path.contains('\0') { anyhow::bail!("apply_patch path is invalid: {path}"); }
    for component in raw.components() {
        match component {
            Component::ParentDir => anyhow::bail!("apply_patch path leaves workspace: {path}"),
            Component::Prefix(_) | Component::RootDir => anyhow::bail!("apply_patch path must be relative to the workspace: {path}"),
            _ => {}
        }
    }
    Ok(())
}
