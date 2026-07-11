use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderCreated {
    order_id: String,
    customer_id: String,
    total_cents: u32,
}

impl OrderCreated {
    pub fn new(
        order_id: impl Into<String>,
        customer_id: impl Into<String>,
        total_cents: u32,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            customer_id: customer_id.into(),
            total_cents,
        }
    }

    fn as_message(&self) -> String {
        format!(
            "order-created:{}:{}:{}",
            self.order_id, self.customer_id, self.total_cents
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct OrderEventBus {
    subscribers: BTreeMap<String, Vec<String>>,
}

impl OrderEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: BTreeMap::new(),
        }
    }

    pub fn subscribe(&mut self, name: impl Into<String>) {
        self.subscribers.entry(name.into()).or_default();
    }

    pub fn publish(&mut self, event: OrderCreated) {
        let message = event.as_message();
        for inbox in self.subscribers.values_mut() {
            inbox.push(message.clone());
        }
    }

    pub fn received_by(&self, name: &str) -> Vec<String> {
        self.subscribers.get(name).cloned().unwrap_or_default()
    }
}
