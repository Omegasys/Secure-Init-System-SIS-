use std::collections::HashMap;
use crate::service::{Service, ServiceState};

pub struct ServiceManager {
    pub services: HashMap<String, Service>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or("Service not found")?;

        service.state = ServiceState::Starting;

        // In real SIS:
        // - spawn process
        // - attach supervision
        // - enforce policy checks

        service.state = ServiceState::Running;
        Ok(())
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name)
            .ok_or("Service not found")?;

        service.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }
}
