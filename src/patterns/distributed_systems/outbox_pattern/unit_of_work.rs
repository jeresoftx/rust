#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: String,
    customer_id: String,
}

impl Order {
    pub fn new(id: impl Into<String>, customer_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxMessage {
    id: String,
    event_type: String,
    aggregate_id: String,
    sent: bool,
}

impl OutboxMessage {
    pub fn new(
        id: impl Into<String>,
        event_type: impl Into<String>,
        aggregate_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            event_type: event_type.into(),
            aggregate_id: aggregate_id.into(),
            sent: false,
        }
    }

    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }
}

#[derive(Debug, Default)]
pub struct InMemoryUnitOfWork {
    orders: Vec<Order>,
    messages: Vec<OutboxMessage>,
}

impl InMemoryUnitOfWork {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_order(&mut self, id: &str, customer_id: &str) {
        let message_id = format!("msg-{}", self.messages.len() + 1);
        self.orders.push(Order::new(id, customer_id));
        self.messages
            .push(OutboxMessage::new(message_id, "OrderCreated", id));
    }

    pub fn create_order_with_failure(&mut self, id: &str, customer_id: &str) -> Result<(), String> {
        let order = Order::new(id, customer_id);
        let message = OutboxMessage::new("msg-failed", "OrderCreated", id);
        let _pending = (order, message);
        Err("transaction aborted".to_string())
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn pending_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| !message.sent)
            .cloned()
            .collect()
    }
}
