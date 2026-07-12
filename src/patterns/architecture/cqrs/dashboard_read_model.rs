#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductStockSnapshot {
    sku: String,
    name: String,
    available: u32,
    low_stock_threshold: u32,
}

impl ProductStockSnapshot {
    pub fn new(
        sku: impl Into<String>,
        name: impl Into<String>,
        available: u32,
        low_stock_threshold: u32,
    ) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
            available,
            low_stock_threshold,
        }
    }

    fn status(&self) -> StockStatus {
        if self.available == 0 {
            StockStatus::OutOfStock
        } else if self.available <= self.low_stock_threshold {
            StockStatus::Low
        } else {
            StockStatus::Healthy
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockStatus {
    OutOfStock,
    Low,
    Healthy,
}

impl StockStatus {
    fn priority(self) -> u8 {
        match self {
            Self::OutOfStock => 0,
            Self::Low => 1,
            Self::Healthy => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventorySummaryCard {
    sku: String,
    name: String,
    available: u32,
    status: StockStatus,
}

impl InventorySummaryCard {
    pub fn new(
        sku: impl Into<String>,
        name: impl Into<String>,
        available: u32,
        status: StockStatus,
    ) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
            available,
            status,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardQuery {
    AllProducts,
    NeedsAttention,
}

impl DashboardQuery {
    pub fn all_products() -> Self {
        Self::AllProducts
    }

    pub fn needs_attention() -> Self {
        Self::NeedsAttention
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryDashboardReadModel {
    snapshots: Vec<ProductStockSnapshot>,
}

impl InventoryDashboardReadModel {
    pub fn from_snapshots(snapshots: Vec<ProductStockSnapshot>) -> Self {
        Self { snapshots }
    }

    pub fn handle(&self, query: DashboardQuery) -> Vec<InventorySummaryCard> {
        let mut cards = self
            .snapshots
            .iter()
            .filter(|snapshot| match query {
                DashboardQuery::AllProducts => true,
                DashboardQuery::NeedsAttention => snapshot.status() != StockStatus::Healthy,
            })
            .map(|snapshot| {
                InventorySummaryCard::new(
                    &snapshot.sku,
                    &snapshot.name,
                    snapshot.available,
                    snapshot.status(),
                )
            })
            .collect::<Vec<_>>();

        cards.sort_by(|left, right| {
            left.status
                .priority()
                .cmp(&right.status.priority())
                .then_with(|| left.name.cmp(&right.name))
        });
        cards
    }
}
