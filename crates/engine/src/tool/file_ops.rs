//! File operation tools.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolKind};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

impl ToolExecutor {
    pub async fn execute_file_read(&self, request: ToolRequest) -> Result<ToolResult> {
        let path: String = serde_json::from_value(request.args.clone())?;
        let full_path = self.resolve_path(&path)?;
        
        let content = fs::read_to_string(&full_path).await
            .with_context(|| format!("Failed to read file: {}", full_path.display()))?;
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
            ]),
        })
    }

    pub async fn execute_file_write(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { path: String, content: String }
        let args: Args = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&args.path)?;
        
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::write(&full_path, &args.content).await
            .with_context(|| format!("Failed to write file: {}", full_path.display()))?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileWrite,
            success: true,
            output: format!("Written {} bytes to {}", args.content.len(), args.path),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("path".to_string(), serde_json::json!(args.path)),
                ("bytes".to_string(), serde_json::json!(args.content.len())),
            ]),
        })
    }

    pub async fn execute_file_edit(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { path: String, old_string: String, new_string: String, replace_all: Option<bool> }
        let args: Args = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&args.path)?;
        
        let content = fs::read_to_string(&full_path).await?;
        let replace_all = args.replace_all.unwrap_or(false);
        
        let new_content = if replace_all {
            content.replace(&args.old_string, &args.new_string)
        } else {
            content.replacen(&args.old_string, &args.new_string, 1)
        };
        
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
        
        fs::write(&full_path, new_content).await?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileEdit,
            success: true,
            output: format!("Edited {}", args.path),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("path".to_string(), serde_json::json!(args.path)),
                ("replacements".to_string(), serde_json::json!(if replace_all { "all" } else { "first" })),
            ]),
        })
    }

    pub async fn execute_file_delete(&self, request: ToolRequest) -> Result<ToolResult> {
        let path: String = serde_json::from_value(request.args)?;
        let full_path = self.resolve_path(&path)?;
        
        fs::remove_file(&full_path).await
            .with_context(|| format!("Failed to delete file: {}", full_path.display()))?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::FileDelete,
            success: true,
            output: format!("Deleted {}", path),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([("path".to_string(), serde_json::json!(path))]),
        })
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
        for entry in glob::glob(full_path.join(&pattern).to_str().unwrap())? {
            matches.push(entry?.to_string_lossy().to_string());
        }
        
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
                            results.push(serde_json::json!({
                                "file": entry.to_string_lossy(),
                                "line": line_num + 1,
                                "content": line.trim(),
                            }));
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
        let full = if path.is_absolute() {
            path.to_path_buf()
        } else {
            Path::new(&self.workspace_root).join(path)
        };
        
        let canonical = full.canonicalize()
            .or_else(|_| Ok::<PathBuf, anyhow::Error>(full.clone()))?;
        
        let workspace_canonical = Path::new(&self.workspace_root).canonicalize()?;
        if !canonical.starts_with(&workspace_canonical) {
            anyhow::bail!("Path escapes workspace: {}", path.display());
        }
        
        Ok(canonical)
    }

    async fn list_dir_recursive(
        &self, 
        root: &Path, 
        rel: &Path, 
        depth: usize, 
        max_depth: usize,
        entries: &mut Vec<serde_json::Value>
    ) -> Result<()> {
        if depth > max_depth { return Ok(()); }
        
        let mut dir = fs::read_dir(root).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let rel_path = rel.join(entry.file_name());
            
            let is_dir = path.is_dir();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if is_dir && (name == ".git" || name == "target" || name == "node_modules") {
                continue;
            }
            
            entries.push(serde_json::json!({
                "name": name,
                "path": rel_path.to_string_lossy(),
                "is_dir": is_dir,
            }));
            
            if is_dir {
                let fut = Box::pin(self.list_dir_recursive(&path, &rel_path, depth + 1, max_depth, entries));
                fut.await?;
            }
        }
        Ok(())
    }
}