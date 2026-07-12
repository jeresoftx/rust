#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: String,
    customer: String,
    total_cents: u64,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceLine {
    order_id: String,
    customer: String,
    total_cents: u64,
}

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
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn total_cents(&self) -> u64 {
        self.total_cents
    }

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
