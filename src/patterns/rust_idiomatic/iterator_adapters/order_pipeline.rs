#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderStatus` dentro del ejemplo.
pub enum OrderStatus {
    /// Variante `Pending` del estado o error del ejemplo.
    Pending,
    /// Variante `Paid` del estado o error del ejemplo.
    Paid,
    /// Variante `Cancelled` del estado o error del ejemplo.
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    customer: String,
    total_cents: u64,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `InvoiceLine` usado por el ejemplo para expresar el dominio del patron.
pub struct InvoiceLine {
    order_id: String,
    customer: String,
    total_cents: u64,
}

/// Modela la operacion `billable order lines` dentro del ejemplo del patron.
pub fn billable_order_lines(orders: &[Order]) -> Vec<InvoiceLine> {
    orders
        .iter()
        .filter(|order| order.status == OrderStatus::Paid)
        .filter(|order| order.total_cents > 0)
        .map(|order| InvoiceLine {
            order_id: order.id.clone(),
            customer: order.customer.trim().to_string(),
            total_cents: order.total_cents,
        })
        .collect()
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        id: impl Into<String>,
        customer: impl Into<String>,
        total_cents: u64,
        status: OrderStatus,
    ) -> Self {
        Self {
            id: id.into(),
            customer: customer.into(),
            total_cents,
            status,
        }
    }
}

impl InvoiceLine {
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
            "{} {} ${}.{:02}",
            self.order_id,
            self.customer,
            self.total_cents / 100,
            self.total_cents % 100
        )
    }
}
