use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryWriteModel` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryWriteModel {
    stock_by_sku: HashMap<String, u32>,
}

impl InventoryWriteModel {
    /// Modela la operacion `available` dentro del ejemplo del patron.
    pub fn available(&self, sku: &str) -> u32 {
        self.stock_by_sku.get(sku).copied().unwrap_or_default()
    }

    /// Operacion `receive` definida por la abstraccion del ejemplo.
    fn receive(&mut self, sku: &str, quantity: u32) {
        *self.stock_by_sku.entry(sku.to_string()).or_default() += quantity;
    }

    /// Operacion `reserve` definida por la abstraccion del ejemplo.
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
/// Tipo publico `ReceiveStockCommand` usado por el ejemplo para expresar el dominio del patron.
pub struct ReceiveStockCommand {
    sku: String,
    quantity: u32,
}

impl ReceiveStockCommand {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ReserveStockCommand` usado por el ejemplo para expresar el dominio del patron.
pub struct ReserveStockCommand {
    sku: String,
    quantity: u32,
}

impl ReserveStockCommand {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `InventoryCommandHandler` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryCommandHandler<'a> {
    write_model: &'a mut InventoryWriteModel,
}

impl<'a> InventoryCommandHandler<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(write_model: &'a mut InventoryWriteModel) -> Self {
        Self { write_model }
    }

    /// Modela la operacion `handle receive` dentro del ejemplo del patron.
    pub fn handle_receive(&mut self, command: ReceiveStockCommand) -> Result<(), InventoryError> {
        self.write_model.receive(&command.sku, command.quantity);
        Ok(())
    }

    /// Modela la operacion `handle reserve` dentro del ejemplo del patron.
    pub fn handle_reserve(&mut self, command: ReserveStockCommand) -> Result<(), InventoryError> {
        self.write_model.reserve(&command.sku, command.quantity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `GetInventoryQuery` usado por el ejemplo para expresar el dominio del patron.
pub struct GetInventoryQuery {
    sku: String,
}

impl GetInventoryQuery {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>) -> Self {
        Self { sku: sku.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StockLevel` usado por el ejemplo para expresar el dominio del patron.
pub struct StockLevel {
    sku: String,
    available: u32,
}

impl StockLevel {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, available: u32) -> Self {
        Self {
            sku: sku.into(),
            available,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `InventoryQueryHandler` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryQueryHandler<'a> {
    read_source: &'a InventoryWriteModel,
}

impl<'a> InventoryQueryHandler<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(read_source: &'a InventoryWriteModel) -> Self {
        Self { read_source }
    }

    /// Procesa la entrada publica del ejemplo.
    pub fn handle(&self, query: GetInventoryQuery) -> StockLevel {
        StockLevel::new(&query.sku, self.read_source.available(&query.sku))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `InventoryError` dentro del ejemplo.
pub enum InventoryError {
    /// Variante `InsufficientStock` del estado o error del ejemplo.
    InsufficientStock,
}
