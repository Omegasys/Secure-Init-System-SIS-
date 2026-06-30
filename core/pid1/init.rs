use crate::boot;
use crate::shutdown;

pub struct InitSystem {
    pub state: SystemState,
}

#[derive(Debug)]
pub enum SystemState {
    Booting,
    Running,
    ShuttingDown,
    Failed,
}

impl InitSystem {
    pub fn new() -> Self {
        Self {
            state: SystemState::Booting,
        }
    }

    pub fn boot(&mut self) -> Result<(), String> {
        self.state = SystemState::Booting;

        boot::run_boot_sequence()?;

        self.state = SystemState::Running;
        Ok(())
    }

    pub fn run_event_loop(&mut self) {
        loop {
            // Placeholder for SIS event-driven runtime loop
            // In real system: poll event bus + service manager

            if let SystemState::ShuttingDown = self.state {
                shutdown::execute_shutdown();
                break;
            }

            // simulate idle loop
        }
    }

    pub fn shutdown(&mut self) {
        self.state = SystemState::ShuttingDown;
    }
}
