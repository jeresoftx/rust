#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderId(String);

impl OrderId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderConfirmed {
    order_id: String,
    item_count: usize,
}

impl OrderConfirmed {
    pub fn new(order_id: impl Into<String>, item_count: usize) -> Self {
        Self {
            order_id: order_id.into(),
            item_count,
        }
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn item_count(&self) -> usize {
        self.item_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    OrderConfirmed(OrderConfirmed),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    status: OrderStatus,
    items: Vec<String>,
    events: Vec<DomainEvent>,
}

impl Order {
    pub fn new(id: OrderId) -> Self {
        Self {
            id,
            status: OrderStatus::Draft,
            items: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_item(&mut self, sku: impl Into<String>, quantity: usize) {
        let sku = sku.into();

        for _ in 0..quantity {
            self.items.push(sku.clone());
        }
    }

    pub fn confirm(&mut self) -> Result<(), DomainEventError> {
        if self.status == OrderStatus::Confirmed {
            return Err(DomainEventError::AlreadyConfirmed);
        }

        if self.items.is_empty() {
            return Err(DomainEventError::CannotConfirmEmptyOrder);
        }

        self.status = OrderStatus::Confirmed;
        self.events
            .push(DomainEvent::OrderConfirmed(OrderConfirmed::new(
                self.id.as_str(),
                self.items.len(),
            )));
        Ok(())
    }

    pub fn status(&self) -> &'static str {
        match self.status {
            OrderStatus::Draft => "draft",
            OrderStatus::Confirmed => "confirmed",
        }
    }

    pub fn pull_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderStatus {
    Draft,
    Confirmed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEventError {
    AlreadyConfirmed,
    CannotConfirmEmptyOrder,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InternalIntegrationHandler {
    actions: Vec<String>,
}

impl InternalIntegrationHandler {
    pub fn handle_many(&mut self, events: Vec<DomainEvent>) {
        for event in events {
            self.handle(event);
        }
    }

    pub fn actions(&self) -> Vec<&str> {
        self.actions.iter().map(String::as_str).collect()
    }

    fn handle(&mut self, event: DomainEvent) {
        match event {
            DomainEvent::OrderConfirmed(event) => {
                self.actions.push(format!(
                    "reserve inventory for {} with {} items",
                    event.order_id(),
                    event.item_count()
                ));
                self.actions
                    .push(format!("notify billing for {}", event.order_id()));
            }
        }
    }
}
