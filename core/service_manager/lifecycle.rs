use crate::service::{Service, ServiceState};

pub fn start_lifecycle(service: &mut Service) {
    if service.state != ServiceState::Pending {
        return;
    }

    service.state = ServiceState::Starting;

    // Pre-start checks (policy + dependencies)
    if !check_dependencies(service) {
        service.state = ServiceState::Failed;
        return;
    }

    // Simulated start
    service.state = ServiceState::Running;
}

pub fn stop_lifecycle(service: &mut Service) {
    service.state = ServiceState::Stopped;
}

pub fn fail_service(service: &mut Service) {
    service.state = ServiceState::Failed;
    service.crash_count += 1;
}

fn check_dependencies(_service: &Service) -> bool {
    // In real SIS:
    // - dependency engine validation
    // - policy engine checks
    true
}
