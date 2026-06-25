use crate::graph::{KnowledgeGraph, GraphNode, GraphEdge, NodeType, EdgeType};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub struct GraphExtractor {
    symbol_table: HashMap<String, String>,
    current_file: Option<String>,
}

impl GraphExtractor {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            current_file: None,
        }
    }

    pub async fn extract_file(&mut self, path: &str) -> Result<KnowledgeGraph> {
        self.current_file = Some(path.to_string());

        let content = tokio::fs::read_to_string(path).await?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        self.extract_from_content(path, &content, &mut nodes, &mut edges);

        self.current_file = None;

        Ok(KnowledgeGraph {
            nodes,
            edges,
            file_path: Some(path.to_string()),
        })
    }

    pub async fn extract_from_patterns(&mut self, patterns: &[String]) -> Result<KnowledgeGraph> {
        let mut combined = KnowledgeGraph::new();

        for pattern in patterns {
            let path = Path::new(pattern);

            if path.exists() {
                if path.is_dir() {
                    let files = self.find_files_in_directory(&path, 0).await?;
                    for file_path in files {
                        let graph = self.extract_file(&file_path).await?;
                        combined.merge(graph);
                    }
                } else {
                    let graph = self.extract_file(pattern).await?;
                    combined.merge(graph);
                }
            }
        }

        Ok(combined)
    }

    async fn find_files_in_directory(&self, dir: &Path, current_depth: u32) -> Result<Vec<String>> {
        Box::pin(async move {
            if current_depth > 10 {
                return Ok(Vec::new());
            }

            let mut files = Vec::new();

            match tokio::fs::read_dir(dir).await {
                Ok(mut entries) => {
                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();

                        if path.is_dir() {
                            let sub_files = self.find_files_in_directory(&path, current_depth + 1).await?;
                            files.extend(sub_files);
                        } else if self.should_include_file(&path) {
                            files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
                Err(_) => {}
            }

            Ok(files)
        }).await
    }

    fn should_include_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            matches!(
                ext_lower.as_str(),
                "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "go" | "java"
                | "cpp" | "cc" | "cxx" | "c" | "h" | "hpp" | "hh" | "cs"
                | "kt" | "scala" | "php" | "swift" | "lua" | "luau" | "zig"
                | "ps1" | "psm1" | "rb" | "d" | "dart"
            )
        } else {
            false
        }
    }

    fn extract_from_content(
        &mut self,
        file_path: &str,
        content: &str,
        nodes: &mut Vec<GraphNode>,
        edges: &mut Vec<GraphEdge>,
    ) {
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num as u32 + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if let Some(node) = self.extract_node(file_path, line_num, line, trimmed) {
                nodes.push(node);
            }

            if let Some(edge) = self.extract_edge(file_path, line_num, trimmed) {
                edges.push(edge);
            }
        }
    }

    fn extract_node(
        &mut self,
        file_path: &str,
        line_num: u32,
        original_line: &str,
        trimmed: &str,
    ) -> Option<GraphNode> {
        let node_type;
        let name;

        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            node_type = NodeType::Function;
            name = trimmed.split('(').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("fn ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
            node_type = NodeType::Struct;
            name = trimmed.split('{').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("struct ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ") {
            node_type = NodeType::Enum;
            name = trimmed.split('{').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("enum ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
            node_type = NodeType::Trait;
            name = trimmed.split('{').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("trait ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("use ") {
            node_type = NodeType::Import;
            name = trimmed.strip_prefix("use ").unwrap_or(trimmed)
                .split(';').next().unwrap_or("")
                .trim().to_string();
        } else if trimmed.contains("const ") {
            node_type = NodeType::Constant;
            name = trimmed.split('=').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("const ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("pub mod ") || trimmed.starts_with("mod ") {
            node_type = NodeType::Module;
            name = trimmed.split('{').next()
                .map(|s| s.strip_prefix("pub ").unwrap_or(s))
                .map(|s| s.strip_prefix("mod ").unwrap_or(s))
                .unwrap_or("").trim().to_string();
        } else {
            return None;
        }

        if name.is_empty() {
            return None;
        }

        self.symbol_table.insert(name.clone(), file_path.to_string());

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type,
            name,
            file_path: file_path.to_string(),
            line: Some(line_num),
            column: None,
            content: original_line.to_string(),
            metadata: None,
        })
    }

    fn extract_edge(
        &self,
        _file_path: &str,
        _line_num: u32,
        trimmed: &str,
    ) -> Option<GraphEdge> {
        let trimmed_lower = trimmed.to_lowercase();

        if trimmed_lower.starts_with("use ") {
            let import_part = trimmed.strip_prefix("use ").unwrap_or(trimmed);
            let parts: Vec<&str> = import_part.split_whitespace().collect();
            if parts.len() >= 2 {
                return Some(GraphEdge {
                    id: uuid::Uuid::new_v4().to_string(),
                    edge_type: EdgeType::Import,
                    from_node_id: String::new(),
                    to_node_id: parts.join("::"),
                    metadata: None,
                });
            }
        } else if trimmed.contains('.') {
            let parts: Vec<&str> = trimmed.split('.').collect();
            if parts.len() >= 2 {
                let from_name = parts[0].trim().split_whitespace().last().unwrap_or("");
                let to_name = parts[1].trim().split_whitespace().next().unwrap_or("");

                if !from_name.is_empty() && !to_name.is_empty() {
                    return Some(GraphEdge {
                        id: uuid::Uuid::new_v4().to_string(),
                        edge_type: EdgeType::Call,
                        from_node_id: from_name.to_string(),
                        to_node_id: to_name.to_string(),
                        metadata: None,
                    });
                }
            }
        }

        None
    }
}
