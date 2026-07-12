use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Draft;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cancelled;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<State> {
    id: String,
    items: Vec<String>,
    payment_id: Option<String>,
    state: PhantomData<State>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShipmentReceipt {
    id: String,
    items: Vec<String>,
    payment_id: String,
    tracking_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cancellation {
    id: String,
    items: Vec<String>,
    reason: String,
}

impl Order<Draft> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            payment_id: None,
            state: PhantomData,
        }
    }

    pub fn add_item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn pay(self, payment_id: impl Into<String>) -> Order<Paid> {
        Order {
            id: self.id,
            items: self.items,
            payment_id: Some(payment_id.into()),
            state: PhantomData,
        }
    }

    pub fn cancel(self, reason: impl Into<String>) -> Cancellation {
        Cancellation {
            id: self.id,
            items: self.items,
            reason: reason.into(),
        }
    }
}

impl Order<Paid> {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn payment_id(&self) -> &str {
        self.payment_id
            .as_deref()
            .expect("Paid siempre contiene payment_id")
    }

    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }

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
    pub fn summary(&self) -> String {
        format!(
            "{} paid with {} shipped as {}",
            self.id, self.payment_id, self.tracking_number
        )
    }

    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }
}

impl Cancellation {
    pub fn summary(&self) -> String {
        format!("{} cancelled: {}", self.id, self.reason)
    }

    pub fn items(&self) -> Vec<&str> {
        self.items.iter().map(String::as_str).collect()
    }
}
