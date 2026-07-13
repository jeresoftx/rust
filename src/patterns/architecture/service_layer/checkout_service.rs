use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CheckoutRequest` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutRequest {
    order_id: String,
    items: Vec<(String, u32)>,
    payment_token: String,
}

impl CheckoutRequest {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        order_id: impl Into<String>,
        items: Vec<(String, u32)>,
        payment_token: impl Into<String>,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            items,
            payment_token: payment_token.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CheckoutReceipt` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutReceipt {
    total_cents: u32,
}

impl CheckoutReceipt {
    /// Modela la operacion `total cents` dentro del ejemplo del patron.
    pub fn total_cents(&self) -> u32 {
        self.total_cents
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CheckoutError` dentro del ejemplo.
pub enum CheckoutError {
    /// Variante `InsufficientStock` del estado o error del ejemplo.
    InsufficientStock,
    /// Variante `PaymentDeclined` del estado o error del ejemplo.
    PaymentDeclined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProductStock {
    available: u32,
    price_cents: u32,
}

#[derive(Debug, Default)]
/// Tipo publico `InMemoryInventoryRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryInventoryRepository {
    products: HashMap<String, ProductStock>,
}

impl InMemoryInventoryRepository {
    /// Modela la operacion `seed` dentro del ejemplo del patron.
    pub fn seed(&mut self, sku: impl Into<String>, available: u32, price_cents: u32) {
        self.products.insert(
            sku.into(),
            ProductStock {
                available,
                price_cents,
            },
        );
    }

    /// Operacion `has stock` definida por la abstraccion del ejemplo.
    fn has_stock(&self, sku: &str, quantity: u32) -> bool {
        self.products
            .get(sku)
            .is_some_and(|product| product.available >= quantity)
    }

    /// Operacion `price for` definida por la abstraccion del ejemplo.
    fn price_for(&self, sku: &str) -> Option<u32> {
        self.products.get(sku).map(|product| product.price_cents)
    }

    /// Operacion `reserve` definida por la abstraccion del ejemplo.
    fn reserve(&mut self, sku: &str, quantity: u32) {
        if let Some(product) = self.products.get_mut(sku) {
            product.available -= quantity;
        }
    }

    /// Operacion `stock` definida por la abstraccion del ejemplo.
    fn stock(&self, sku: &str) -> u32 {
        self.products
            .get(sku)
            .map(|product| product.available)
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    subtotal_cents: u32,
    discount_cents: u32,
    total_cents: u32,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        id: impl Into<String>,
        subtotal_cents: u32,
        discount_cents: u32,
        total_cents: u32,
    ) -> Self {
        Self {
            id: id.into(),
            subtotal_cents,
            discount_cents,
            total_cents,
        }
    }
}

#[derive(Debug, Default)]
/// Tipo publico `InMemoryOrderRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryOrderRepository {
    orders: HashMap<String, Order>,
}

impl InMemoryOrderRepository {
    /// Operacion `save` definida por la abstraccion del ejemplo.
    fn save(&mut self, order: Order) {
        self.orders.insert(order.id.clone(), order);
    }

    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, order_id: &str) -> Option<Order> {
        self.orders.get(order_id).cloned()
    }
}

#[derive(Debug, Default)]
/// Tipo publico `PaymentGateway` usado por el ejemplo para expresar el dominio del patron.
pub struct PaymentGateway {
    log: Vec<String>,
}

impl PaymentGateway {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&mut self, token: &str, amount_cents: u32) -> Result<(), CheckoutError> {
        if token == "tok_fail" {
            self.log.push(format!("declined:{token}:{amount_cents}"));
            return Err(CheckoutError::PaymentDeclined);
        }

        self.log.push(format!("charged:{token}:{amount_cents}"));
        Ok(())
    }

    /// Operacion `log` definida por la abstraccion del ejemplo.
    fn log(&self) -> Vec<String> {
        self.log.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DiscountPolicy` usado por el ejemplo para expresar el dominio del patron.
pub struct DiscountPolicy {
    percent: u32,
    threshold_cents: u32,
}

impl DiscountPolicy {
    /// Modela la operacion `percent over threshold` dentro del ejemplo del patron.
    pub fn percent_over_threshold(percent: u32, threshold_cents: u32) -> Self {
        Self {
            percent,
            threshold_cents,
        }
    }

    /// Operacion `discount for` definida por la abstraccion del ejemplo.
    fn discount_for(&self, subtotal_cents: u32) -> u32 {
        if subtotal_cents >= self.threshold_cents {
            subtotal_cents * self.percent / 100
        } else {
            0
        }
    }
}

#[derive(Debug)]
/// Tipo publico `CheckoutService` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutService {
    inventory: InMemoryInventoryRepository,
    orders: InMemoryOrderRepository,
    payment: PaymentGateway,
    discount_policy: DiscountPolicy,
}

impl CheckoutService {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        inventory: InMemoryInventoryRepository,
        orders: InMemoryOrderRepository,
        payment: PaymentGateway,
        discount_policy: DiscountPolicy,
    ) -> Self {
        Self {
            inventory,
            orders,
            payment,
            discount_policy,
        }
    }

    /// Modela la operacion `checkout` dentro del ejemplo del patron.
    pub fn checkout(&mut self, request: CheckoutRequest) -> Result<CheckoutReceipt, CheckoutError> {
        if request
            .items
            .iter()
            .any(|(sku, quantity)| !self.inventory.has_stock(sku, *quantity))
        {
            return Err(CheckoutError::InsufficientStock);
        }

        let subtotal_cents = request
            .items
            .iter()
            .map(|(sku, quantity)| self.inventory.price_for(sku).unwrap_or_default() * quantity)
            .sum();
        let discount_cents = self.discount_policy.discount_for(subtotal_cents);
        let total_cents = subtotal_cents - discount_cents;

        self.payment.charge(&request.payment_token, total_cents)?;

        for (sku, quantity) in &request.items {
            self.inventory.reserve(sku, *quantity);
        }

        self.orders.save(Order::new(
            &request.order_id,
            subtotal_cents,
            discount_cents,
            total_cents,
        ));

        Ok(CheckoutReceipt { total_cents })
    }

    /// Modela la operacion `stock` dentro del ejemplo del patron.
    pub fn stock(&self, sku: &str) -> u32 {
        self.inventory.stock(sku)
    }

    /// Modela la operacion `find order` dentro del ejemplo del patron.
    pub fn find_order(&self, order_id: &str) -> Option<Order> {
        self.orders.find(order_id)
    }

    /// Modela la operacion `payment log` dentro del ejemplo del patron.
    pub fn payment_log(&self) -> Vec<String> {
        self.payment.log()
    }
}
