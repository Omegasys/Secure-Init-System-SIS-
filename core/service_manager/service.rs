#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub command: String,
    pub dependencies: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub state: ServiceState,
    pub crash_count: u32,
}

#[derive(Debug, Clone)]
pub enum ServiceState {
    Pending,
    Starting,
    Running,
    Failed,
    Stopped,
    Quarantined,
}

#[derive(Debug, Clone)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
    OnCrashLimit(u32),
}

impl Service {
    pub fn new(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            dependencies: vec![],
            restart_policy: RestartPolicy::OnFailure,
            state: ServiceState::Pending,
            crash_count: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, ServiceState::Running)
    }
}
