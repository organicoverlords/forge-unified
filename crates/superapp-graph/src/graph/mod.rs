use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Module,
    Function,
    Struct,
    Enum,
    Trait,
    Import,
    Constant,
    TypeAlias,
    Field,
    Method,
    Variable,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Module => "module",
            NodeType::Function => "function",
            NodeType::Struct => "struct",
            NodeType::Enum => "enum",
            NodeType::Trait => "trait",
            NodeType::Import => "import",
            NodeType::Constant => "constant",
            NodeType::TypeAlias => "type_alias",
            NodeType::Field => "field",
            NodeType::Method => "method",
            NodeType::Variable => "variable",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "module" => Some(NodeType::Module),
            "function" => Some(NodeType::Function),
            "struct" => Some(NodeType::Struct),
            "enum" => Some(NodeType::Enum),
            "trait" => Some(NodeType::Trait),
            "import" => Some(NodeType::Import),
            "constant" => Some(NodeType::Constant),
            "type_alias" => Some(NodeType::TypeAlias),
            "field" => Some(NodeType::Field),
            "method" => Some(NodeType::Method),
            "variable" => Some(NodeType::Variable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    Import,
    Call,
    Reference,
    Implements,
    Inherits,
    UsedBy,
    DefinedIn,
}

impl EdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::Import => "import",
            EdgeType::Call => "call",
            EdgeType::Reference => "reference",
            EdgeType::Implements => "implements",
            EdgeType::Inherits => "inherits",
            EdgeType::UsedBy => "used_by",
            EdgeType::DefinedIn => "defined_in",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "import" => Some(EdgeType::Import),
            "call" => Some(EdgeType::Call),
            "reference" => Some(EdgeType::Reference),
            "implements" => Some(EdgeType::Implements),
            "inherits" => Some(EdgeType::Inherits),
            "used_by" => Some(EdgeType::UsedBy),
            "defined_in" => Some(EdgeType::DefinedIn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub node_type: NodeType,
    pub name: String,
    pub file_path: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub edge_type: EdgeType,
    pub from_node_id: String,
    pub to_node_id: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub file_path: Option<String>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn merge(&mut self, other: KnowledgeGraph) {
        self.nodes.extend(other.nodes);
        self.edges.extend(other.edges);
    }
}

pub struct SymbolTable {
    entries: HashMap<String, SymbolEntry>,
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub node_id: String,
    pub file_path: String,
    pub node_type: NodeType,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, node_id: String, file_path: String, node_type: NodeType) {
        self.entries.insert(name.clone(), SymbolEntry {
            name,
            node_id,
            file_path,
            node_type,
        });
    }

    pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
        self.entries.get(name)
    }

    pub fn resolve_references(&self, graph: &KnowledgeGraph) -> KnowledgeGraph {
        let new_nodes = graph.nodes.clone();
        let mut new_edges = Vec::new();

        for edge in &graph.edges {
            if self.get(&edge.from_node_id).is_some() || self.get(&edge.to_node_id).is_some() {
                new_edges.push(GraphEdge {
                    id: edge.id.clone(),
                    edge_type: edge.edge_type.clone(),
                    from_node_id: edge.from_node_id.clone(),
                    to_node_id: edge.to_node_id.clone(),
                    metadata: edge.metadata.clone(),
                });
            }
        }

        KnowledgeGraph {
            nodes: new_nodes,
            edges: new_edges,
            file_path: graph.file_path.clone(),
        }
    }
}

pub struct PathTraverser {
    visited: HashSet<String>,
}

impl PathTraverser {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
        }
    }

    pub fn traverse<F>(&mut self, start_node: &GraphNode, graph: &KnowledgeGraph, depth: u32, callback: &mut F)
    where
        F: FnMut(&GraphNode, &GraphEdge, u32),
    {
        if depth > 10 {
            return;
        }

        let node_key = format!("{}:{}", start_node.file_path, start_node.name);
        if self.visited.contains(&node_key) {
            return;
        }

        self.visited.insert(node_key);

        for edge in &graph.edges {
            if edge.from_node_id == start_node.id {
                if let Some(to_node) = graph.nodes.iter().find(|n| n.id == edge.to_node_id) {
                    callback(to_node, edge, depth + 1);
                    self.traverse(to_node, graph, depth, callback);
                }
            }
        }
    }
}

pub struct GraphQuery {
    graph: KnowledgeGraph,
}

impl GraphQuery {
    pub fn new(graph: KnowledgeGraph) -> Self {
        Self { graph }
    }

    pub fn find_by_type(&self, node_type: &NodeType) -> Vec<&GraphNode> {
        self.graph.nodes.iter()
            .filter(|n| std::mem::discriminant(&n.node_type) == std::mem::discriminant(node_type))
            .collect()
    }

    pub fn find_by_file(&self, file_path: &str) -> Vec<&GraphNode> {
        self.graph.nodes.iter()
            .filter(|n| n.file_path == file_path)
            .collect()
    }

    pub fn find_by_name(&self, pattern: &str) -> Vec<&GraphNode> {
        let pattern_lower = pattern.to_lowercase();
        self.graph.nodes.iter()
            .filter(|n| n.name.to_lowercase().contains(&pattern_lower))
            .collect()
    }

    pub fn count_edges(&self, edge_type: &EdgeType) -> usize {
        self.graph.edges.iter()
            .filter(|e| std::mem::discriminant(&e.edge_type) == std::mem::discriminant(edge_type))
            .count()
    }
}

pub struct GraphOptimizer;

impl GraphOptimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_strongly_connected_components(&self, graph: &KnowledgeGraph) -> Vec<Vec<String>> {
        let mut components: Vec<Vec<String>> = Vec::new();

        for node in &graph.nodes {
            let mut component: Vec<String> = Vec::new();

            for edge in &graph.edges {
                if edge.from_node_id == node.id || edge.to_node_id == node.id {
                    component.push(node.id.clone());
                }
            }

            if !component.is_empty() {
                components.push(component);
            }
        }

        components
    }

    pub fn compute_clustering_coefficients(&self, graph: &KnowledgeGraph) -> HashMap<String, f32> {
        let mut coefficients: HashMap<String, f32> = HashMap::new();

        for node in &graph.nodes {
            let mut total_possible: f32 = 0.0;
            let mut actual_connections: f32 = 0.0;

            for other in &graph.nodes {
                if node.id == other.id {
                    continue;
                }

                total_possible += 1.0;

                let has_edge = graph.edges.iter().any(|e|
                    (e.from_node_id == node.id && e.to_node_id == other.id) ||
                    (e.to_node_id == node.id && e.from_node_id == other.id)
                );

                if has_edge {
                    actual_connections += 1.0;
                }
            }

            let coefficient = if total_possible > 0.0 {
                actual_connections / total_possible
            } else {
                0.0
            };

            coefficients.insert(node.id.clone(), coefficient);
        }

        coefficients
    }
}
