use std::collections::HashMap;

/// Tipo publico `Cart` usado por el ejemplo para expresar el dominio del patron.
pub struct Cart {
    id: String,
    items: Vec<CartItem>,
}

struct CartItem {
    sku: String,
    quantity: u32,
    unit_price_cents: u64,
}

impl Cart {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
        }
    }

    /// Modela la operacion `with item` dentro del ejemplo del patron.
    pub fn with_item(
        mut self,
        sku: impl Into<String>,
        quantity: u32,
        unit_price_cents: u64,
    ) -> Self {
        self.items.push(CartItem {
            sku: sku.into(),
            quantity,
            unit_price_cents,
        });
        self
    }

    /// Operacion `total cents` definida por la abstraccion del ejemplo.
    fn total_cents(&self) -> u64 {
        self.items
            .iter()
            .map(|item| item.quantity as u64 * item.unit_price_cents)
            .sum()
    }
}

/// Tipo publico `InventoryService` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryService {
    stock: HashMap<String, u32>,
}

impl InventoryService {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(stock: Vec<(&str, u32)>) -> Self {
        Self {
            stock: stock
                .into_iter()
                .map(|(sku, quantity)| (sku.to_string(), quantity))
                .collect(),
        }
    }

    /// Operacion `reserve` definida por la abstraccion del ejemplo.
    fn reserve(&self, cart: &Cart) -> Result<String, String> {
        for item in &cart.items {
            let available = self.stock.get(&item.sku).copied().unwrap_or_default();
            if available < item.quantity {
                return Err(format!("insufficient inventory for {}", item.sku));
            }
        }

        Ok(cart
            .items
            .iter()
            .map(|item| format!("{}x{}", item.sku, item.quantity))
            .collect::<Vec<_>>()
            .join(","))
    }
}

/// Tipo publico `PaymentGateway` usado por el ejemplo para expresar el dominio del patron.
pub struct PaymentGateway {
    provider: String,
}

impl PaymentGateway {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }

    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, amount_cents: u64) -> String {
        format!("charged={amount_cents} provider={}", self.provider)
    }
}

/// Tipo publico `CheckoutFacade` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutFacade {
    inventory: InventoryService,
    payment: PaymentGateway,
}

impl CheckoutFacade {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inventory: InventoryService, payment: PaymentGateway) -> Self {
        Self { inventory, payment }
    }

    /// Modela la operacion `checkout` dentro del ejemplo del patron.
    pub fn checkout(&self, cart: Cart) -> Result<String, String> {
        let reserved = self.inventory.reserve(&cart)?;
        let payment = self.payment.charge(cart.total_cents());

        Ok(format!(
            "cart={} reserved={} {}",
            cart.id, reserved, payment
        ))
    }
}
