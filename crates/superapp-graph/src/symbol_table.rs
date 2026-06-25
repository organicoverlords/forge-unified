use crate::graph::{KnowledgeGraph, NodeType};
use std::collections::{HashMap, HashSet};

pub struct SymbolTable {
    pub entries: HashMap<String, SymbolEntry>,
    pub name_index: HashMap<String, String>,
    pub file_index: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub node_id: String,
    pub file_path: String,
    pub node_type: NodeType,
    pub reference_count: usize,
    pub components: HashSet<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            name_index: HashMap::new(),
            file_index: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        name: String,
        file_path: String,
        node_id: String,
        node_type: NodeType,
    ) {
        let entry = SymbolEntry {
            name: name.clone(),
            node_id: node_id.clone(),
            file_path: file_path.clone(),
            node_type,
            reference_count: 1,
            components: HashSet::new(),
        };

        self.entries.insert(name.clone(), entry);
        self.name_index.insert(name, node_id.clone());

        let file_entries = self.file_index.entry(file_path).or_insert(Vec::new());
        if !file_entries.contains(&node_id) {
            file_entries.push(node_id);
        }
    }

    pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
        self.entries.get(name)
    }

    pub fn get_symbols_in_file(&self, file_path: &str) -> Vec<&SymbolEntry> {
        let node_ids = self.file_index.get(file_path);
        match node_ids {
            Some(ids) => ids.iter()
                .filter_map(|node_id| self.entries.get(node_id))
                .collect(),
            None => vec![],
        }
    }

    pub fn find_similar(&self, pattern: &str) -> Vec<&SymbolEntry> {
        self.entries.values()
            .filter(|entry| entry.name.contains(pattern))
            .collect()
    }

    pub fn resolve_references(&mut self, graph: &KnowledgeGraph) {
        let mut reference_counts: HashMap<String, usize> = HashMap::new();

        for edge in &graph.edges {
            if let Some(entry) = self.entries.get_mut(&edge.to_node_id) {
                *reference_counts.entry(entry.name.clone()).or_insert(0) += 1;
                entry.reference_count = reference_counts[&entry.name];
            }
        }

        let updates: Vec<(String, HashSet<String>)> = self.name_index.iter()
            .filter_map(|(name, node_id)| {
                self.entries.get(name)?;
                let mut components = HashSet::new();
                for edge in &graph.edges {
                    if edge.from_node_id == *node_id || edge.to_node_id == *node_id {
                        if let Some(other) = self.entries.get(&edge.from_node_id) {
                            components.insert(other.name.clone());
                        }
                        if let Some(other) = self.entries.get(&edge.to_node_id) {
                            components.insert(other.name.clone());
                        }
                    }
                }
                Some((name.clone(), components))
            })
            .collect();

        for (name, components) in updates {
            if let Some(entry) = self.entries.get_mut(&name) {
                entry.components = components;
            }
        }
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# Symbol Table Report\n\n");

        report.push_str("## Symbol Summary\n");
        report.push_str("| Name | Type | File | References |\n");
        report.push_str("|------|------|------|------------|\n");

        for entry in self.entries.values() {
            report.push_str(&format!("| {} | {} | {} | {} |\n",
                entry.name, entry.node_type.as_str(), entry.file_path, entry.reference_count));
        }

        report.push_str("\n## File Breakdown\n");

        for (file_path, node_ids) in &self.file_index {
            report.push_str(&format!("### {}\n", file_path));
            report.push_str(&format!("{} symbols\n\n", node_ids.len()));
        }

        report
    }
}
