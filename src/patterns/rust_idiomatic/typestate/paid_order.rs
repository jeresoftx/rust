use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Draft` usado por el ejemplo para expresar el dominio del patron.
pub struct Draft;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Paid` usado por el ejemplo para expresar el dominio del patron.
pub struct Paid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Cancelled` usado por el ejemplo para expresar el dominio del patron.
pub struct Cancelled;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order<State> {
    id: String,
    items: Vec<String>,
    payment_id: Option<String>,
    state: PhantomData<State>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ShipmentReceipt` usado por el ejemplo para expresar el dominio del patron.
pub struct ShipmentReceipt {
    id: String,
    items: Vec<String>,
    payment_id: String,
    tracking_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Cancellation` usado por el ejemplo para expresar el dominio del patron.
pub struct Cancellation {
    id: String,
    items: Vec<String>,
    reason: String,
}

impl Order<Draft> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            payment_id: None,
            state: PhantomData,
        }
    }

    /// Modela la operacion `add item` dentro del ejemplo del patron.
    pub fn add_item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Modela la operacion `pay` dentro del ejemplo del patron.
    pub fn pay(self, payment_id: impl Into<String>) -> Order<Paid> {
        Order {
            id: self.id,
            items: self.items,
            payment_id: Some(payment_id.into()),
            state: PhantomData,
        }
    }

    /// Modela la operacion `cancel` dentro del ejemplo del patron.
    pub fn cancel(self, reason: impl Into<String>) -> Cancellation {
        Cancellation {
            id: self.id,
            items: self.items,
            reason: reason.into(),
        }
    }
}

impl Order<Paid> {
    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Modela la operacion `payment id` dentro del ejemplo del patron.
    pub fn payment_id(&self) -> &str {
        self.payment_id
            .as_deref()
            .expect("Paid siempre contiene payment_id")
    }

    /// Modela la operacion `items` dentro del ejemplo del patron.
    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }

    /// Modela la operacion `ship` dentro del ejemplo del patron.
    pub fn ship(self, tracking_number: impl Into<String>) -> ShipmentReceipt {
        ShipmentReceipt {
            id: self.id,
            items: self.items,
            payment_id: self.payment_id.expect("Paid siempre contiene payment_id"),
            tracking_number: tracking_number.into(),
        }
    }
}

impl ShipmentReceipt {
    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} paid with {} shipped as {}",
            self.id, self.payment_id, self.tracking_number
        )
    }

    /// Modela la operacion `items` dentro del ejemplo del patron.
    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }
}

impl Cancellation {
    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!("{} cancelled: {}", self.id, self.reason)
    }

    /// Modela la operacion `items` dentro del ejemplo del patron.
    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }
}
