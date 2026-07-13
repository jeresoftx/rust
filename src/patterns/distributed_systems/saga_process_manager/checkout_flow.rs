#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SagaState` dentro del ejemplo.
pub enum SagaState {
    /// Variante `Started` del estado o error del ejemplo.
    Started,
    /// Variante `Completed` del estado o error del ejemplo.
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SagaLog` dentro del ejemplo.
pub enum SagaLog {
    /// Variante `InventoryReserved` del estado o error del ejemplo.
    InventoryReserved,
    /// Variante `PaymentCaptured` del estado o error del ejemplo.
    PaymentCaptured,
    /// Variante `ShipmentScheduled` del estado o error del ejemplo.
    ShipmentScheduled,
}

#[derive(Debug)]
/// Tipo publico `CheckoutSaga` usado por el ejemplo para expresar el dominio del patron.
pub struct CheckoutSaga {
    order_id: String,
    state: SagaState,
    log: Vec<SagaLog>,
}

impl CheckoutSaga {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            state: SagaState::Started,
            log: Vec::new(),
        }
    }

    /// Ejecuta el flujo principal del ejemplo.
    pub fn run(&mut self) -> SagaState {
        if self.state == SagaState::Completed {
            return self.state;
        }

        self.log.push(SagaLog::InventoryReserved);
        self.log.push(SagaLog::PaymentCaptured);
        self.log.push(SagaLog::ShipmentScheduled);
        self.state = SagaState::Completed;
        self.state
    }

    /// Modela la operacion `order id` dentro del ejemplo del patron.
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> SagaState {
        self.state
    }

    /// Modela la operacion `log` dentro del ejemplo del patron.
    pub fn log(&self) -> &[SagaLog] {
        &self.log
    }
}
