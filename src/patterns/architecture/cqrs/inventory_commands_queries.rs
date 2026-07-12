use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InventoryWriteModel {
    stock_by_sku: HashMap<String, u32>,
}

impl InventoryWriteModel {
    pub fn available(&self, sku: &str) -> u32 {
        self.stock_by_sku.get(sku).copied().unwrap_or_default()
    }

    fn receive(&mut self, sku: &str, quantity: u32) {
        *self.stock_by_sku.entry(sku.to_string()).or_default() += quantity;
    }

    fn reserve(&mut self, sku: &str, quantity: u32) -> Result<(), InventoryError> {
        let available = self.available(sku);

        if available < quantity {
            return Err(InventoryError::InsufficientStock);
        }

        self.stock_by_sku
            .insert(sku.to_string(), available - quantity);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveStockCommand {
    sku: String,
    quantity: u32,
}

impl ReceiveStockCommand {
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReserveStockCommand {
    sku: String,
    quantity: u32,
}

impl ReserveStockCommand {
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct InventoryCommandHandler<'a> {
    write_model: &'a mut InventoryWriteModel,
}

impl<'a> InventoryCommandHandler<'a> {
    pub fn new(write_model: &'a mut InventoryWriteModel) -> Self {
        Self { write_model }
    }

    pub fn handle_receive(&mut self, command: ReceiveStockCommand) -> Result<(), InventoryError> {
        self.write_model.receive(&command.sku, command.quantity);
        Ok(())
    }

    pub fn handle_reserve(&mut self, command: ReserveStockCommand) -> Result<(), InventoryError> {
        self.write_model.reserve(&command.sku, command.quantity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetInventoryQuery {
    sku: String,
}

impl GetInventoryQuery {
    pub fn new(sku: impl Into<String>) -> Self {
        Self { sku: sku.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockLevel {
    sku: String,
    available: u32,
}

impl StockLevel {
    pub fn new(sku: impl Into<String>, available: u32) -> Self {
        Self {
            sku: sku.into(),
            available,
        }
    }
}

#[derive(Debug)]
pub struct InventoryQueryHandler<'a> {
    read_source: &'a InventoryWriteModel,
}

impl<'a> InventoryQueryHandler<'a> {
    pub fn new(read_source: &'a InventoryWriteModel) -> Self {
        Self { read_source }
    }

    pub fn handle(&self, query: GetInventoryQuery) -> StockLevel {
        StockLevel::new(&query.sku, self.read_source.available(&query.sku))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InventoryError {
    InsufficientStock,
}
