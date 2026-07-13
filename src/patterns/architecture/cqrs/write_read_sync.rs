use std::collections::HashMap;

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryWriteModel` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryWriteModel {
    stock_by_sku: HashMap<String, u32>,
}

impl InventoryWriteModel {
    /// Modela la operacion `receive` dentro del ejemplo del patron.
    pub fn receive(&mut self, command: ReceiveStockCommand) -> StockChanged {
        let available = self.available(&command.sku) + command.quantity;
        self.stock_by_sku.insert(command.sku.clone(), available);
        StockChanged::received(command.sku, command.quantity, available)
    }

    /// Modela la operacion `reserve` dentro del ejemplo del patron.
    pub fn reserve(
        &mut self,
        command: ReserveStockCommand,
    ) -> Result<StockChanged, InventoryError> {
        let available = self.available(&command.sku);

        if available < command.quantity {
            return Err(InventoryError::InsufficientStock);
        }

        let new_available = available - command.quantity;
        self.stock_by_sku.insert(command.sku.clone(), new_available);
        Ok(StockChanged::reserved(
            command.sku,
            command.quantity,
            new_available,
        ))
    }

    /// Operacion `available` definida por la abstraccion del ejemplo.
    fn available(&self, sku: &str) -> u32 {
        self.stock_by_sku.get(sku).copied().unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `InventoryError` dentro del ejemplo.
pub enum InventoryError {
    /// Variante `InsufficientStock` del estado o error del ejemplo.
    InsufficientStock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StockChanged` usado por el ejemplo para expresar el dominio del patron.
pub struct StockChanged {
    sku: String,
    quantity: u32,
    available_after_change: u32,
    change_type: StockChangeType,
}

impl StockChanged {
    /// Modela la operacion `received` dentro del ejemplo del patron.
    pub fn received(sku: impl Into<String>, quantity: u32, available_after_change: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
            available_after_change,
            change_type: StockChangeType::Received,
        }
    }

    /// Modela la operacion `reserved` dentro del ejemplo del patron.
    pub fn reserved(sku: impl Into<String>, quantity: u32, available_after_change: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
            available_after_change,
            change_type: StockChangeType::Reserved,
        }
    }

    /// Operacion `description` definida por la abstraccion del ejemplo.
    fn description(&self) -> String {
        match self.change_type {
            StockChangeType::Received => format!("received {}", self.quantity),
            StockChangeType::Reserved => format!("reserved {}", self.quantity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StockChangeType {
    Received,
    Reserved,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryReadModel` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryReadModel {
    available_by_sku: HashMap<String, u32>,
    last_event_by_sku: HashMap<String, String>,
}

impl InventoryReadModel {
    /// Modela la operacion `available` dentro del ejemplo del patron.
    pub fn available(&self, sku: &str) -> u32 {
        self.available_by_sku.get(sku).copied().unwrap_or_default()
    }

    /// Modela la operacion `last event for` dentro del ejemplo del patron.
    pub fn last_event_for(&self, sku: &str) -> Option<&str> {
        self.last_event_by_sku.get(sku).map(String::as_str)
    }
}

#[derive(Debug)]
/// Tipo publico `InventoryProjection` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryProjection<'a> {
    read_model: &'a mut InventoryReadModel,
}

impl<'a> InventoryProjection<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(read_model: &'a mut InventoryReadModel) -> Self {
        Self { read_model }
    }

    /// Aplica la regla o transformacion del patron.
    pub fn apply(&mut self, event: StockChanged) {
        self.read_model
            .available_by_sku
            .insert(event.sku.clone(), event.available_after_change);
        self.read_model
            .last_event_by_sku
            .insert(event.sku.clone(), event.description());
    }

    /// Modela la operacion `apply many` dentro del ejemplo del patron.
    pub fn apply_many(&mut self, events: Vec<StockChanged>) {
        for event in events {
            self.apply(event);
        }
    }

    /// Modela la operacion `rebuild` dentro del ejemplo del patron.
    pub fn rebuild(events: Vec<StockChanged>) -> InventoryReadModel {
        let mut read_model = InventoryReadModel::default();

        for event in events {
            read_model
                .available_by_sku
                .insert(event.sku.clone(), event.available_after_change);
            read_model
                .last_event_by_sku
                .insert(event.sku.clone(), event.description());
        }

        read_model
    }
}
