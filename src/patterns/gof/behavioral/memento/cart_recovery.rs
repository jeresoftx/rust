use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CartLine` usado por el ejemplo para expresar el dominio del patron.
pub struct CartLine {
    sku: String,
    quantity: u32,
    unit_price_cents: u32,
}

impl CartLine {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
            unit_price_cents,
        }
    }

    /// Operacion `subtotal cents` definida por la abstraccion del ejemplo.
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
/// Tipo publico `CartSnapshot` usado por el ejemplo para expresar el dominio del patron.
pub struct CartSnapshot {
    state: CartState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ShoppingCart` usado por el ejemplo para expresar el dominio del patron.
pub struct ShoppingCart {
    state: CartState,
}

impl ShoppingCart {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            state: CartState {
                session_id: session_id.into(),
                items: Vec::new(),
                coupon: None,
            },
        }
    }

    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&self) -> CartSnapshot {
        CartSnapshot {
            state: self.state.clone(),
        }
    }

    /// Modela la operacion `restore` dentro del ejemplo del patron.
    pub fn restore(&mut self, snapshot: CartSnapshot) {
        self.state = snapshot.state;
    }

    /// Modela la operacion `add item` dentro del ejemplo del patron.
    pub fn add_item(&mut self, sku: impl Into<String>, quantity: u32, unit_price_cents: u32) {
        self.state
            .items
            .push(CartLine::new(sku, quantity, unit_price_cents));
    }

    /// Modela la operacion `apply coupon` dentro del ejemplo del patron.
    pub fn apply_coupon(&mut self, coupon: impl Into<String>) {
        self.state.coupon = Some(coupon.into());
    }

    /// Modela la operacion `clear coupon` dentro del ejemplo del patron.
    pub fn clear_coupon(&mut self) {
        self.state.coupon = None;
    }

    /// Modela la operacion `clear` dentro del ejemplo del patron.
    pub fn clear(&mut self) {
        self.state.items.clear();
        self.state.coupon = None;
    }

    /// Modela la operacion `session id` dentro del ejemplo del patron.
    pub fn session_id(&self) -> &str {
        &self.state.session_id
    }

    /// Modela la operacion `items` dentro del ejemplo del patron.
    pub fn items(&self) -> Vec<CartLine> {
        self.state.items.clone()
    }

    /// Modela la operacion `coupon` dentro del ejemplo del patron.
    pub fn coupon(&self) -> Option<&str> {
        self.state.coupon.as_deref()
    }

    /// Modela la operacion `total cents` dentro del ejemplo del patron.
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
/// Tipo publico `CartBackupStore` usado por el ejemplo para expresar el dominio del patron.
pub struct CartBackupStore {
    snapshots: BTreeMap<String, CartSnapshot>,
}

impl CartBackupStore {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
        }
    }

    /// Modela la operacion `capture` dentro del ejemplo del patron.
    pub fn capture(&mut self, cart: &ShoppingCart) {
        self.snapshots
            .insert(cart.session_id().to_string(), cart.save());
    }

    /// Modela la operacion `restore` dentro del ejemplo del patron.
    pub fn restore(&self, session_id: &str, cart: &mut ShoppingCart) -> bool {
        if let Some(snapshot) = self.snapshots.get(session_id) {
            cart.restore(snapshot.clone());
            true
        } else {
            false
        }
    }
}
