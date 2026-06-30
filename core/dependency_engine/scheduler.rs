use crate::graph::DependencyGraph;
use crate::resolver::resolve_order;
use crate::cycle_detection::has_cycle;

/// Schedules service startup order based on dependency graph.
pub struct Scheduler;

impl Scheduler {
    pub fn schedule(graph: &DependencyGraph) -> Result<Vec<String>, String> {
        if has_cycle(graph) {
            return Err("Dependency cycle detected".to_string());
        }

        let order = resolve_order(graph)?;
        Ok(order)
    }
}
