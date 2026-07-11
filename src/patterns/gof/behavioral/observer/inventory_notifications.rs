#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockChange {
    sku: String,
    quantity: u32,
}

impl StockChange {
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InventorySubject {
    restock_threshold: Option<u32>,
    sales_updates_enabled: bool,
    restock_alerts: Vec<String>,
    sales_updates: Vec<String>,
}

impl InventorySubject {
    pub fn new() -> Self {
        Self {
            restock_threshold: None,
            sales_updates_enabled: false,
            restock_alerts: Vec::new(),
            sales_updates: Vec::new(),
        }
    }

    pub fn subscribe_restock_alerts(&mut self, threshold: u32) {
        self.restock_threshold = Some(threshold);
    }

    pub fn subscribe_sales_updates(&mut self) {
        self.sales_updates_enabled = true;
    }

    pub fn change_stock(&mut self, change: StockChange) {
        if let Some(threshold) = self.restock_threshold {
            if change.quantity <= threshold {
                self.restock_alerts
                    .push(format!("restock-needed:{}:{}", change.sku, change.quantity));
            }
        }

        if self.sales_updates_enabled {
            self.sales_updates
                .push(format!("stock-updated:{}:{}", change.sku, change.quantity));
        }
    }

    pub fn restock_alerts(&self) -> Vec<String> {
        self.restock_alerts.clone()
    }

    pub fn sales_updates(&self) -> Vec<String> {
        self.sales_updates.clone()
    }
}
