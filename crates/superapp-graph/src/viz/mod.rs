use crate::graph::{KnowledgeGraph, GraphNode, GraphEdge, NodeType, EdgeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    pub file_path: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub color: String,
    pub size: f32,
    pub shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub color: String,
    pub width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
    pub groups: Option<Vec<VisualizationGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationGroup {
    pub id: String,
    pub label: String,
    pub nodes: Vec<String>,
    pub color: String,
}

impl From<GraphNode> for VisualizationNode {
    fn from(node: GraphNode) -> Self {
        let (color, size, shape) = match node.node_type {
            NodeType::Module => ("#3498db".to_string(), 15.0, "box".to_string()),
            NodeType::Function => ("#2ecc71".to_string(), 10.0, "ellipse".to_string()),
            NodeType::Struct => ("#f39c12".to_string(), 12.0, "box".to_string()),
            NodeType::Enum => ("#9b59b6".to_string(), 12.0, "diamond".to_string()),
            NodeType::Trait => ("#e67e22".to_string(), 12.0, "ellipse".to_string()),
            NodeType::Import => ("#95a5a6".to_string(), 8.0, "dot".to_string()),
            NodeType::Constant => ("#e74c3c".to_string(), 10.0, "ellipse".to_string()),
            NodeType::TypeAlias => ("#34495e".to_string(), 10.0, "ellipse".to_string()),
            NodeType::Field => ("#d35400".to_string(), 8.0, "dot".to_string()),
            NodeType::Method => ("#27ae60".to_string(), 9.0, "ellipse".to_string()),
            NodeType::Variable => ("#7f8c8d".to_string(), 7.0, "dot".to_string()),
        };

        VisualizationNode {
            id: node.id,
            label: node.name,
            node_type: node.node_type.as_str().to_string(),
            file_path: node.file_path,
            line_start: node.line,
            line_end: node.line,
            color,
            size,
            shape,
        }
    }
}

impl From<GraphEdge> for VisualizationEdge {
    fn from(edge: GraphEdge) -> Self {
        let (color, width) = match edge.edge_type {
            EdgeType::Import => ("#95a5a6".to_string(), 1.5),
            EdgeType::Call => ("#3498db".to_string(), 2.0),
            EdgeType::Reference => ("#e67e22".to_string(), 1.0),
            EdgeType::Implements => ("#9b59b6".to_string(), 2.5),
            EdgeType::Inherits => ("#e74c3c".to_string(), 3.0),
            EdgeType::UsedBy => ("#2ecc71".to_string(), 2.0),
            EdgeType::DefinedIn => ("#f39c12".to_string(), 1.5),
        };

        VisualizationEdge {
            id: edge.id,
            from: edge.from_node_id,
            to: edge.to_node_id,
            edge_type: edge.edge_type.as_str().to_string(),
            color,
            width,
        }
    }
}

pub fn generate_visualization_data(graph: KnowledgeGraph) -> VisualizationData {
    let nodes: Vec<VisualizationNode> = graph.nodes.into_iter().map(|n| n.into()).collect();
    let edges: Vec<VisualizationEdge> = graph.edges.into_iter().map(|e| e.into()).collect();

    VisualizationData {
        nodes,
        edges,
        groups: None,
    }
}
