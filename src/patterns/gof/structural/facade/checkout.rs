use std::collections::HashMap;

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
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
        }
    }

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

    fn total_cents(&self) -> u64 {
        self.items
            .iter()
            .map(|item| item.quantity as u64 * item.unit_price_cents)
            .sum()
    }
}

pub struct InventoryService {
    stock: HashMap<String, u32>,
}

impl InventoryService {
    pub fn new(stock: Vec<(&str, u32)>) -> Self {
        Self {
            stock: stock
                .into_iter()
                .map(|(sku, quantity)| (sku.to_string(), quantity))
                .collect(),
        }
    }

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

pub struct PaymentGateway {
    provider: String,
}

impl PaymentGateway {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }

    fn charge(&self, amount_cents: u64) -> String {
        format!("charged={amount_cents} provider={}", self.provider)
    }
}

pub struct CheckoutFacade {
    inventory: InventoryService,
    payment: PaymentGateway,
}

impl CheckoutFacade {
    pub fn new(inventory: InventoryService, payment: PaymentGateway) -> Self {
        Self { inventory, payment }
    }

    pub fn checkout(&self, cart: Cart) -> Result<String, String> {
        let reserved = self.inventory.reserve(&cart)?;
        let payment = self.payment.charge(cart.total_cents());

        Ok(format!(
            "cart={} reserved={} {}",
            cart.id, reserved, payment
        ))
    }
}
