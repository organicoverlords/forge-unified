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
        let new_content = if replace_all { content.replace(&args.old_string, &args.new_string) } else { content.replacen(&args.old_string, &args.new_string, 1) };
        if new_content == content {
            let mut metadata = stale_file_edit_metadata(&args.path, &args.old_string, &content);
            metadata.insert("replace_all".to_string(), serde_json::json!(replace_all));
            return Ok(ToolResult {
                id: request.id,
                kind: ToolKind::FileEdit,
                success: false,
                output: "file_edit failed: old_string was not found in the current file. Treat this as stale edit evidence: read the current file, then retry with the current exact text or use apply_patch with current context.".to_string(),
                error: Some("stale_exact_replacement_old_string_not_found".to_string()),
                duration_ms: 0,
                metadata,
            });
        }
        let keep_bom = existing_bom || new_content.starts_with(UTF8_BOM);
        let write_content = join_bom(&new_content, keep_bom);
        fs::write(&full_path, write_content.as_bytes()).await?;
        let formatter_status = format_file_like_forge(&full_path, keep_bom).await;
        let final_bytes = fs::read(&full_path).await.unwrap_or_else(|_| write_content.as_bytes().to_vec());
        let final_text = String::from_utf8_lossy(&final_bytes).to_string();
        let files = vec![file_change_record(&args.path, "update", line_count(&final_text), line_count(&args.old_string), keep_bom)];
        let mut metadata = file_event_metadata(files);
        metadata.insert("path".to_string(), serde_json::json!(args.path));
        metadata.insert("replacements".to_string(), serde_json::json!(if replace_all { "all" } else { "first" }));
        metadata.insert("bom".to_string(), serde_json::json!(keep_bom));
        metadata.insert("bom_preserved".to_string(), serde_json::json!(existing_bom && keep_bom));
        metadata.insert("bom_strategy".to_string(), serde_json::json!("writeTextPreservingBom: preserve existing/input UTF-8 BOM and emit at most one BOM"));
        metadata.insert("formatter_status".to_string(), formatter_status);
        metadata.insert("forge_formatter_contract".to_string(), forge_formatter_contract());
        metadata.insert("forge_file_tool_contract".to_string(), forge_file_tool_contract());
        Ok(ToolResult { id: request.id, kind: ToolKind::FileEdit, success: true, output: format!("Edited {}", metadata.get("path").and_then(serde_json::Value::as_str).unwrap_or("file")), error: None, duration_ms: 0, metadata })
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
        ("recovery_hint".to_string(), serde_json::json!("The old_string did not match the current file. Read the current file and retry with the current exact text, or use apply_patch with current context.")),
        ("recommended_next_tools".to_string(), serde_json::json!(["file_read", "file_edit", "apply_patch"])),
        ("forge_tool_failure_lifecycle".to_string(), serde_json::json!("failed file_edit is returned as first-class error state with original input, explicit error, current-file preview, and recovery guidance")),
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
    FormatterSpec { name: "prettier", command: "prettier", args: &["--write"], extensions: &["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts", "html", "htm", "css", "scss", "sass", "less", "vue", "svelte", "json", "jsonc", "yaml", "yml", "toml", "xml", "md", "mdx", "graphql", "gql"] },
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
];

async fn sync_bom_to_file(path: &Path, desired_bom: bool) -> Result<()> { let bytes = fs::read(path).await?; let normalized = normalize_bom_bytes(&bytes, desired_bom); if normalized != bytes { fs::write(path, normalized).await?; } Ok(()) }
fn normalize_bom_bytes(bytes: &[u8], desired_bom: bool) -> Vec<u8> { let mut body = bytes; while body.starts_with(UTF8_BOM_BYTES) { body = &body[UTF8_BOM_BYTES.len()..]; } if desired_bom { let mut result = UTF8_BOM_BYTES.to_vec(); result.extend_from_slice(body); result } else { body.to_vec() } }
fn has_utf8_bom(content: &[u8]) -> bool { content.starts_with(UTF8_BOM_BYTES) }
fn split_bom(text: &str) -> (bool, String) { let stripped = text.trim_start_matches(UTF8_BOM); (stripped.len() != text.len(), stripped.to_string()) }
fn join_bom(text: &str, bom: bool) -> String { let (_, stripped) = split_bom(text); if bom { format!("{UTF8_BOM}{stripped}") } else { stripped } }
fn forge_formatter_contract() -> serde_json::Value { serde_json::json!({"source_backing": ["packages/opencode/src/format/index.ts", "packages/opencode/src/format/formatter.ts"], "behaviors": ["match formatter commands by extension using a catalog", "probe matching formatter commands and safely disable unavailable commands", "run write/edit formatting after mutation when a formatter is available", "contain formatter spawn/status failures in metadata instead of failing the file tool", "resynchronize desired UTF-8 BOM after formatter mutation"]}) }
fn forge_file_tool_contract() -> serde_json::Value { serde_json::json!({"behaviors": ["emit file edit events after write/edit/delete", "emit watcher-style update records", "touch LSP documents", "collect contained diagnostic report envelopes", "preserve an existing/input UTF-8 BOM and emit at most one BOM", "run formatting after write/edit and resynchronize BOM"]}) }
