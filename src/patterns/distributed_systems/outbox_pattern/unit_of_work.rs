#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    customer_id: String,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, customer_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OutboxMessage` usado por el ejemplo para expresar el dominio del patron.
pub struct OutboxMessage {
    id: String,
    event_type: String,
    aggregate_id: String,
    sent: bool,
}

impl OutboxMessage {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Modela la operacion `aggregate id` dentro del ejemplo del patron.
    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }
}

#[derive(Debug, Default)]
/// Tipo publico `InMemoryUnitOfWork` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryUnitOfWork {
    orders: Vec<Order>,
    messages: Vec<OutboxMessage>,
}

impl InMemoryUnitOfWork {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `create order` dentro del ejemplo del patron.
    pub fn create_order(&mut self, id: &str, customer_id: &str) {
        let message_id = format!("msg-{}", self.messages.len() + 1);
        self.orders.push(Order::new(id, customer_id));
        self.messages
            .push(OutboxMessage::new(message_id, "OrderCreated", id));
    }

    /// Modela la operacion `create order with failure` dentro del ejemplo del patron.
    pub fn create_order_with_failure(&mut self, id: &str, customer_id: &str) -> Result<(), String> {
        let order = Order::new(id, customer_id);
        let message = OutboxMessage::new("msg-failed", "OrderCreated", id);
        let _pending = (order, message);
        Err("transaction aborted".to_string())
    }

    /// Modela la operacion `orders` dentro del ejemplo del patron.
    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    /// Modela la operacion `pending messages` dentro del ejemplo del patron.
    pub fn pending_messages(&self) -> Vec<OutboxMessage> {
        self.messages
            .iter()
            .filter(|message| !message.sent)
            .cloned()
            .collect()
    }
}
