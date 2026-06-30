use std::collections::{HashMap, HashSet};

pub type ServiceName = String;

/// Directed dependency graph:
/// key = service
/// value = services it depends on
pub struct DependencyGraph {
    pub edges: HashMap<ServiceName, Vec<ServiceName>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service: ServiceName) {
        self.edges.entry(service).or_insert(vec![]);
    }

    pub fn add_dependency(&mut self, service: ServiceName, depends_on: ServiceName) {
        self.edges
            .entry(service)
            .or_insert(vec![])
            .push(depends_on);
    }

    pub fn get_dependencies(&self, service: &str) -> Option<&Vec<ServiceName>> {
        self.edges.get(service)
    }

    pub fn all_services(&self) -> Vec<ServiceName> {
        self.edges.keys().cloned().collect()
    }
}
