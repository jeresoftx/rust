#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderTemplate {
    customer_id: String,
    sku: String,
    quantity: u32,
    cadence: String,
    shipping: String,
}

impl OrderTemplate {
    pub fn weekly_office_supplies() -> Self {
        Self {
            customer_id: "<customer>".to_string(),
            sku: "PAPER-A4".to_string(),
            quantity: 1,
            cadence: "weekly".to_string(),
            shipping: "standard".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "customer={} sku={} quantity={} cadence={} shipping={}",
            self.customer_id, self.sku, self.quantity, self.cadence, self.shipping
        )
    }
}

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
