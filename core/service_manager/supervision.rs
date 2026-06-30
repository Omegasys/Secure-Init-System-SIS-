use crate::service::{Service, ServiceState};

pub struct Supervisor;

impl Supervisor {
    pub fn monitor(service: &mut Service) {
        match service.state {
            ServiceState::Running => {
                // health checks would go here
            }

            ServiceState::Failed => {
                Self::handle_failure(service);
            }

            _ => {}
        }
    }

    fn handle_failure(service: &mut Service) {
        service.crash_count += 1;

        // Basic quarantine rule (simplified SIS behavior)
        if service.crash_count > 3 {
            service.state = ServiceState::Quarantined;
            return;
        }

        // Otherwise mark for restart
        service.state = ServiceState::Pending;
    }
}
