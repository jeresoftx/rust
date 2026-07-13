#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CheckoutError` dentro del ejemplo.
pub enum CheckoutError {
    /// Variante `EmptyCart` del estado o error del ejemplo.
    EmptyCart,
    /// Variante `MissingPayment` del estado o error del ejemplo.
    MissingPayment,
    /// Variante `SkuNotFound` del estado o error del ejemplo.
    SkuNotFound {
        /// Valor publico `sku` asociado a la variante `SkuNotFound`.
        sku: String,
    },
    /// Variante `InsufficientInventory` del estado o error del ejemplo.
    InsufficientInventory {
        /// Valor publico `sku` asociado a la variante del enum.
        sku: String,
        /// Valor publico `requested` asociado a la variante del enum.
        requested: u32,
        /// Valor publico `available` asociado a la variante del enum.
        available: u32,
    },
    /// Variante `PaymentRejected` del estado o error del ejemplo.
    PaymentRejected,
}

impl CheckoutError {
    /// Modela la operacion `message` dentro del ejemplo del patron.
    pub fn message(&self) -> String {
        match self {
            Self::EmptyCart => "cart has no items".to_string(),
            Self::MissingPayment => "payment method is required".to_string(),
            Self::SkuNotFound { sku } => format!("sku {sku} was not found"),
            Self::InsufficientInventory {
                sku,
                requested,
                available,
            } => {
                format!("sku {sku} requested {requested} units but only {available} are available")
            }
            Self::PaymentRejected => "payment was rejected".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PaymentMethod` dentro del ejemplo.
pub enum PaymentMethod {
    /// Variante `Card` del estado o error del ejemplo.
    Card {
        /// Valor publico `approved` asociado a la variante `Card`.
        approved: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CheckoutRequest` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutRequest {
    order_id: String,
    items: Vec<CartItem>,
    payment: Option<PaymentMethod>,
}

impl CheckoutRequest {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            items: Vec::new(),
            payment: None,
        }
    }

    /// Modela la operacion `with item` dentro del ejemplo del patron.
    pub fn with_item(mut self, sku: impl Into<String>, quantity: u32) -> Self {
        self.items.push(CartItem {
            sku: sku.into(),
            quantity,
        });
        self
    }

    /// Modela la operacion `with payment` dentro del ejemplo del patron.
    pub fn with_payment(mut self, payment: PaymentMethod) -> Self {
        self.payment = Some(payment);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CartItem {
    sku: String,
    quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryItem` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryItem {
    sku: String,
    available: u32,
    unit_price_cents: u64,
}

impl InventoryItem {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, available: u32, unit_price_cents: u64) -> Self {
        Self {
            sku: sku.into(),
            available,
            unit_price_cents,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Receipt` usado por el ejemplo para expresar el dominio del patron.
pub struct Receipt {
    order_id: String,
    total_cents: u64,
}

impl Receipt {
    /// Modela la operacion `order id` dentro del ejemplo del patron.
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    /// Modela la operacion `total cents` dentro del ejemplo del patron.
    pub fn total_cents(&self) -> u64 {
        self.total_cents
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} charged ${}.{:02}",
            self.order_id,
            self.total_cents / 100,
            self.total_cents % 100
        )
    }
}

/// Modela la operacion `checkout` dentro del ejemplo del patron.
pub fn checkout(
    request: &CheckoutRequest,
    inventory: &[InventoryItem],
) -> Result<Receipt, CheckoutError> {
    if request.items.is_empty() {
        return Err(CheckoutError::EmptyCart);
    }

    let payment = request
        .payment
        .as_ref()
        .ok_or(CheckoutError::MissingPayment)?;
    let mut total_cents = 0;

    for item in &request.items {
        let stock = inventory
            .iter()
            .find(|stock| stock.sku == item.sku)
            .ok_or_else(|| CheckoutError::SkuNotFound {
                sku: item.sku.clone(),
            })?;

        if stock.available < item.quantity {
            return Err(CheckoutError::InsufficientInventory {
                sku: item.sku.clone(),
                requested: item.quantity,
                available: stock.available,
            });
        }

        total_cents += stock.unit_price_cents * u64::from(item.quantity);
    }

    match payment {
        PaymentMethod::Card { approved: true } => Ok(Receipt {
            order_id: request.order_id.clone(),
            total_cents,
        }),
        PaymentMethod::Card { approved: false } => Err(CheckoutError::PaymentRejected),
    }
}
