use crate::graph::KnowledgeGraph;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsManifest {
    pub skills: Vec<SkillDefinition>,
    pub metadata: SkillsMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub triggers: Vec<String>,
    pub parameters: Vec<SkillParameter>,
    pub output_schema: Option<serde_json::Value>,
    pub examples: Vec<SkillExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillType {
    GraphQuery,
    Extract,
    Visualize,
    Export,
    Analyze,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub description: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsMetadata {
    pub version: String,
    pub generated_at: String,
    pub target_agent: String,
    pub compatibility: String,
}

pub struct SkillsGenerator;

impl SkillsGenerator {
    pub fn generate_for_graph(_graph: &KnowledgeGraph) -> Result<SkillsManifest> {
        let skills = vec![
            SkillDefinition {
                name: "graph_extract".to_string(),
                description: "Extract code structure from files into a knowledge graph".to_string(),
                skill_type: SkillType::Extract,
                triggers: vec!["user asked to extract code structure".to_string()],
                parameters: vec![
                    SkillParameter {
                        name: "paths".to_string(),
                        param_type: "array".to_string(),
                        description: "Source files or directories to extract".to_string(),
                        required: true,
                    },
                ],
                output_schema: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "nodes": {"type": "array"},
                        "edges": {"type": "array"},
                        "statistics": {"type": "object"}
                    }
                })),
                examples: vec![],
            },
            SkillDefinition {
                name: "graph_query".to_string(),
                description: "Query the extracted knowledge graph".to_string(),
                skill_type: SkillType::GraphQuery,
                triggers: vec!["user asked to search or analyze code relationships".to_string()],
                parameters: vec![
                    SkillParameter {
                        name: "pattern".to_string(),
                        param_type: "string".to_string(),
                        description: "Pattern to search for".to_string(),
                        required: true,
                    },
                    SkillParameter {
                        name: "node_type".to_string(),
                        param_type: "string".to_string(),
                        description: "Filter by node type (function, struct, etc.)".to_string(),
                        required: false,
                    },
                ],
                output_schema: Some(serde_json::json!({
                    "type": "array",
                    "items": {"type": "object"}
                })),
                examples: vec![],
            },
            SkillDefinition {
                name: "graph_visualize".to_string(),
                description: "Generate interactive visualizations of the knowledge graph".to_string(),
                skill_type: SkillType::Visualize,
                triggers: vec!["user asked to see code relationships visually".to_string()],
                parameters: vec![
                    SkillParameter {
                        name: "format".to_string(),
                        param_type: "string".to_string(),
                        description: "Output format (html, json)".to_string(),
                        required: false,
                    },
                ],
                output_schema: None,
                examples: vec![],
            },
        ];

        let manifest = SkillsManifest {
            skills,
            metadata: SkillsMetadata {
                version: "0.1.0".to_string(),
                generated_at: chrono::Utc::now().to_rfc3339(),
                target_agent: "OpenCode".to_string(),
                compatibility: "Forge Unified v1.0".to_string(),
            },
        };

        Ok(manifest)
    }
}

pub fn generate_opencode_skills(graph: &KnowledgeGraph) -> Result<String> {
    let manifest = SkillsGenerator::generate_for_graph(graph)?;
    let json = serde_json::to_string_pretty(&manifest)?;

    Ok(json)
}
