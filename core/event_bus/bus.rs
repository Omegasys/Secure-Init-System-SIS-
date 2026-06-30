use std::sync::mpsc::{Sender, Receiver, channel};
use crate::event::Event;

/// Central event bus (simplified SIS version)
pub struct EventBus {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        Self { sender, receiver }
    }

    pub fn emit(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn poll(&self) -> Option<Event> {
        self.receiver.try_recv().ok()
    }

    pub fn receiver(&self) -> &Receiver<Event> {
        &self.receiver
    }
}
