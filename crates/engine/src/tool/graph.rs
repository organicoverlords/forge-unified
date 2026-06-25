//! Graph knowledge tool — builds and queries code knowledge graphs using unifiedgraph.

use crate::tool::ToolExecutor;
use crate::types::{ToolKind, ToolRequest, ToolResult};
use anyhow::Result;
use std::collections::HashMap;
use unifiedgraph::{GraphExtractor, KnowledgeGraph, GraphQuery};

impl ToolExecutor {
    pub async fn execute_graph_build(&self, request: ToolRequest) -> Result<ToolResult> {
        let pattern = request.args.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("**/crates/**/*.rs");

        let workspace_root = &self.workspace_root;

        let mut extractor = GraphExtractor::new();

        let full_pattern = if pattern.starts_with('/') || pattern.contains(':') {
            pattern.to_string()
        } else {
            format!("{}/{}", workspace_root, pattern)
        };

        let graph = extractor.extract_from_patterns(&[full_pattern]).await?;

        let mut nodes: Vec<serde_json::Value> = Vec::new();
        let mut edges: Vec<serde_json::Value> = Vec::new();

        for node in &graph.nodes {
            nodes.push(serde_json::json!({
                "id": node.id,
                "label": node.name,
                "type": node.node_type.as_str(),
                "file": node.file_path,
                "line": node.line,
                "content": node.content,
            }));
        }

        for edge in &graph.edges {
            let source = if edge.from_node_id.is_empty() { "import".to_string() } else { edge.from_node_id.clone() };
            let target = if edge.to_node_id.is_empty() { "unknown".to_string() } else { edge.to_node_id.clone() };

            edges.push(serde_json::json!({
                "source": source,
                "target": target,
                "type": edge.edge_type.as_str(),
            }));
        }

        let graph_json = serde_json::json!({
            "nodes": nodes,
            "edges": edges,
            "stats": {
                "files": graph.nodes.iter().map(|n| &n.file_path).collect::<std::collections::HashSet<_>>().len(),
                "nodes": graph.nodes.len(),
                "edges": graph.edges.len(),
            }
        });

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::GraphBuild,
            success: true,
            output: serde_json::to_string(&graph_json)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::new(),
        })
    }

    pub async fn execute_graph_query(&self, request: ToolRequest) -> Result<ToolResult> {
        let graph_json_str = request.args.get("graph_json")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing graph_json parameter"))?;

        let query = request.args.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let graph: KnowledgeGraph = serde_json::from_str(graph_json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse graph JSON: {}", e))?;

        let gq = GraphQuery::new(graph);

        let results: Vec<_> = gq.find_by_name(query).iter()
            .map(|n| {
                serde_json::json!({
                    "id": n.id,
                    "name": n.name,
                    "type": n.node_type.as_str(),
                    "file": n.file_path,
                    "line": n.line,
                })
            })
            .collect();

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::GraphQuery,
            success: true,
            output: serde_json::to_string(&serde_json::json!({
                "query": query,
                "matches": results.len(),
                "results": results,
            }))?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::new(),
        })
    }
}