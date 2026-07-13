use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DomainEvent` usado por el ejemplo para expresar el dominio del patron.
pub struct DomainEvent {
    name: String,
    aggregate_id: String,
}

impl DomainEvent {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, aggregate_id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aggregate_id: aggregate_id.into(),
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `aggregate id` dentro del ejemplo del patron.
    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }
}

#[derive(Debug, Default)]
/// Tipo publico `EventFanout` usado por el ejemplo para expresar el dominio del patron.
pub struct EventFanout {
    subscribers: Vec<Sender<DomainEvent>>,
}

impl EventFanout {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `subscribe` dentro del ejemplo del patron.
    pub fn subscribe(&mut self, name: impl Into<String>) -> EventInbox {
        let (sender, receiver) = mpsc::channel();
        self.subscribers.push(sender);

        EventInbox {
            name: name.into(),
            receiver,
        }
    }

    /// Modela la operacion `publish` dentro del ejemplo del patron.
    pub fn publish(&self, event: DomainEvent) {
        for subscriber in &self.subscribers {
            let _ = subscriber.send(event.clone());
        }
    }
}

#[derive(Debug)]
/// Tipo publico `EventInbox` usado por el ejemplo para expresar el dominio del patron.
pub struct EventInbox {
    name: String,
    receiver: Receiver<DomainEvent>,
}

impl EventInbox {
    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `drain` dentro del ejemplo del patron.
    pub fn drain(&self) -> Vec<DomainEvent> {
        self.receiver.try_iter().collect()
    }
}
