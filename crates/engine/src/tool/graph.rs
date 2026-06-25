//! Graph knowledge tool — builds and queries code knowledge graphs.

use crate::tool::ToolExecutor;
use crate::types::{ToolKind, ToolRequest, ToolResult};
use anyhow::Result;
use std::collections::HashMap;
use serde_json::json;

impl ToolExecutor {
    pub async fn execute_graph_build(&self, request: ToolRequest) -> Result<ToolResult> {
        let pattern = request.args.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("**/crates/**/*.rs");

        let workspace_root = &self.workspace_root;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut file_count = 0;

        let walker = glob::glob(&format!("{}/{}", workspace_root, pattern))?;
        for entry in walker.flatten() {
            if entry.is_file() {
                let path = entry.strip_prefix(workspace_root).unwrap_or(&entry);
                let path_str = path.to_string_lossy().to_string();
                
                if path_str.contains("\\target\\") || path_str.contains("/target/") {
                    continue;
                }
                
                let content = std::fs::read_to_string(&entry).unwrap_or_default();
                let imports = Self::extract_imports(&content);
                
                nodes.push(json!({
                    "id": format!("file:{}", file_count),
                    "label": path_str,
                    "type": "file",
                    "imports": imports,
                    "language": Self::detect_language(&path_str),
                }));
                
                for imp in &imports {
                    edges.push(json!({
                        "source": format!("file:{}", file_count),
                        "target": format!("import:{}", imp),
                        "type": "imports",
                    }));
                }
                
                file_count += 1;
            }
        }

        let graph = json!({
            "nodes": nodes,
            "edges": edges,
            "stats": {
                "files": file_count,
                "nodes": nodes.len(),
                "edges": edges.len(),
            }
        });

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::GraphBuild,
            success: true,
            output: serde_json::to_string(&graph)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::new(),
        })
    }

    pub async fn execute_graph_query(&self, request: ToolRequest) -> Result<ToolResult> {
        let graph_json = request.args.get("graph_json")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing graph_json parameter"))?;

        let query = request.args.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let graph: serde_json::Value = serde_json::from_str(graph_json)?;
        let query_lower = query.to_lowercase();

        let mut results = Vec::new();
        
        if let Some(nodes) = graph["nodes"].as_array() {
            for node in nodes {
                let label = node["label"].as_str().unwrap_or("");
                let node_type = node["type"].as_str().unwrap_or("");
                let imports = node["imports"].as_array().map(|a| {
                    a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ")
                }).unwrap_or_default();

                if label.to_lowercase().contains(&query_lower)
                    || imports.to_lowercase().contains(&query_lower)
                    || node_type.to_lowercase().contains(&query_lower)
                {
                    results.push(json!({
                        "id": node["id"],
                        "label": label,
                        "type": node_type,
                        "imports": imports,
                    }));
                }
            }
        }

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::GraphQuery,
            success: true,
            output: serde_json::to_string(&json!({
                "query": query,
                "matches": results.len(),
                "results": results,
            }))?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::new(),
        })
    }

    fn extract_imports(content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("use ") || line.starts_with("mod ") || line.starts_with("extern crate ") {
                imports.push(line.to_string());
            }
        }
        imports
    }

    fn detect_language(path: &str) -> String {
        if path.ends_with(".rs") { "rust".to_string() }
        else if path.ends_with(".js") || path.ends_with(".ts") { "javascript".to_string() }
        else if path.ends_with(".py") { "python".to_string() }
        else if path.ends_with(".go") { "go".to_string() }
        else if path.ends_with(".java") { "java".to_string() }
        else if path.ends_with(".cpp") || path.ends_with(".cc") || path.ends_with(".cxx") { "cpp".to_string() }
        else if path.ends_with(".c") || path.ends_with(".h") { "c".to_string() }
        else { "unknown".to_string() }
    }
}