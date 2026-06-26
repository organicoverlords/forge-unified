//! OpenCode-compatible apply_patch parser and mutation entrypoint.

use crate::tool::patch_apply::{apply_file_changes, file_change_metadata, file_change_summary_line, prepare_file_changes, total_diff};
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
        struct Args { patchText: String }
        let args: Args = serde_json::from_value(request.args)?;
        let patch_len = args.patchText.len();
        if args.patchText.trim().is_empty() {
            anyhow::bail!("patchText cannot be empty");
        }

        let hunks = match parse_opencode_patch(&args.patchText) {
            Ok(hunks) => hunks,
            Err(err) => return Ok(apply_patch_failure(request.id, patch_len, err.to_string())),
        };
        if hunks.is_empty() {
            let normalized = args.patchText.replace("\r\n", "\n").replace('\r', "\n");
            let error = if normalized.trim() == "*** Begin Patch\n*** End Patch" {
                "patch rejected: empty patch"
            } else {
                "apply_patch verification failed: no hunks found"
            };
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
        if let Err(err) = apply_file_changes(&changes).await {
            return Ok(apply_patch_failure(request.id, patch_len, err.to_string()));
        }

        let files: Vec<_> = changes.iter().map(file_change_metadata).collect();
        let summary_lines = changes.iter().map(file_change_summary_line).collect::<Vec<_>>();
        let diff = total_diff(&changes);
        let output = format!("Success. Updated the following files:\n{}", summary_lines.join("\n"));

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::ApplyPatch,
            success: true,
            output,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("opencode_source".to_string(), opencode_source()),
                ("patch_length".to_string(), serde_json::json!(patch_len)),
                ("hunk_count".to_string(), serde_json::json!(hunks.len())),
                ("files".to_string(), serde_json::json!(files)),
                ("summary_lines".to_string(), serde_json::json!(summary_lines)),
                ("validated_paths".to_string(), serde_json::json!(validated_paths)),
                ("permission".to_string(), serde_json::json!({
                    "required": "edit",
                    "patterns": patch_relative_paths(&hunks),
                    "metadata_ready": true,
                    "diff": diff,
                    "note": "Forge records edit-permission metadata; interactive approval is not wired yet."
                })),
                ("parsed_hunks".to_string(), serde_json::json!(hunks)),
                ("diff".to_string(), serde_json::json!(diff)),
                ("diagnostics".to_string(), serde_json::json!({"status": "not_collected"})),
                ("applied".to_string(), serde_json::json!(true)),
            ]),
        })
    }
}

fn apply_patch_failure(id: ToolCallId, patch_length: usize, error: impl Into<String>) -> ToolResult {
    ToolResult {
        id,
        kind: ToolKind::ApplyPatch,
        success: false,
        output: "Patch validation failed".to_string(),
        error: Some(error.into()),
        duration_ms: 0,
        metadata: HashMap::from([
            ("opencode_source".to_string(), opencode_source()),
            ("patch_length".to_string(), serde_json::json!(patch_length)),
            ("applied".to_string(), serde_json::json!(false)),
        ]),
    }
}

fn opencode_source() -> serde_json::Value {
    serde_json::json!([
        "packages/opencode/src/tool/apply_patch.ts",
        "packages/opencode/src/patch/index.ts"
    ])
}

fn parse_opencode_patch(patch_text: &str) -> Result<Vec<PatchHunk>> {
    let cleaned = strip_heredoc(patch_text.trim());
    let lines: Vec<&str> = cleaned.lines().collect();
    let begin_idx = lines.iter().position(|line| line.trim() == "*** Begin Patch");
    let end_idx = lines.iter().position(|line| line.trim() == "*** End Patch");
    let (Some(begin), Some(end)) = (begin_idx, end_idx) else {
        return Err(anyhow!("Invalid patch format: missing Begin/End markers"));
    };
    if begin >= end {
        return Err(anyhow!("Invalid patch format: missing Begin/End markers"));
    }

    let mut hunks = Vec::new();
    let mut i = begin + 1;
    while i < end {
        let line = lines[i];
        if let Some(path) = line.strip_prefix("*** Add File:").map(str::trim).filter(|value| !value.is_empty()) {
            let (contents, next_idx) = parse_add_file_content(&lines, i + 1, end);
            hunks.push(PatchHunk::Add { path: path.to_string(), contents });
            i = next_idx;
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Delete File:").map(str::trim).filter(|value| !value.is_empty()) {
            hunks.push(PatchHunk::Delete { path: path.to_string() });
            i += 1;
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Update File:").map(str::trim).filter(|value| !value.is_empty()) {
            let mut next_idx = i + 1;
            let mut move_path = None;
            if next_idx < end {
                if let Some(target) = lines[next_idx].strip_prefix("*** Move to:").map(str::trim).filter(|value| !value.is_empty()) {
                    move_path = Some(target.to_string());
                    next_idx += 1;
                }
            }
            let (chunks, after_chunks) = parse_update_chunks(&lines, next_idx, end);
            hunks.push(PatchHunk::Update { path: path.to_string(), move_path, chunks });
            i = after_chunks;
            continue;
        }
        i += 1;
    }
    Ok(hunks)
}

fn strip_heredoc(input: &str) -> String {
    let mut lines = input.lines();
    let Some(first) = lines.next() else { return input.to_string(); };
    let marker = first.trim()
        .strip_prefix("cat <<")
        .or_else(|| first.trim().strip_prefix("<<"))
        .map(|value| value.trim_matches('\'').trim_matches('"').trim().to_string());
    let Some(marker) = marker.filter(|value| !value.is_empty()) else { return input.to_string(); };
    let body: Vec<&str> = lines.collect();
    if body.last().map(|line| line.trim()) == Some(marker.as_str()) { body[..body.len() - 1].join("\n") } else { input.to_string() }
}

fn parse_add_file_content(lines: &[&str], start_idx: usize, end_idx: usize) -> (String, usize) {
    let mut contents = Vec::new();
    let mut i = start_idx;
    while i < end_idx && !lines[i].starts_with("***") {
        if let Some(line) = lines[i].strip_prefix('+') { contents.push(line.to_string()); }
        i += 1;
    }
    (contents.join("\n"), i)
}

fn parse_update_chunks(lines: &[&str], start_idx: usize, end_idx: usize) -> (Vec<UpdateChunk>, usize) {
    let mut chunks = Vec::new();
    let mut i = start_idx;
    while i < end_idx && !lines[i].starts_with("***") {
        if !lines[i].starts_with("@@") { i += 1; continue; }
        let context = lines[i].trim_start_matches("@@").trim();
        i += 1;
        let mut old_lines = Vec::new();
        let mut new_lines = Vec::new();
        let mut is_end_of_file = false;
        while i < end_idx && !lines[i].starts_with("@@") && !lines[i].starts_with("***") {
            let change_line = lines[i];
            if change_line == "*** End of File" { is_end_of_file = true; i += 1; break; }
            if let Some(line) = change_line.strip_prefix(' ') {
                old_lines.push(line.to_string()); new_lines.push(line.to_string());
            } else if let Some(line) = change_line.strip_prefix('-') {
                old_lines.push(line.to_string());
            } else if let Some(line) = change_line.strip_prefix('+') {
                new_lines.push(line.to_string());
            }
            i += 1;
        }
        chunks.push(UpdateChunk { old_lines, new_lines, change_context: (!context.is_empty()).then(|| context.to_string()), is_end_of_file });
    }
    (chunks, i)
}

fn validate_patch_paths(hunks: &[PatchHunk], workspace_root: &str) -> Result<Vec<serde_json::Value>> {
    let workspace = Path::new(workspace_root);
    let mut validated = Vec::new();
    for hunk in hunks {
        for (kind, raw_path) in patch_paths(hunk) {
            validate_relative_patch_path(raw_path)?;
            validated.push(serde_json::json!({ "kind": kind, "path": raw_path, "workspace_target": workspace.join(raw_path).to_string_lossy() }));
        }
    }
    Ok(validated)
}

pub(crate) fn validate_relative_patch_path(raw_path: &str) -> Result<()> {
    let path = raw_path.trim();
    if path.is_empty() { anyhow::bail!("apply_patch verification failed: empty path"); }
    if path.contains('\0') || path.contains('\\') || path.contains(':') {
        anyhow::bail!("apply_patch verification failed: invalid path: {path}");
    }
    let parsed = Path::new(path);
    if parsed.is_absolute() { anyhow::bail!("apply_patch verification failed: path escapes workspace: {path}"); }
    for component in parsed.components() {
        if !matches!(component, Component::Normal(_) | Component::CurDir) {
            anyhow::bail!("apply_patch verification failed: path escapes workspace: {path}");
        }
    }
    Ok(())
}

fn patch_paths(hunk: &PatchHunk) -> Vec<(&'static str, &str)> {
    match hunk {
        PatchHunk::Add { path, .. } | PatchHunk::Delete { path } => vec![("path", path.as_str())],
        PatchHunk::Update { path, move_path, .. } => {
            let mut paths = vec![("path", path.as_str())];
            if let Some(move_path) = move_path.as_deref() { paths.push(("move_path", move_path)); }
            paths
        }
    }
}

fn patch_relative_paths(hunks: &[PatchHunk]) -> Vec<String> {
    hunks.iter().flat_map(|hunk| patch_paths(hunk).into_iter().map(|(_, path)| path.to_string())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_add_update_delete_and_move_hunks() {
        let patch = r#"*** Begin Patch
*** Add File: notes/new.txt
+hello
+world
*** Update File: old.txt
*** Move to: new.txt
@@ heading
-old
+new
*** Delete File: remove.txt
*** End Patch"#;
        let hunks = parse_opencode_patch(patch).unwrap();
        assert_eq!(hunks.len(), 3);
        assert_eq!(patch_summary_lines(&hunks), vec!["A notes/new.txt", "M new.txt", "D remove.txt"]);
    }

    #[test]
    fn rejects_missing_markers() {
        let err = parse_opencode_patch("*** Add File: nope\n+x").unwrap_err();
        assert!(err.to_string().contains("missing Begin/End markers"));
    }

    #[test]
    fn rejects_path_escape() {
        let patch = "*** Begin Patch\n*** Delete File: ../secret.txt\n*** End Patch";
        let hunks = parse_opencode_patch(patch).unwrap();
        assert!(validate_patch_paths(&hunks, ".").unwrap_err().to_string().contains("path escapes workspace"));
    }

    fn patch_summary_lines(hunks: &[PatchHunk]) -> Vec<String> {
        hunks.iter().map(|hunk| match hunk {
            PatchHunk::Add { path, .. } => format!("A {path}"),
            PatchHunk::Delete { path } => format!("D {path}"),
            PatchHunk::Update { path, move_path, .. } => format!("M {}", move_path.as_deref().unwrap_or(path)),
        }).collect()
    }
}
