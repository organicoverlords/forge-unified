//! File operation tools.

use crate::tool::patch_events;
use crate::tool::ToolExecutor;
use crate::types::{ToolKind, ToolRequest, ToolResult};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

const UTF8_BOM: &str = "\u{feff}";
const UTF8_BOM_BYTES: &[u8] = &[0xef, 0xbb, 0xbf];

impl ToolExecutor {
    pub async fn execute_file_read(&self, request: ToolRequest) -> Result<ToolResult> {
        let path: String = serde_json::from_value(request.args.clone())?;
        let full_path = self.resolve_path(&path)?;
        let content = fs::read_to_string(&full_path).await.with_context(|| format!("Failed to read file: {}", full_path.display()))?;
        let content_clone = content.clone();
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileRead,
            success: true,
            output: content,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("path".to_string(), serde_json::json!(path)),
                ("size".to_string(), serde_json::json!(content_clone.len())),
                ("bom".to_string(), serde_json::json!(content_clone.starts_with(UTF8_BOM))),
            ]),
        })
    }

    pub async fn execute_file_write(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { path: String, content: String }
        let args: Args = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&args.path)?;
        let existed = full_path.exists();
        let current = if existed { fs::read(&full_path).await.ok() } else { None };
        let existing_bom = current.as_deref().map(has_utf8_bom).unwrap_or(false);
        let input_bom = args.content.starts_with(UTF8_BOM);
        let keep_bom = existing_bom || input_bom;
        let write_content = join_bom(&args.content, keep_bom);
        if let Some(parent) = full_path.parent() { fs::create_dir_all(parent).await?; }
        fs::write(&full_path, write_content.as_bytes()).await.with_context(|| format!("Failed to write file: {}", full_path.display()))?;
        let kind = if existed { "update" } else { "add" };
        let files = vec![file_change_record(&args.path, kind, line_count(&write_content), if existed { 1 } else { 0 }, keep_bom)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(args.path));
        metadata.insert("bytes".to_string(), serde_json::json!(write_content.len()));
        metadata.insert("bom".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_preserved".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_strategy".to_string(), serde_json::json!("writeTextPreservingBom: preserve existing/input UTF-8 BOM and emit at most one BOM"));
        metadata.insert("opencode_file_tool_source".to_string(), opencode_file_tool_source());
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileWrite,
            success: true,
            output: format!("Written {} bytes to {}", write_content.len(), metadata.get("path").and_then(serde_json::Value::as_str).unwrap_or("file")),
            error: None,
            duration_ms: 0,
            metadata,
        })
    }

    pub async fn execute_file_edit(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { path: String, old_string: String, new_string: String, replace_all: Option<bool> }
        let args: Args = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&args.path)?;
        let bytes = fs::read(&full_path).await?;
        let existing_bom = has_utf8_bom(&bytes);
        let content = String::from_utf8(bytes).with_context(|| format!("File is not valid UTF-8: {}", full_path.display()))?;
        let replace_all = args.replace_all.unwrap_or(false);
        let new_content = if replace_all { content.replace(&args.old_string, &args.new_string) } else { content.replacen(&args.old_string, &args.new_string, 1) };
        if new_content == content {
            return Ok(ToolResult {
                id: request.id,
                kind: ToolKind::FileEdit,
                success: false,
                output: "Old string not found in file".to_string(),
                error: Some("Old string not found".to_string()),
                duration_ms: 0,
                metadata: HashMap::new(),
            });
        }
        let keep_bom = existing_bom || new_content.starts_with(UTF8_BOM);
        let write_content = join_bom(&new_content, keep_bom);
        fs::write(&full_path, write_content.as_bytes()).await?;
        let files = vec![file_change_record(&args.path, "update", line_count(&args.new_string), line_count(&args.old_string), keep_bom)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(args.path));
        metadata.insert("replacements".to_string(), serde_json::json!(if replace_all { "all" } else { "first" }));
        metadata.insert("bom".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_preserved".to_string(), serde_json::json!(existing_bom && keep_bom));
        metadata.insert("bom_strategy".to_string(), serde_json::json!("writeTextPreservingBom: preserve existing/input UTF-8 BOM and emit at most one BOM"));
        metadata.insert("opencode_file_tool_source".to_string(), opencode_file_tool_source());
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileEdit,
            success: true,
            output: format!("Edited {}", metadata.get("path").and_then(serde_json::Value::as_str).unwrap_or("file")),
            error: None,
            duration_ms: 0,
            metadata,
        })
    }

    pub async fn execute_file_delete(&self, request: ToolRequest) -> Result<ToolResult> {
        let path: String = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&path)?;
        fs::remove_file(&full_path).await.with_context(|| format!("Failed to delete file: {}", full_path.display()))?;
        let files = vec![file_change_record(&path, "delete", 0, 1, false)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(path.clone()));
        metadata.insert("opencode_file_tool_source".to_string(), opencode_file_tool_source());
        Ok(ToolResult { id: request.id, kind: ToolKind::FileDelete, success: true, output: format!("Deleted {path}"), error: None, duration_ms: 0, metadata })
    }

    pub async fn execute_file_list(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { path: Option<String>, depth: Option<usize> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let full_path = self.resolve_path(&path)?;
        let max_depth = args.depth.unwrap_or(3);
        let mut entries = Vec::new();
        self.list_dir_recursive(&full_path, Path::new(""), 0, max_depth, &mut entries).await?;
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileList,
            success: true,
            output: serde_json::to_string_pretty(&entries)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("path".to_string(), serde_json::json!(path)),
                ("count".to_string(), serde_json::json!(entries.len())),
            ]),
        })
    }

    pub async fn execute_file_glob(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { pattern: String, path: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let full_path = self.resolve_path(&path)?;
        let pattern = args.pattern.clone();
        let mut matches = Vec::new();
        for entry in glob::glob(full_path.join(&pattern).to_str().unwrap())? { matches.push(entry?.to_string_lossy().to_string()); }
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileGlob,
            success: true,
            output: serde_json::to_string_pretty(&matches)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("pattern".to_string(), serde_json::json!(pattern)),
                ("count".to_string(), serde_json::json!(matches.len())),
            ]),
        })
    }

    pub async fn execute_file_search(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { pattern: String, path: Option<String>, file_pattern: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let full_path = self.resolve_path(&path)?;
        let mut results = Vec::new();
        let file_pattern = args.file_pattern.unwrap_or_else(|| "**".to_string());
        for entry in glob::glob(full_path.join(file_pattern).to_str().unwrap())? {
            let entry = entry?;
            if entry.is_file() {
                if let Ok(content) = fs::read_to_string(&entry).await {
                    for (line_num, line) in content.lines().enumerate() {
                        if line.contains(&args.pattern) {
                            results.push(serde_json::json!({"file": entry.to_string_lossy(), "line": line_num + 1, "content": line.trim()}));
                        }
                    }
                }
            }
        }
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileSearch,
            success: true,
            output: serde_json::to_string_pretty(&results)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("pattern".to_string(), serde_json::json!(args.pattern)),
                ("matches".to_string(), serde_json::json!(results.len())),
            ]),
        })
    }

    pub fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let path = Path::new(path);
        let full = if path.is_absolute() { path.to_path_buf() } else { Path::new(&self.workspace_root).join(path) };
        let canonical = full.canonicalize().or_else(|_| Ok::<PathBuf, anyhow::Error>(full.clone()))?;
        let workspace_canonical = Path::new(&self.workspace_root).canonicalize()?;
        if !canonical.starts_with(&workspace_canonical) { anyhow::bail!("Path escapes workspace: {}", path.display()); }
        Ok(canonical)
    }

    async fn list_dir_recursive(&self, root: &Path, rel: &Path, depth: usize, max_depth: usize, entries: &mut Vec<serde_json::Value>) -> Result<()> {
        if depth > max_depth { return Ok(()); }
        let mut dir = fs::read_dir(root).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let rel_path = rel.join(entry.file_name());
            let is_dir = path.is_dir();
            let name = entry.file_name().to_string_lossy().to_string();
            if is_dir && (name == ".git" || name == "target" || name == "node_modules") { continue; }
            entries.push(serde_json::json!({"name": name, "path": rel_path.to_string_lossy(), "is_dir": is_dir}));
            if is_dir {
                let fut = Box::pin(self.list_dir_recursive(&path, &rel_path, depth + 1, max_depth, entries));
                fut.await?;
            }
        }
        Ok(())
    }
}

fn line_count(value: &str) -> usize { value.lines().count().max(1) }

fn file_change_record(path: &str, kind: &str, additions: usize, deletions: usize, bom: bool) -> serde_json::Value {
    serde_json::json!({"type": kind, "path": path, "relativePath": path, "additions": additions, "deletions": deletions, "bom": bom})
}

fn file_event_metadata(files: Vec<serde_json::Value>) -> HashMap<String, serde_json::Value> {
    let file_events = patch_events::file_change_events(&files);
    let watcher_updates = patch_events::watcher_updates(&files);
    let filesystem_edits = patch_events::filesystem_edits(&files);
    let lsp_touches = patch_events::lsp_touches(&files);
    let lsp_warmups = patch_events::lsp_warmups(&files);
    let diagnostic_reports = patch_events::diagnostic_reports(&files);
    let diagnostics = patch_events::diagnostics_metadata(&files);
    HashMap::from([
        ("files".to_string(), serde_json::json!(files)),
        ("file_events".to_string(), serde_json::json!(file_events)),
        ("opencode_event_publisher".to_string(), serde_json::json!("opencode.file_tool")),
        ("opencode_event_source".to_string(), patch_events::opencode_event_source()),
        ("opencode_watcher_updates".to_string(), serde_json::json!(watcher_updates)),
        ("opencode_filesystem_edits".to_string(), serde_json::json!(filesystem_edits)),
        ("opencode_lsp_warmups".to_string(), serde_json::json!(lsp_warmups)),
        ("opencode_lsp_diagnostics".to_string(), serde_json::json!(diagnostic_reports)),
        ("lsp_touches".to_string(), serde_json::json!(lsp_touches)),
        ("diagnostics".to_string(), diagnostics),
    ])
}

fn has_utf8_bom(content: &[u8]) -> bool { content.starts_with(UTF8_BOM_BYTES) }

fn split_bom(text: &str) -> (bool, String) {
    let stripped = text.trim_start_matches(UTF8_BOM);
    (stripped.len() != text.len(), stripped.to_string())
}

fn join_bom(text: &str, bom: bool) -> String {
    let (_, stripped) = split_bom(text);
    if bom { format!("{UTF8_BOM}{stripped}") } else { stripped }
}

fn opencode_file_tool_source() -> serde_json::Value {
    serde_json::json!({
        "paths": ["packages/opencode/src/tool/write.ts", "packages/opencode/src/tool/edit.ts", "packages/opencode/src/tool/apply_patch.ts", "packages/core/src/file-mutation.ts"],
        "behaviors": ["events.publish(FileSystem.Event.Edited)", "events.publish(Watcher.Event.Updated)", "lsp.touchFile(document)", "lsp.diagnostics() -> LSP.Diagnostic.report", "FileMutation.writeTextPreservingBom preserves an existing/input UTF-8 BOM and emits at most one BOM"]
    })
}
