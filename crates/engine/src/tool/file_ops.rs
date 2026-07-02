//! File operation tools.

use crate::tool::patch_events;
use crate::tool::ToolExecutor;
use crate::types::{ToolKind, ToolRequest, ToolResult};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;

const UTF8_BOM: &str = "\u{feff}";
const UTF8_BOM_BYTES: &[u8] = &[0xef, 0xbb, 0xbf];

impl ToolExecutor {
    pub async fn execute_file_read(&self, request: ToolRequest) -> Result<ToolResult> {
        let path = path_arg(&request.args)?;
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
        #[derive(serde::Deserialize)] struct Args { path: String, content: String }
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
        let formatter_status = format_file_like_forge(&full_path, keep_bom).await;
        let final_bytes = fs::read(&full_path).await.unwrap_or_else(|_| write_content.as_bytes().to_vec());
        let final_text = String::from_utf8_lossy(&final_bytes).to_string();
        let kind = if existed { "update" } else { "add" };
        let files = vec![file_change_record(&args.path, kind, line_count(&final_text), if existed { 1 } else { 0 }, keep_bom)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(args.path));
        metadata.insert("bytes".to_string(), serde_json::json!(final_bytes.len()));
        metadata.insert("bom".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_preserved".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_strategy".to_string(), serde_json::json!("writeTextPreservingBom: preserve existing/input UTF-8 BOM and emit at most one BOM"));
        metadata.insert("formatter_status".to_string(), formatter_status);
        metadata.insert("forge_formatter_contract".to_string(), forge_formatter_contract());
        metadata.insert("forge_file_tool_contract".to_string(), forge_file_tool_contract());
        Ok(ToolResult { id: request.id, kind: ToolKind::FileWrite, success: true, output: format!("Written {} bytes to {}", final_bytes.len(), metadata.get("path").and_then(serde_json::Value::as_str).unwrap_or("file")), error: None, duration_ms: 0, metadata })
    }

    pub async fn execute_file_edit(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)] struct Args { path: String, old_string: String, new_string: String, replace_all: Option<bool> }
        let args: Args = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&args.path)?;
        let bytes = fs::read(&full_path).await?;
        let existing_bom = has_utf8_bom(&bytes);
        let content = String::from_utf8(bytes).with_context(|| format!("File is not valid UTF-8: {}", full_path.display()))?;
        let replace_all = args.replace_all.unwrap_or(false);
        let Some(edit) = replace_file_edit_content(&content, &args.old_string, &args.new_string, replace_all) else {
            let mut metadata = stale_file_edit_metadata(&args.path, &args.old_string, &content);
            metadata.insert("replace_all".to_string(), serde_json::json!(replace_all));
            return Ok(ToolResult {
                id: request.id,
                kind: ToolKind::FileEdit,
                success: false,
                output: "file_edit failed: old_string was not found in the current file after exact and OpenCode-backed fuzzy replacement strategies. Treat this as stale edit evidence: read the current file, then retry with the current exact text or use apply_patch with current context.".to_string(),
                error: Some("stale_exact_replacement_old_string_not_found".to_string()),
                duration_ms: 0,
                metadata,
            });
        };
        let new_content = edit.content;
        let keep_bom = existing_bom || new_content.starts_with(UTF8_BOM);
        let write_content = join_bom(&new_content, keep_bom);
        fs::write(&full_path, write_content.as_bytes()).await?;
        let formatter_status = format_file_like_forge(&full_path, keep_bom).await;
        let final_bytes = fs::read(&full_path).await.unwrap_or_else(|_| write_content.as_bytes().to_vec());
        let final_text = String::from_utf8_lossy(&final_bytes).to_string();
        let files = vec![file_change_record(&args.path, "update", line_count(&final_text), line_count(&edit.matched), keep_bom)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(args.path));
        metadata.insert("replacements".to_string(), serde_json::json!(if replace_all { "all" } else { "first" }));
        metadata.insert("edit_match_strategy".to_string(), serde_json::json!(edit.strategy));
        metadata.insert("edit_matched_old_string_preview".to_string(), serde_json::json!(preview_text(&edit.matched, 240)));
        metadata.insert("forge_edit_replacer_contract".to_string(), forge_edit_replacer_contract());
        metadata.insert("bom".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_preserved".to_string(), serde_json::json!(existing_bom && keep_bom));
        metadata.insert("bom_strategy".to_string(), serde_json::json!("writeTextPreservingBom: preserve existing/input UTF-8 BOM and emit at most one BOM"));
        metadata.insert("formatter_status".to_string(), formatter_status);
        metadata.insert("forge_formatter_contract".to_string(), forge_formatter_contract());
        metadata.insert("forge_file_tool_contract".to_string(), forge_file_tool_contract());
        Ok(ToolResult { id: request.id, kind: ToolKind::FileEdit, success: true, output: format!("Edited {} via {}", metadata.get("path").and_then(serde_json::Value::as_str).unwrap_or("file"), edit.strategy), error: None, duration_ms: 0, metadata })
    }

    pub async fn execute_file_delete(&self, request: ToolRequest) -> Result<ToolResult> {
        let path = path_arg(&request.args)?;
        let full_path = self.resolve_path(&path)?;
        fs::remove_file(&full_path).await.with_context(|| format!("Failed to delete file: {}", full_path.display()))?;
        let files = vec![file_change_record(&path, "delete", 0, 1, false)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(path.clone()));
        metadata.insert("forge_file_tool_contract".to_string(), forge_file_tool_contract());
        Ok(ToolResult { id: request.id, kind: ToolKind::FileDelete, success: true, output: format!("Deleted {path}"), error: None, duration_ms: 0, metadata })
    }

    pub async fn execute_file_list(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)] struct Args { path: Option<String>, depth: Option<usize> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = workspace_path(args.path);
        let full_path = self.resolve_path(&path)?;
        let max_depth = args.depth.unwrap_or(3);
        let mut entries = Vec::new();
        self.list_dir_recursive(&full_path, Path::new(""), 0, max_depth, &mut entries).await?;
        Ok(ToolResult { id: request.id, kind: ToolKind::FileList, success: true, output: serde_json::to_string_pretty(&entries)?, error: None, duration_ms: 0, metadata: HashMap::from([("path".to_string(), serde_json::json!(path)), ("count".to_string(), serde_json::json!(entries.len()))]) })
    }

    pub async fn execute_file_glob(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)] struct Args { pattern: String, path: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = workspace_path(args.path);
        let full_path = self.resolve_path(&path)?;
        let pattern = args.pattern.clone();
        let mut matches = Vec::new();
        for entry in glob::glob(full_path.join(&pattern).to_str().unwrap())? { matches.push(entry?.to_string_lossy().to_string()); }
        Ok(ToolResult { id: request.id, kind: ToolKind::FileGlob, success: true, output: serde_json::to_string_pretty(&matches)?, error: None, duration_ms: 0, metadata: HashMap::from([("pattern".to_string(), serde_json::json!(pattern)), ("count".to_string(), serde_json::json!(matches.len()))]) })
    }

    pub async fn execute_file_search(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)] struct Args { pattern: String, path: Option<String>, file_pattern: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        let path = workspace_path(args.path);
        let full_path = self.resolve_path(&path)?;
        let mut results = Vec::new();
        let file_pattern = args.file_pattern.unwrap_or_else(|| "**".to_string());
        for entry in glob::glob(full_path.join(file_pattern).to_str().unwrap())? {
            let entry = entry?;
            if entry.is_file() {
                if let Ok(content) = fs::read_to_string(&entry).await {
                    for (line_num, line) in content.lines().enumerate() {
                        if line.contains(&args.pattern) { results.push(serde_json::json!({"file": entry.to_string_lossy(), "line": line_num + 1, "content": line.trim()})); }
                    }
                }
            }
        }
        Ok(ToolResult { id: request.id, kind: ToolKind::FileSearch, success: true, output: serde_json::to_string_pretty(&results)?, error: None, duration_ms: 0, metadata: HashMap::from([("pattern".to_string(), serde_json::json!(args.pattern)), ("matches".to_string(), serde_json::json!(results.len()))]) })
    }

    pub fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let normalized = workspace_path(Some(path.to_string()));
        let path = Path::new(&normalized);
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
            if is_dir { let fut = Box::pin(self.list_dir_recursive(&path, &rel_path, depth + 1, max_depth, entries)); fut.await?; }
        }
        Ok(())
    }
}

struct EditReplacement { content: String, matched: String, strategy: &'static str }

fn replace_file_edit_content(content: &str, old_string: &str, new_string: &str, replace_all: bool) -> Option<EditReplacement> {
    if old_string.is_empty() || old_string == new_string { return None; }
    let exact = if replace_all { content.replace(old_string, new_string) } else { content.replacen(old_string, new_string, 1) };
    if exact != content {
        return Some(EditReplacement { content: exact, matched: old_string.to_string(), strategy: "simple_exact" });
    }
    let candidates = [
        ("line_trimmed", line_trimmed_candidate(content, old_string)),
        ("whitespace_normalized", whitespace_normalized_candidate(content, old_string)),
        ("indentation_flexible", indentation_flexible_candidate(content, old_string)),
        ("trimmed_boundary", trimmed_boundary_candidate(content, old_string)),
    ];
    for (strategy, candidate) in candidates {
        if let Some(matched) = candidate {
            if let Some(next) = replace_unique_candidate(content, &matched, new_string, replace_all) {
                return Some(EditReplacement { content: next, matched, strategy });
            }
        }
    }
    None
}

fn replace_unique_candidate(content: &str, matched: &str, new_string: &str, replace_all: bool) -> Option<String> {
    if matched.is_empty() { return None; }
    let first = content.find(matched)?;
    if replace_all { return Some(content.replace(matched, new_string)); }
    if content[first + matched.len()..].contains(matched) { return None; }
    Some(format!("{}{}{}", &content[..first], new_string, &content[first + matched.len()..]))
}

fn line_trimmed_candidate(content: &str, old_string: &str) -> Option<String> {
    let content_lines: Vec<&str> = content.split('\n').collect();
    let mut old_lines: Vec<&str> = old_string.split('\n').collect();
    if old_lines.last() == Some(&"") { old_lines.pop(); }
    if old_lines.is_empty() || old_lines.len() > content_lines.len() { return None; }
    let mut matches = Vec::new();
    for start in 0..=content_lines.len() - old_lines.len() {
        if old_lines.iter().enumerate().all(|(offset, old)| content_lines[start + offset].trim() == old.trim()) {
            matches.push(content_lines[start..start + old_lines.len()].join("\n"));
        }
    }
    if matches.len() == 1 { matches.pop() } else { None }
}

fn whitespace_normalized_candidate(content: &str, old_string: &str) -> Option<String> {
    let target = normalize_whitespace(old_string);
    if target.is_empty() { return None; }
    let lines: Vec<&str> = content.split('\n').collect();
    let old_line_count = old_string.split('\n').count();
    let mut matches = Vec::new();
    for line in &lines {
        if normalize_whitespace(line) == target { matches.push((*line).to_string()); }
    }
    if old_line_count > 1 && old_line_count <= lines.len() {
        for start in 0..=lines.len() - old_line_count {
            let block = lines[start..start + old_line_count].join("\n");
            if normalize_whitespace(&block) == target { matches.push(block); }
        }
    }
    matches.sort();
    matches.dedup();
    if matches.len() == 1 { matches.pop() } else { None }
}

fn indentation_flexible_candidate(content: &str, old_string: &str) -> Option<String> {
    let old_lines: Vec<&str> = old_string.split('\n').collect();
    if old_lines.len() <= 1 { return None; }
    let content_lines: Vec<&str> = content.split('\n').collect();
    if old_lines.len() > content_lines.len() { return None; }
    let target = remove_min_indent(old_string);
    let mut matches = Vec::new();
    for start in 0..=content_lines.len() - old_lines.len() {
        let block = content_lines[start..start + old_lines.len()].join("\n");
        if remove_min_indent(&block) == target { matches.push(block); }
    }
    if matches.len() == 1 { matches.pop() } else { None }
}

fn trimmed_boundary_candidate(content: &str, old_string: &str) -> Option<String> {
    let trimmed = old_string.trim();
    if trimmed.is_empty() || trimmed == old_string { return None; }
    if content.matches(trimmed).count() == 1 { return Some(trimmed.to_string()); }
    let lines: Vec<&str> = content.split('\n').collect();
    let old_line_count = old_string.split('\n').count();
    if old_line_count > lines.len() { return None; }
    let mut matches = Vec::new();
    for start in 0..=lines.len() - old_line_count {
        let block = lines[start..start + old_line_count].join("\n");
        if block.trim() == trimmed { matches.push(block); }
    }
    if matches.len() == 1 { matches.pop() } else { None }
}

fn normalize_whitespace(value: &str) -> String { value.split_whitespace().collect::<Vec<_>>().join(" ") }

fn remove_min_indent(value: &str) -> String {
    let min_indent = value.lines().filter(|line| !line.trim().is_empty()).map(|line| line.chars().take_while(|ch| ch.is_whitespace()).count()).min().unwrap_or(0);
    value.lines().map(|line| if line.trim().is_empty() { "".to_string() } else { line.chars().skip(min_indent).collect::<String>() }).collect::<Vec<_>>().join("\n")
}

fn workspace_path(path: Option<String>) -> String {
    let raw = path.unwrap_or_else(|| ".".to_string());
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "/" { ".".to_string() } else { trimmed.to_string() }
}

fn path_arg(value: &serde_json::Value) -> Result<String> {
    if let Some(path) = value.as_str() { return Ok(workspace_path(Some(path.to_string()))); }
    if let Some(path) = value.get("path").and_then(serde_json::Value::as_str) { return Ok(workspace_path(Some(path.to_string()))); }
    anyhow::bail!("expected path string or object with path")
}

fn line_count(value: &str) -> usize { value.lines().count().max(1) }
fn file_change_record(path: &str, kind: &str, additions: usize, deletions: usize, bom: bool) -> serde_json::Value { serde_json::json!({"type": kind, "path": path, "relativePath": path, "additions": additions, "deletions": deletions, "bom": bom}) }

fn stale_file_edit_metadata(path: &str, old_string: &str, current_content: &str) -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("path".to_string(), serde_json::json!(path)),
        ("stale_exact_replacement".to_string(), serde_json::json!(true)),
        ("current_file_lines".to_string(), serde_json::json!(line_count(current_content))),
        ("old_string_lines".to_string(), serde_json::json!(line_count(old_string))),
        ("old_string_preview".to_string(), serde_json::json!(preview_text(old_string, 240))),
        ("current_file_preview".to_string(), serde_json::json!(preview_text(current_content, 360))),
        ("recovery_hint".to_string(), serde_json::json!("The old_string did not match the current file after exact and fuzzy strategies. Read the current file and retry with current exact text, or use apply_patch with current context.")),
        ("recommended_next_tools".to_string(), serde_json::json!(["file_read", "file_edit", "apply_patch"])),
        ("forge_edit_replacer_contract".to_string(), forge_edit_replacer_contract()),
        ("forge_tool_failure_lifecycle".to_string(), serde_json::json!("failed file_edit is returned as first-class error state with original input, explicit error, current-file preview, fuzzy-replacer contract, and recovery guidance")),
    ])
}

fn preview_text(value: &str, limit: usize) -> String {
    let normalized = value.lines().take(24).collect::<Vec<_>>().join("\n");
    let mut out = normalized.chars().take(limit).collect::<String>();
    if normalized.chars().count() > limit { out.push_str("..."); }
    out
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
        ("forge_event_publisher".to_string(), serde_json::json!("forge.file_tool")),
        ("forge_event_contract".to_string(), patch_events::forge_event_contract()),
        ("forge_watcher_updates".to_string(), serde_json::json!(watcher_updates)),
        ("forge_filesystem_edits".to_string(), serde_json::json!(filesystem_edits)),
        ("forge_lsp_warmups".to_string(), serde_json::json!(lsp_warmups)),
        ("forge_lsp_diagnostics".to_string(), serde_json::json!(diagnostic_reports)),
        ("lsp_touches".to_string(), serde_json::json!(lsp_touches)),
        ("diagnostics".to_string(), diagnostics),
    ])
}

async fn format_file_like_forge(path: &Path, desired_bom: bool) -> serde_json::Value {
    let Some(formatter) = formatter_for(path) else {
        return serde_json::json!({
            "name": serde_json::Value::Null,
            "matched": false,
            "enabled": false,
            "applied": false,
            "status": "no_formatter",
            "contract": "formatter probe returned no matching extension"
        });
    };
    if Command::new(formatter.command).arg("--version").output().await.is_err() {
        return serde_json::json!({
            "name": formatter.name,
            "matched": true,
            "enabled": false,
            "applied": false,
            "status": "formatter_unavailable",
            "command": formatter.command,
            "args": formatter.args,
            "extensions": formatter.extensions,
            "contract": "formatter command was probed and safely disabled when unavailable"
        });
    }
    let output = Command::new(formatter.command).args(formatter.args).arg(path).output().await;
    match output {
        Ok(result) => {
            let _ = sync_bom_to_file(path, desired_bom).await;
            serde_json::json!({
                "name": formatter.name,
                "matched": true,
                "enabled": true,
                "applied": result.status.success(),
                "status": if result.status.success() { "formatted" } else { "formatter_failed_contained" },
                "exit_code": result.status.code(),
                "command": formatter.command,
                "args": formatter.args,
                "extensions": formatter.extensions,
                "bom_resynced": true,
                "contract": "write/edit formatting is contained and BOM is resynchronized before file events"
            })
        }
        Err(error) => serde_json::json!({
            "name": formatter.name,
            "matched": true,
            "enabled": true,
            "applied": false,
            "status": "spawn_failed_contained",
            "command": formatter.command,
            "args": formatter.args,
            "extensions": formatter.extensions,
            "error": error.to_string(),
            "contract": "formatter spawn errors are contained in tool metadata"
        }),
    }
}

#[derive(Clone, Copy)]
struct FormatterSpec { name: &'static str, command: &'static str, args: &'static [&'static str], extensions: &'static [&'static str] }

fn formatter_for(path: &Path) -> Option<FormatterSpec> {
    let filename = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
    let ext = path.extension().and_then(|value| value.to_str()).unwrap_or_default();
    FORMATTERS.iter().find(|formatter| formatter.extensions.iter().any(|item| ext == *item || filename.ends_with(item))).copied()
}

const FORMATTERS: &[FormatterSpec] = &[
    FormatterSpec { name: "rustfmt", command: "rustfmt", args: &[], extensions: &["rs"] },
    FormatterSpec { name: "gofmt", command: "gofmt", args: &["-w"], extensions: &["go"] },
    FormatterSpec { name: "mix", command: "mix", args: &["format"], extensions: &["ex", "exs", "eex", "heex", "leex", "neex", "sface"] },
    FormatterSpec { name: "prettier", command: "prettier", args: &["--write"], extensions: &["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts", "html", "htm", "css", "scss", "sass", "less", "vue", "svelte", "json", "jsonc", "yaml", "yml", "toml", "xml", "md", "mdx", "graphql", "gql"] },
    FormatterSpec { name: "oxfmt", command: "oxfmt", args: &[], extensions: &["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts"] },
    FormatterSpec { name: "biome", command: "biome", args: &["format", "--write"], extensions: &["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts", "html", "htm", "css", "scss", "sass", "less", "vue", "svelte", "json", "jsonc", "yaml", "yml", "toml", "xml", "md", "mdx", "graphql", "gql"] },
    FormatterSpec { name: "ruff", command: "ruff", args: &["format"], extensions: &["py", "pyi"] },
    FormatterSpec { name: "uv", command: "uv", args: &["format", "--"], extensions: &["py", "pyi"] },
    FormatterSpec { name: "clang-format", command: "clang-format", args: &["-i"], extensions: &["c", "cc", "cpp", "cxx", "c++", "h", "hh", "hpp", "hxx", "h++", "ino", "C", "H"] },
    FormatterSpec { name: "shfmt", command: "shfmt", args: &["-w"], extensions: &["sh", "bash"] },
    FormatterSpec { name: "terraform", command: "terraform", args: &["fmt"], extensions: &["tf", "tfvars"] },
    FormatterSpec { name: "zig", command: "zig", args: &["fmt"], extensions: &["zig", "zon"] },
    FormatterSpec { name: "dart", command: "dart", args: &["format"], extensions: &["dart"] },
    FormatterSpec { name: "ktlint", command: "ktlint", args: &["-F"], extensions: &["kt", "kts"] },
    FormatterSpec { name: "rubocop", command: "rubocop", args: &["--autocorrect"], extensions: &["rb", "rake", "gemspec", "ru"] },
    FormatterSpec { name: "standardrb", command: "standardrb", args: &["--fix"], extensions: &["rb", "rake", "gemspec", "ru"] },
    FormatterSpec { name: "htmlbeautifier", command: "htmlbeautifier", args: &[], extensions: &["erb", "html.erb"] },
    FormatterSpec { name: "ocamlformat", command: "ocamlformat", args: &["-i"], extensions: &["ml", "mli"] },
    FormatterSpec { name: "latexindent", command: "latexindent", args: &["-w", "-s"], extensions: &["tex"] },
    FormatterSpec { name: "gleam", command: "gleam", args: &["format"], extensions: &["gleam"] },
    FormatterSpec { name: "nixfmt", command: "nixfmt", args: &[], extensions: &["nix"] },
    FormatterSpec { name: "air", command: "air", args: &["format"], extensions: &["R"] },
    FormatterSpec { name: "pint", command: "./vendor/bin/pint", args: &[], extensions: &["php"] },
    FormatterSpec { name: "ormolu", command: "ormolu", args: &["-i"], extensions: &["hs"] },
    FormatterSpec { name: "cljfmt", command: "cljfmt", args: &["fix", "--quiet"], extensions: &["clj", "cljs", "cljc", "edn"] },
    FormatterSpec { name: "dfmt", command: "dfmt", args: &["-i"], extensions: &["d"] },
];

async fn sync_bom_to_file(path: &Path, desired_bom: bool) -> Result<()> { let bytes = fs::read(path).await?; let normalized = normalize_bom_bytes(&bytes, desired_bom); if normalized != bytes { fs::write(path, normalized).await?; } Ok(()) }
fn normalize_bom_bytes(bytes: &[u8], desired_bom: bool) -> Vec<u8> { let mut body = bytes; while body.starts_with(UTF8_BOM_BYTES) { body = &body[UTF8_BOM_BYTES.len()..]; } if desired_bom { let mut result = UTF8_BOM_BYTES.to_vec(); result.extend_from_slice(body); result } else { body.to_vec() } }
fn has_utf8_bom(content: &[u8]) -> bool { content.starts_with(UTF8_BOM_BYTES) }
fn split_bom(text: &str) -> (bool, String) { let stripped = text.trim_start_matches(UTF8_BOM); (stripped.len() != text.len(), stripped.to_string()) }
fn join_bom(text: &str, bom: bool) -> String { let (_, stripped) = split_bom(text); if bom { format!("{UTF8_BOM}{stripped}") } else { stripped } }
fn forge_edit_replacer_contract() -> serde_json::Value { serde_json::json!({"source_backing": ["packages/opencode/src/tool/edit.ts"], "behaviors": ["attempt exact replacement first", "fall back to line-trimmed matching", "fall back to whitespace-normalized matching", "fall back to indentation-flexible matching", "fall back to trimmed-boundary matching", "require a unique fuzzy match unless replace_all is explicitly requested", "keep stale-edit failures as first-class tool error states with recovery metadata"]}) }
fn forge_formatter_contract() -> serde_json::Value { serde_json::json!({"source_backing": ["packages/opencode/src/format/index.ts", "packages/opencode/src/format/formatter.ts"], "behaviors": ["match formatter commands by extension using a catalog", "probe matching formatter commands and safely disable unavailable commands", "run write/edit formatting after mutation when a formatter is available", "contain formatter spawn/status failures in metadata instead of failing the file tool", "resynchronize desired UTF-8 BOM after formatter mutation", "cover upstream formatter families for Rust, Go, Elixir, JS/TS/web/data/doc, Python, C/C++, shell, Terraform, Zig, Dart, Kotlin, Ruby, OCaml, LaTeX, Gleam, Nix, R, PHP, Haskell, Clojure, and D"]}) }
fn forge_file_tool_contract() -> serde_json::Value { serde_json::json!({"behaviors": ["emit file edit events after write/edit/delete", "emit watcher-style update records", "touch LSP documents", "collect contained diagnostic report envelopes", "preserve an existing/input UTF-8 BOM and emit at most one BOM", "run formatting after write/edit and resynchronize BOM", "use OpenCode-backed fuzzy file edit replacement before returning stale-edit recovery"]}) }
