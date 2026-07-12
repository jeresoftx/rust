use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainEvent {
    name: String,
    aggregate_id: String,
}

impl DomainEvent {
    pub fn new(name: impl Into<String>, aggregate_id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aggregate_id: aggregate_id.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }
}

#[derive(Debug, Default)]
pub struct EventFanout {
    subscribers: Vec<Sender<DomainEvent>>,
}

impl EventFanout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&mut self, name: impl Into<String>) -> EventInbox {
        let (sender, receiver) = mpsc::channel();
        self.subscribers.push(sender);

        EventInbox {
            name: name.into(),
            receiver,
        }
    }

    pub fn publish(&self, event: DomainEvent) {
        for subscriber in &self.subscribers {
            let _ = subscriber.send(event.clone());
        }
    }
}

#[derive(Debug)]
pub struct EventInbox {
    name: String,
    receiver: Receiver<DomainEvent>,
}

impl EventInbox {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn drain(&self) -> Vec<DomainEvent> {
        self.receiver.try_iter().collect()
    }
}
