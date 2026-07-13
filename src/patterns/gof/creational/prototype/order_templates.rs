#[derive(Clone, Debug, PartialEq, Eq)]
/// Tipo publico `OrderTemplate` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderTemplate {
    customer_id: String,
    sku: String,
    quantity: u32,
    cadence: String,
    shipping: String,
}

impl OrderTemplate {
    /// Modela la operacion `weekly office supplies` dentro del ejemplo del patron.
    pub fn weekly_office_supplies() -> Self {
        Self {
            customer_id: "<customer>".to_string(),
            sku: "PAPER-A4".to_string(),
            quantity: 1,
            cadence: "weekly".to_string(),
            shipping: "standard".to_string(),
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "customer={} sku={} quantity={} cadence={} shipping={}",
            self.customer_id, self.sku, self.quantity, self.cadence, self.shipping
        )
    }
}

/// Modela la operacion `create order from template` dentro del ejemplo del patron.
pub fn create_order_from_template(
    template: &OrderTemplate,
    customer_id: &str,
    quantity: u32,
) -> OrderTemplate {
    let mut order = template.clone();
    order.customer_id = customer_id.to_string();
    order.quantity = quantity;
    order
}
