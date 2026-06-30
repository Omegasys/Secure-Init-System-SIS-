use crate::service::{Service, RestartPolicy, ServiceState};

pub struct RestartEngine;

impl RestartEngine {
    pub fn evaluate(service: &mut Service) {
        match service.restart_policy {
            RestartPolicy::Never => {}

            RestartPolicy::Always => {
                Self::restart(service);
            }

            RestartPolicy::OnFailure => {
                if matches!(service.state, ServiceState::Failed) {
                    Self::restart(service);
                }
            }

            RestartPolicy::OnCrashLimit(limit) => {
                if service.crash_count < limit {
                    Self::restart(service);
                } else {
                    service.state = ServiceState::Quarantined;
                }
            }
        }
    }

    fn restart(service: &mut Service) {
        service.state = ServiceState::Starting;
        service.state = ServiceState::Running;
    }
}
