use crate::graph::KnowledgeGraph;
use crate::extractor::GraphExtractor;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResult {
    pub content: Vec<McpContent>,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: serde_json::Value,
}

pub struct McpServer {
    graph: Option<KnowledgeGraph>,
    tool_definitions: Vec<McpTool>,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            graph: None,
            tool_definitions: vec![
                McpTool {
                    name: "graph_extract".to_string(),
                    description: "Extract code structure from files".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "paths": {
                                "type": "array",
                                "items": {"type": "string"}
                            }
                        },
                        "required": ["paths"]
                    }),
                    output_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "nodes": {"type": "array"},
                            "edges": {"type": "array"}
                        }
                    }),
                },
                McpTool {
                    name: "graph_query".to_string(),
                    description: "Query the knowledge graph".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {"type": "string"}
                        },
                        "required": ["query"]
                    }),
                    output_schema: serde_json::json!({
                        "type": "array",
                        "items": {"type": "object"}
                    }),
                },
                McpTool {
                    name: "graph_visualize".to_string(),
                    description: "Get visualization data for the graph".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "format": {"type": "string", "enum": ["json", "html"]}
                        }
                    }),
                    output_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "data": {"type": "object"},
                            "format": {"type": "string"}
                        }
                    }),
                },
                McpTool {
                    name: "graph_analytics".to_string(),
                    description: "Perform analysis on the graph".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "analysis_type": {
                                "type": "string",
                                "enum": ["statistics", "centrality", "clustering"]
                            }
                        },
                        "required": ["analysis_type"]
                    }),
                    output_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "results": {"type": "object"},
                            "insights": {"type": "array"}
                        }
                    }),
                },
            ],
        }
    }

    pub async fn load_graph(&mut self, extractor: &mut GraphExtractor, paths: &[String]) -> Result<()> {
        let graph = extractor.extract_from_patterns(paths).await?;
        self.graph = Some(graph);
        Ok(())
    }

    pub fn list_tools(&self) -> &[McpTool] {
        &self.tool_definitions
    }

    pub fn process_request(&mut self, request: McpRequest) -> Result<McpToolResult> {
        match request.method.as_str() {
            "graph_extract" => {
                let paths = request.params["paths"].as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                let result_text = if !paths.is_empty() {
                    format!("Would extract {} files", paths.len())
                } else {
                    "No paths provided".to_string()
                };

                Ok(McpToolResult {
                    content: vec![McpContent {
                        r#type: "text".to_string(),
                        text: result_text,
                    }],
                    is_error: false,
                })
            }
            "graph_query" => {
                let query = request.params["query"].as_str().unwrap_or("");
                let mut result_text = format!("Query: {} - found results", query);

                if let Some(filters) = request.params.get("filters") {
                    result_text.push_str(&format!(" with filters: {:?}", filters));
                }

                Ok(McpToolResult {
                    content: vec![McpContent {
                        r#type: "text".to_string(),
                        text: result_text,
                    }],
                    is_error: false,
                })
            }
            _ => Ok(McpToolResult {
                content: vec![McpContent {
                    r#type: "text".to_string(),
                    text: format!("Unknown method: {}", request.method),
                }],
                is_error: true,
            }),
        }
    }
}
