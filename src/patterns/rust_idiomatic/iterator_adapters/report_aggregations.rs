#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaleStatus {
    Completed,
    Refunded,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaleEvent {
    channel: String,
    amount_cents: u64,
    status: SaleStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReportSummary {
    completed_sales: usize,
    refunds: usize,
    completed_revenue_cents: u64,
    refunded_cents: u64,
    largest_completed_sale_cents: Option<u64>,
}

pub fn summarize_sales(events: &[SaleEvent]) -> ReportSummary {
    events
        .iter()
        .fold(ReportSummary::default(), |mut summary, event| {
            match event.status {
                SaleStatus::Completed => {
                    summary.completed_sales += 1;
                    summary.completed_revenue_cents += event.amount_cents;
                    summary.largest_completed_sale_cents = summary
                        .largest_completed_sale_cents
                        .map(|current| current.max(event.amount_cents))
                        .or(Some(event.amount_cents));
                }
                SaleStatus::Refunded => {
                    summary.refunds += 1;
                    summary.refunded_cents += event.amount_cents;
                }
                SaleStatus::Pending => {}
            }

            summary
        })
}

impl SaleEvent {
    pub fn new(channel: impl Into<String>, amount_cents: u64, status: SaleStatus) -> Self {
        Self {
            channel: channel.into(),
            amount_cents,
            status,
        }
    }
}

impl ReportSummary {
    pub fn completed_sales(&self) -> usize {
        self.completed_sales
    }

    pub fn refunds(&self) -> usize {
        self.refunds
    }

    pub fn net_revenue_cents(&self) -> i64 {
        self.completed_revenue_cents as i64 - self.refunded_cents as i64
    }

    pub fn average_completed_ticket_cents(&self) -> u64 {
        if self.completed_sales == 0 {
            return 0;
        }

        self.completed_revenue_cents / self.completed_sales as u64
    }

    pub fn largest_completed_sale_cents(&self) -> Option<u64> {
        self.largest_completed_sale_cents
    }

    pub fn summary(&self) -> String {
        let net_revenue_cents = self.net_revenue_cents();
        let sign = if net_revenue_cents < 0 { "-" } else { "" };
        let absolute = net_revenue_cents.abs();

        format!(
            "{} sales, {} refunds, net {}${}.{:02}",
            self.completed_sales,
            self.refunds,
            sign,
            absolute / 100,
            absolute % 100
        )
    }
}
