use crate::graph::{KnowledgeGraph, GraphNode, GraphEdge, NodeType, EdgeType};
use anyhow::Result;
use glob::glob;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;
use tree_sitter::{Parser, TreeCursor, Language};

pub struct GraphExtractor {
    symbol_table: HashMap<String, String>,
    parser: Parser,
}

impl GraphExtractor {
    pub fn new() -> Self {
        use tree_sitter_rust::LANGUAGE;
        static LANGUAGE_INSTANCE: OnceLock<Language> = OnceLock::new();
        let language = LANGUAGE_INSTANCE.get_or_init(|| Language::from(LANGUAGE));
        let mut parser = Parser::new();
        parser.set_language(language).ok();
        Self {
            symbol_table: HashMap::new(),
            parser,
        }
    }

    pub async fn extract_file(&mut self, path: &str) -> Result<KnowledgeGraph> {
        let content = tokio::fs::read_to_string(path).await?;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        self.extract_from_content(path, &content, &mut nodes, &mut edges);

        Ok(KnowledgeGraph {
            nodes,
            edges,
            file_path: Some(path.to_string()),
        })
    }

    pub async fn extract_from_patterns(&mut self, patterns: &[String]) -> Result<KnowledgeGraph> {
        let mut combined = KnowledgeGraph::new();

        for pattern in patterns {
            if pattern.contains('*') || pattern.contains('?') {
                let matches = glob(pattern).map_err(|e| anyhow::anyhow!("Invalid glob pattern: {}", e))?;
                for entry in matches.flatten() {
                    let file_path = entry.as_path();
                        if self.should_include_file(file_path) {
                        let graph = self.extract_file(&file_path.to_string_lossy()).await?;
                        combined.merge(graph);
                    }
                }
            } else {
                let path = Path::new(pattern);
                if path.exists() {
                    if path.is_dir() {
                        let files = self.find_files_in_directory(&path, 0).await?;
                        for file_path in files {
                            let graph = self.extract_file(&file_path).await?;
                            combined.merge(graph);
                        }
                    } else if self.should_include_file(path) {
                        let graph = self.extract_file(pattern).await?;
                        combined.merge(graph);
                    }
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

            if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        files.extend(self.find_files_in_directory(&path, current_depth + 1).await?);
                    } else if self.should_include_file(&path) {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }

            Ok(files)
        }).await
    }

    fn should_include_file(&self, path: &Path) -> bool {
        path.extension().and_then(|e| e.to_str()).map_or(false, |ext| {
            matches!(
                ext.to_lowercase().as_str(),
                "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "go" | "java"
                | "cpp" | "cc" | "cxx" | "c" | "h" | "hpp" | "hh" | "cs"
                | "kt" | "scala" | "php" | "swift" | "lua" | "luau" | "zig"
                | "ps1" | "psm1" | "rb" | "d" | "dart"
            )
        })
    }

    fn extract_from_content(
        &mut self,
        file_path: &str,
        content: &str,
        nodes: &mut Vec<GraphNode>,
        edges: &mut Vec<GraphEdge>,
    ) {
        let tree = match self.parser.parse(content, None) {
            Some(t) => t,
            None => return,
        };

        self.walk_tree(file_path, content, &mut tree.walk(), nodes, edges);
    }

    fn walk_tree(
        &mut self,
        file_path: &str,
        content: &str,
        cursor: &mut TreeCursor,
        nodes: &mut Vec<GraphNode>,
        edges: &mut Vec<GraphEdge>,
    ) {
        let kind = cursor.node().kind();

        match kind {
            "function_item" => {
                if let Some(graph_node) = self.extract_function_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "struct_item" => {
                if let Some(graph_node) = self.extract_struct_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "enum_item" => {
                if let Some(graph_node) = self.extract_enum_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "trait_item" => {
                if let Some(graph_node) = self.extract_trait_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "use_declaration" => {
                if let Some(graph_node) = self.extract_use_declaration(content, cursor, file_path) {
                    nodes.push(graph_node.clone());
                    edges.push(GraphEdge {
                        id: uuid::Uuid::new_v4().to_string(),
                        edge_type: EdgeType::Import,
                        from_node_id: String::new(),
                        to_node_id: graph_node.name.clone(),
                        metadata: None,
                    });
                }
            }
            "const_item" => {
                if let Some(graph_node) = self.extract_const_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "mod_item" => {
                if let Some(graph_node) = self.extract_mod_item(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "type_alias" => {
                if let Some(graph_node) = self.extract_type_alias(content, cursor, file_path) {
                    self.symbol_table.insert(graph_node.name.clone(), file_path.to_string());
                    nodes.push(graph_node);
                }
            }
            "impl_item" => {
                if let Some(graph_node) = self.extract_impl_item(content, cursor, file_path) {
                    nodes.push(graph_node);
                }
            }
            _ => {}
        }

        if cursor.goto_first_child() {
            loop {
                self.walk_tree(file_path, content, cursor, nodes, edges);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    fn extract_function_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "identifier")
            .or_else(|| Some("anonymous".to_string()));

        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Function,
            name: name.unwrap_or_default(),
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_struct_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "type_identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Struct,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_enum_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "type_identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Enum,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_trait_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "type_identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Trait,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_use_declaration(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();
        let name = node.utf8_text(content.as_bytes()).unwrap_or("").trim().to_string();
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Import,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: None,
        })
    }

    fn extract_const_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Constant,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_mod_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Module,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_type_alias(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "type_identifier")?;
        let visibility = self.get_visibility(content, cursor);
        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::TypeAlias,
            name,
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: Some(serde_json::json!({ "visibility": visibility })),
        })
    }

    fn extract_impl_item(&self, content: &str, cursor: &mut TreeCursor, file_path: &str) -> Option<GraphNode> {
        let node = cursor.node();
        let start = node.start_position();

        let name = self.child_text_by_kind(content, cursor, "type_identifier")
            .or_else(|| self.child_text_by_kind(content, cursor, "identifier"))
            .unwrap_or_else(|| "impl".to_string());

        let content_line = content.lines().nth(start.row).unwrap_or("").trim().to_string();

        Some(GraphNode {
            id: uuid::Uuid::new_v4().to_string(),
            node_type: NodeType::Struct,
            name: format!("impl<{}>", name),
            file_path: file_path.to_string(),
            line: Some(start.row as u32 + 1),
            column: Some(start.column as u32),
            content: content_line,
            metadata: None,
        })
    }

    fn child_text_by_kind(&self, content: &str, cursor: &mut TreeCursor, kind: &str) -> Option<String> {
        if !cursor.goto_first_child() {
            return None;
        }

        loop {
            let child = cursor.node();
            if child.kind() == kind {
                let text = child.utf8_text(content.as_bytes()).unwrap_or("").to_string();
                cursor.goto_parent();
                return Some(text);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
        None
    }

    fn get_visibility(&self, content: &str, cursor: &mut TreeCursor) -> String {
        if !cursor.goto_first_child() {
            return "private".to_string();
        }

        loop {
            let child = cursor.node();
            if child.kind() == "visibility_modifier" {
                let text = child.utf8_text(content.as_bytes()).unwrap_or("").to_string();
                cursor.goto_parent();
                return text;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
        "private".to_string()
    }
}

impl Default for GraphExtractor {
    fn default() -> Self {
        Self::new()
    }
}