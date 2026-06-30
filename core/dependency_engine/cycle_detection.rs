use std::collections::{HashMap, HashSet};
use crate::graph::DependencyGraph;

/// Detects cycles in dependency graph using DFS.
pub fn has_cycle(graph: &DependencyGraph) -> bool {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();

    for node in graph.all_services() {
        if dfs(node, graph, &mut visited, &mut stack) {
            return true;
        }
    }

    false
}

fn dfs(
    node: String,
    graph: &DependencyGraph,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
) -> bool {
    if stack.contains(&node) {
        return true;
    }

    if visited.contains(&node) {
        return false;
    }

    visited.insert(node.clone());
    stack.insert(node.clone());

    if let Some(deps) = graph.get_dependencies(&node) {
        for dep in deps {
            if dfs(dep.clone(), graph, visited, stack) {
                return true;
            }
        }
    }

    stack.remove(&node);
    false
}
