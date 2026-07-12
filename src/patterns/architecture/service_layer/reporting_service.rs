use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    id: String,
    name: String,
}

impl Customer {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderRecord {
    id: String,
    customer_id: String,
    amount_cents: u32,
    status: String,
}

impl OrderRecord {
    pub fn new(
        id: impl Into<String>,
        customer_id: impl Into<String>,
        amount_cents: u32,
        status: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            customer_id: customer_id.into(),
            amount_cents,
            status: status.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalesReportQuery {
    include_refunds: bool,
}

impl SalesReportQuery {
    pub fn paid_only() -> Self {
        Self {
            include_refunds: false,
        }
    }

    pub fn include_all() -> Self {
        Self {
            include_refunds: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopCustomer {
    id: String,
    name: String,
    revenue_cents: u32,
}

impl TopCustomer {
    pub fn new(id: impl Into<String>, name: impl Into<String>, revenue_cents: u32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            revenue_cents,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalesReport {
    total_revenue_cents: u32,
    order_count: usize,
    top_customer: Option<TopCustomer>,
}

impl SalesReport {
    pub fn total_revenue_cents(&self) -> u32 {
        self.total_revenue_cents
    }

    pub fn order_count(&self) -> usize {
        self.order_count
    }

    pub fn top_customer(&self) -> Option<TopCustomer> {
        self.top_customer.clone()
    }
}

#[derive(Debug)]
pub struct ReportingRepository {
    customers: Vec<Customer>,
    orders: Vec<OrderRecord>,
}

impl ReportingRepository {
    pub fn new(customers: Vec<Customer>, orders: Vec<OrderRecord>) -> Self {
        Self { customers, orders }
    }

    fn customers_by_id(&self) -> HashMap<&str, &Customer> {
        self.customers
            .iter()
            .map(|customer| (customer.id.as_str(), customer))
            .collect()
    }

    fn orders_matching(&self, query: &SalesReportQuery) -> Vec<&OrderRecord> {
        self.orders
            .iter()
            .filter(|order| query.include_refunds || order.status == "paid")
            .collect()
    }
}

#[derive(Debug)]
pub struct ReportingService {
    repository: ReportingRepository,
}

impl ReportingService {
    pub fn new(repository: ReportingRepository) -> Self {
        Self { repository }
    }

    pub fn sales_report(&self, query: SalesReportQuery) -> SalesReport {
        let customers_by_id = self.repository.customers_by_id();
        let orders = self.repository.orders_matching(&query);
        let total_revenue_cents = orders.iter().map(|order| order.amount_cents).sum();
        let order_count = orders.len();

        let mut revenue_by_customer: HashMap<&str, u32> = HashMap::new();
        for order in orders {
            *revenue_by_customer
                .entry(order.customer_id.as_str())
                .or_default() += order.amount_cents;
        }

        let top_customer = revenue_by_customer
            .into_iter()
            .max_by_key(|(_, revenue_cents)| *revenue_cents)
            .map(|(customer_id, revenue_cents)| {
                let name = customers_by_id
                    .get(customer_id)
                    .map(|customer| customer.name.as_str())
                    .unwrap_or(customer_id);

                TopCustomer::new(customer_id, name, revenue_cents)
            });

        SalesReport {
            total_revenue_cents,
            order_count,
            top_customer,
        }
    }
}
