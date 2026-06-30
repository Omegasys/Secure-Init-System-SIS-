mod init;
mod boot;
mod shutdown;
mod recovery;
mod panic;

use init::InitSystem;

fn main() {
    // SIS PID 1 entry point
    let mut system = InitSystem::new();

    if let Err(e) = system.boot() {
        eprintln!("[SIS PID1] Boot failure: {:?}", e);
        recovery::enter_recovery_mode();
    }

    system.run_event_loop();
}
