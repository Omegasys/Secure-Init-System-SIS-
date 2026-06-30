use std::collections::{HashMap, HashSet};
use crate::graph::DependencyGraph;

/// Resolves a valid startup order using topological sorting.
pub fn resolve_order(graph: &DependencyGraph) -> Result<Vec<String>, String> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for node in graph.all_services() {
        if !visited.contains(&node) {
            visit(node, graph, &mut visited, &mut stack)?;
        }
    }

    Ok(stack)
}

fn visit(
    node: String,
    graph: &DependencyGraph,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
) -> Result<(), String> {
    if visited.contains(&node) {
        return Ok(());
    }

    visited.insert(node.clone());

    if let Some(deps) = graph.get_dependencies(&node) {
        for dep in deps {
            visit(dep.clone(), graph, visited, stack)?;
        }
    }

    stack.push(node);
    Ok(())
}
