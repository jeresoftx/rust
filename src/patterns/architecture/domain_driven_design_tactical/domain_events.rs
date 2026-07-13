#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OrderId` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderId(String);

impl OrderId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OrderConfirmed` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderConfirmed {
    order_id: String,
    item_count: usize,
}

impl OrderConfirmed {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: impl Into<String>, item_count: usize) -> Self {
        Self {
            order_id: order_id.into(),
            item_count,
        }
    }

    /// Modela la operacion `order id` dentro del ejemplo del patron.
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    /// Modela la operacion `item count` dentro del ejemplo del patron.
    pub fn item_count(&self) -> usize {
        self.item_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `DomainEvent` dentro del ejemplo.
pub enum DomainEvent {
    /// Variante `OrderConfirmed` del estado o error del ejemplo.
    OrderConfirmed(OrderConfirmed),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: OrderId,
    status: OrderStatus,
    items: Vec<String>,
    events: Vec<DomainEvent>,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: OrderId) -> Self {
        Self {
            id,
            status: OrderStatus::Draft,
            items: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Modela la operacion `add item` dentro del ejemplo del patron.
    pub fn add_item(&mut self, sku: impl Into<String>, quantity: usize) {
        let sku = sku.into();

        for _ in 0..quantity {
            self.items.push(sku.clone());
        }
    }

    /// Modela la operacion `confirm` dentro del ejemplo del patron.
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

    /// Modela la operacion `status` dentro del ejemplo del patron.
    pub fn status(&self) -> &'static str {
        match self.status {
            OrderStatus::Draft => "draft",
            OrderStatus::Confirmed => "confirmed",
        }
    }

    /// Modela la operacion `pull events` dentro del ejemplo del patron.
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
/// Conjunto de estados o errores publicos de `DomainEventError` dentro del ejemplo.
pub enum DomainEventError {
    /// Variante `AlreadyConfirmed` del estado o error del ejemplo.
    AlreadyConfirmed,
    /// Variante `CannotConfirmEmptyOrder` del estado o error del ejemplo.
    CannotConfirmEmptyOrder,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InternalIntegrationHandler` usado por el ejemplo para expresar el dominio del patron.
pub struct InternalIntegrationHandler {
    actions: Vec<String>,
}

impl InternalIntegrationHandler {
    /// Modela la operacion `handle many` dentro del ejemplo del patron.
    pub fn handle_many(&mut self, events: Vec<DomainEvent>) {
        for event in events {
            self.handle(event);
        }
    }

    /// Modela la operacion `actions` dentro del ejemplo del patron.
    pub fn actions(&self) -> Vec<&str> {
        self.actions.iter().map(String::as_str).collect()
    }

    /// Operacion `handle` definida por la abstraccion del ejemplo.
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
