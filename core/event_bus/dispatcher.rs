use crate::event::Event;
use crate::handlers::handle_event;

/// Dispatches events to the correct handler.
pub fn dispatch(event: Event) {
    match event {
        Event::UsbAttach => handle_event("usb.attach"),
        Event::UsbDetach => handle_event("usb.detach"),
        Event::NetUp => handle_event("net.up"),
        Event::NetDown => handle_event("net.down"),
        Event::ServiceStart(s) => handle_event(&format!("service.start:{}", s)),
        Event::ServiceFail(s) => handle_event(&format!("service.fail:{}", s)),
        Event::ServiceExit(s) => handle_event(&format!("service.exit:{}", s)),
        Event::ModuleLoad(m) => handle_event(&format!("module.load:{}", m)),
        Event::PolicyViolation(p) => handle_event(&format!("policy.violation:{}", p)),
        Event::BootStageCompleted(stage) => {
            handle_event(&format!("boot.stage.completed:{}", stage))
        }
    }
}
