use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartLine {
    sku: String,
    quantity: u32,
    unit_price_cents: u32,
}

impl CartLine {
    pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
            unit_price_cents,
        }
    }

    fn subtotal_cents(&self) -> u32 {
        self.quantity * self.unit_price_cents
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CartState {
    session_id: String,
    items: Vec<CartLine>,
    coupon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartSnapshot {
    state: CartState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShoppingCart {
    state: CartState,
}

impl ShoppingCart {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            state: CartState {
                session_id: session_id.into(),
                items: Vec::new(),
                coupon: None,
            },
        }
    }

    pub fn save(&self) -> CartSnapshot {
        CartSnapshot {
            state: self.state.clone(),
        }
    }

    pub fn restore(&mut self, snapshot: CartSnapshot) {
        self.state = snapshot.state;
    }

    pub fn add_item(&mut self, sku: impl Into<String>, quantity: u32, unit_price_cents: u32) {
        self.state
            .items
            .push(CartLine::new(sku, quantity, unit_price_cents));
    }

    pub fn apply_coupon(&mut self, coupon: impl Into<String>) {
        self.state.coupon = Some(coupon.into());
    }

    pub fn clear_coupon(&mut self) {
        self.state.coupon = None;
    }

    pub fn clear(&mut self) {
        self.state.items.clear();
        self.state.coupon = None;
    }

    pub fn session_id(&self) -> &str {
        &self.state.session_id
    }

    pub fn items(&self) -> Vec<CartLine> {
        self.state.items.clone()
    }

    pub fn coupon(&self) -> Option<&str> {
        self.state.coupon.as_deref()
    }

    pub fn total_cents(&self) -> u32 {
        let subtotal: u32 = self.state.items.iter().map(CartLine::subtotal_cents).sum();

        if self.state.coupon.as_deref() == Some("SAVE10") {
            subtotal * 90 / 100
        } else {
            subtotal
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CartBackupStore {
    snapshots: BTreeMap<String, CartSnapshot>,
}

impl CartBackupStore {
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
        }
    }

    pub fn capture(&mut self, cart: &ShoppingCart) {
        self.snapshots
            .insert(cart.session_id().to_string(), cart.save());
    }

    pub fn restore(&self, session_id: &str, cart: &mut ShoppingCart) -> bool {
        if let Some(snapshot) = self.snapshots.get(session_id) {
            cart.restore(snapshot.clone());
            true
        } else {
            false
        }
    }
}
