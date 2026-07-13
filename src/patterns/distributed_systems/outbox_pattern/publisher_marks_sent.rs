#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OutboxMessage` usado por el ejemplo para expresar el dominio del patron.
pub struct OutboxMessage {
    id: String,
    sent: bool,
}

impl OutboxMessage {
    /// Operacion `pending` definida por la abstraccion del ejemplo.
    fn pending(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sent: false,
        }
    }
}

#[derive(Debug, Default)]
/// Tipo publico `OutboxStore` usado por el ejemplo para expresar el dominio del patron.
pub struct OutboxStore {
    messages: Vec<OutboxMessage>,
}

impl OutboxStore {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `with pending` dentro del ejemplo del patron.
    pub fn with_pending<const N: usize>(ids: [&str; N]) -> Self {
        Self {
            messages: ids.into_iter().map(OutboxMessage::pending).collect(),
        }
    }

    /// Modela la operacion `pending messages` dentro del ejemplo del patron.
    pub fn pending_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| !message.sent)
            .cloned()
            .collect()
    }

    /// Modela la operacion `sent messages` dentro del ejemplo del patron.
    pub fn sent_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| message.sent)
            .cloned()
            .collect()
    }

    /// Operacion `mark sent` definida por la abstraccion del ejemplo.
    fn mark_sent(&mut self, id: &str) {
        if let Some(message) = self.messages.iter_mut().find(|message| message.id == id) {
            message.sent = true;
        }
    }
}

#[derive(Debug, Default)]
/// Tipo publico `FakeBroker` usado por el ejemplo para expresar el dominio del patron.
pub struct FakeBroker {
    published_ids: Vec<String>,
}

impl FakeBroker {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Operacion `publish` definida por la abstraccion del ejemplo.
    fn publish(&mut self, message: &OutboxMessage) {
        self.published_ids.push(message.id.clone());
    }

    /// Modela la operacion `published ids` dentro del ejemplo del patron.
    pub fn published_ids(&self) -> &[String] {
        &self.published_ids
    }
}

/// Tipo publico `OutboxPublisher` usado por el ejemplo para expresar el dominio del patron.
pub struct OutboxPublisher<'a> {
    broker: &'a mut FakeBroker,
}

impl<'a> OutboxPublisher<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(broker: &'a mut FakeBroker) -> Self {
        Self { broker }
    }

    /// Modela la operacion `publish pending` dentro del ejemplo del patron.
    pub fn publish_pending(&mut self, store: &mut OutboxStore) -> usize {
        let pending = store.pending_messages();
        for message in &pending {
            self.broker.publish(message);
            store.mark_sent(&message.id);
        }
        pending.len()
    }
}
