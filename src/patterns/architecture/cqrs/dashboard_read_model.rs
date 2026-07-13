#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ProductStockSnapshot` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductStockSnapshot {
    sku: String,
    name: String,
    available: u32,
    low_stock_threshold: u32,
}

impl ProductStockSnapshot {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Operacion `status` definida por la abstraccion del ejemplo.
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
/// Conjunto de estados o errores publicos de `StockStatus` dentro del ejemplo.
pub enum StockStatus {
    /// Variante `OutOfStock` del estado o error del ejemplo.
    OutOfStock,
    /// Variante `Low` del estado o error del ejemplo.
    Low,
    /// Variante `Healthy` del estado o error del ejemplo.
    Healthy,
}

impl StockStatus {
    /// Operacion `priority` definida por la abstraccion del ejemplo.
    fn priority(self) -> u8 {
        match self {
            Self::OutOfStock => 0,
            Self::Low => 1,
            Self::Healthy => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `InventorySummaryCard` usado por el ejemplo para expresar el dominio del patron.
pub struct InventorySummaryCard {
    sku: String,
    name: String,
    available: u32,
    status: StockStatus,
}

impl InventorySummaryCard {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `DashboardQuery` dentro del ejemplo.
pub enum DashboardQuery {
    /// Variante `AllProducts` del estado o error del ejemplo.
    AllProducts,
    /// Variante `NeedsAttention` del estado o error del ejemplo.
    NeedsAttention,
}

impl DashboardQuery {
    /// Modela la operacion `all products` dentro del ejemplo del patron.
    pub fn all_products() -> Self {
        Self::AllProducts
    }

    /// Modela la operacion `needs attention` dentro del ejemplo del patron.
    pub fn needs_attention() -> Self {
        Self::NeedsAttention
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryDashboardReadModel` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryDashboardReadModel {
    snapshots: Vec<ProductStockSnapshot>,
}

impl InventoryDashboardReadModel {
    /// Modela la operacion `from snapshots` dentro del ejemplo del patron.
    pub fn from_snapshots(snapshots: Vec<ProductStockSnapshot>) -> Self {
        Self { snapshots }
    }

    /// Procesa la entrada publica del ejemplo.
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
