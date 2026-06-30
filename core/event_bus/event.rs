#[derive(Debug, Clone)]
pub enum Event {
    UsbAttach,
    UsbDetach,
    NetUp,
    NetDown,
    ServiceStart(String),
    ServiceFail(String),
    ServiceExit(String),
    ModuleLoad(String),
    PolicyViolation(String),
    BootStageCompleted(String),
}
