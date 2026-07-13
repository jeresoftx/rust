#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StockChange` usado por el ejemplo para expresar el dominio del patron.
pub struct StockChange {
    sku: String,
    quantity: u32,
}

impl StockChange {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Default, Clone)]
/// Tipo publico `InventorySubject` usado por el ejemplo para expresar el dominio del patron.
pub struct InventorySubject {
    restock_threshold: Option<u32>,
    sales_updates_enabled: bool,
    restock_alerts: Vec<String>,
    sales_updates: Vec<String>,
}

impl InventorySubject {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            restock_threshold: None,
            sales_updates_enabled: false,
            restock_alerts: Vec::new(),
            sales_updates: Vec::new(),
        }
    }

    /// Modela la operacion `subscribe restock alerts` dentro del ejemplo del patron.
    pub fn subscribe_restock_alerts(&mut self, threshold: u32) {
        self.restock_threshold = Some(threshold);
    }

    /// Modela la operacion `subscribe sales updates` dentro del ejemplo del patron.
    pub fn subscribe_sales_updates(&mut self) {
        self.sales_updates_enabled = true;
    }

    /// Modela la operacion `change stock` dentro del ejemplo del patron.
    pub fn change_stock(&mut self, change: StockChange) {
        if let Some(threshold) = self.restock_threshold
            && change.quantity <= threshold
        {
            self.restock_alerts
                .push(format!("restock-needed:{}:{}", change.sku, change.quantity));
        }

        if self.sales_updates_enabled {
            self.sales_updates
                .push(format!("stock-updated:{}:{}", change.sku, change.quantity));
        }
    }

    /// Modela la operacion `restock alerts` dentro del ejemplo del patron.
    pub fn restock_alerts(&self) -> Vec<String> {
        self.restock_alerts.clone()
    }

    /// Modela la operacion `sales updates` dentro del ejemplo del patron.
    pub fn sales_updates(&self) -> Vec<String> {
        self.sales_updates.clone()
    }
}
