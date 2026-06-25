use crate::graph::KnowledgeGraph;
use anyhow::Result;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct PathFinder {
}

impl PathFinder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_all_paths(
        &self,
        start_id: &str,
        end_id: &str,
        graph: &KnowledgeGraph,
    ) -> Result<Vec<Vec<String>>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = Vec::new();

        self.dfs_find_paths(
            start_id,
            end_id,
            graph,
            &mut visited,
            &mut current_path,
            &mut paths,
        );

        Ok(paths)
    }

    pub fn find_shortest_path(
        &self,
        start_id: &str,
        end_id: &str,
        graph: &KnowledgeGraph,
    ) -> Result<Option<Vec<String>>> {
        let mut distances: HashMap<String, usize> = HashMap::new();
        let mut previous: HashMap<String, String> = HashMap::new();
        let mut queue: BinaryHeap<(usize, String)> = BinaryHeap::new();

        for node in &graph.nodes {
            distances.insert(node.id.clone(), usize::MAX);
        }

        distances.insert(start_id.to_string(), 0);
        queue.push((0, start_id.to_string()));

        while let Some((current_distance, current_node)) = queue.pop() {
            if current_node == end_id {
                break;
            }

            if current_distance > distances.get(&current_node).copied().unwrap_or(usize::MAX) {
                continue;
            }

            for edge in &graph.edges {
                if edge.from_node_id == current_node {
                    let neighbor = &edge.to_node_id;
                    let new_distance = current_distance + 1;

                    if new_distance < distances.get(neighbor).copied().unwrap_or(usize::MAX) {
                        distances.insert(neighbor.clone(), new_distance);
                        previous.insert(neighbor.clone(), current_node.clone());
                        queue.push((new_distance, neighbor.clone()));
                    }
                }
            }
        }

        if !previous.contains_key(end_id) && start_id != end_id {
            return Ok(None);
        }

        let mut path = Vec::new();
        let mut current = end_id.to_string();

        while let Some(prev) = previous.get(&current) {
            path.insert(0, current.clone());
            current = prev.clone();
            if current == start_id {
                path.insert(0, current);
                break;
            }
        }

        if path.is_empty() && start_id == end_id {
            path.push(start_id.to_string());
        }

        if !path.is_empty() {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    pub fn find_connected_components(&self, graph: &KnowledgeGraph) -> Result<Vec<Vec<String>>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in &graph.nodes {
            if !visited.contains(&node.id) {
                let mut component = Vec::new();
                self.dfs_collect_component(
                    node.id.clone(),
                    graph,
                    &mut visited,
                    &mut component,
                );
                components.push(component);
            }
        }

        Ok(components)
    }

    fn dfs_find_paths(
        &self,
        current: &str,
        end: &str,
        graph: &KnowledgeGraph,
        visited: &mut HashSet<String>,
        current_path: &mut Vec<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        visited.insert(current.to_string());
        current_path.push(current.to_string());

        if current == end {
            paths.push(current_path.clone());
        } else {
            for edge in &graph.edges {
                if edge.from_node_id == current && !visited.contains(&edge.to_node_id) {
                    self.dfs_find_paths(
                        &edge.to_node_id,
                        end,
                        graph,
                        visited,
                        current_path,
                        paths,
                    );
                }
            }
        }

        visited.remove(current);
        current_path.pop();
    }

    fn dfs_collect_component(
        &self,
        current: String,
        graph: &KnowledgeGraph,
        visited: &mut HashSet<String>,
        component: &mut Vec<String>,
    ) {
        visited.insert(current.clone());
        component.push(current.clone());

        for edge in &graph.edges {
            if edge.from_node_id == current && !visited.contains(&edge.to_node_id) {
                self.dfs_collect_component(edge.to_node_id.clone(), graph, visited, component);
            }
            if edge.to_node_id == current && !visited.contains(&edge.from_node_id) {
                self.dfs_collect_component(edge.from_node_id.clone(), graph, visited, component);
            }
        }
    }
}
