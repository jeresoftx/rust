#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxMessage {
    id: String,
    sent: bool,
}

impl OutboxMessage {
    fn pending(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sent: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct OutboxStore {
    messages: Vec<OutboxMessage>,
}

impl OutboxStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pending<const N: usize>(ids: [&str; N]) -> Self {
        Self {
            messages: ids.into_iter().map(OutboxMessage::pending).collect(),
        }
    }

    pub fn pending_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| !message.sent)
            .cloned()
            .collect()
    }

    pub fn sent_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| message.sent)
            .cloned()
            .collect()
    }

    fn mark_sent(&mut self, id: &str) {
        if let Some(message) = self.messages.iter_mut().find(|message| message.id == id) {
            message.sent = true;
        }
    }
}

#[derive(Debug, Default)]
pub struct FakeBroker {
    published_ids: Vec<String>,
}

impl FakeBroker {
    pub fn new() -> Self {
        Self::default()
    }

    fn publish(&mut self, message: &OutboxMessage) {
        self.published_ids.push(message.id.clone());
    }

    pub fn published_ids(&self) -> &[String] {
        &self.published_ids
    }
}

pub struct OutboxPublisher<'a> {
    broker: &'a mut FakeBroker,
}

impl<'a> OutboxPublisher<'a> {
    pub fn new(broker: &'a mut FakeBroker) -> Self {
        Self { broker }
    }

    pub fn publish_pending(&mut self, store: &mut OutboxStore) -> usize {
        let pending = store.pending_messages();
        for message in &pending {
            self.broker.publish(message);
            store.mark_sent(&message.id);
        }
        pending.len()
    }
}
