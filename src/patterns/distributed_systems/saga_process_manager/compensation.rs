#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SagaState` dentro del ejemplo.
pub enum SagaState {
    /// Variante `Started` del estado o error del ejemplo.
    Started,
    /// Variante `Completed` del estado o error del ejemplo.
    Completed,
    /// Variante `Compensated` del estado o error del ejemplo.
    Compensated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SagaLog` dentro del ejemplo.
pub enum SagaLog {
    /// Variante `InventoryReserved` del estado o error del ejemplo.
    InventoryReserved,
    /// Variante `PaymentCaptured` del estado o error del ejemplo.
    PaymentCaptured,
    /// Variante `PaymentFailed` del estado o error del ejemplo.
    PaymentFailed,
    /// Variante `InventoryReleased` del estado o error del ejemplo.
    InventoryReleased,
}

#[derive(Debug)]
/// Tipo publico `CompensatingSaga` usado por el ejemplo para expresar el dominio del patron.
pub struct CompensatingSaga {
    order_id: String,
    payment_fails: bool,
    state: SagaState,
    log: Vec<SagaLog>,
    failure_reason: Option<String>,
}

impl CompensatingSaga {
    /// Modela la operacion `payment will fail` dentro del ejemplo del patron.
    pub fn payment_will_fail(order_id: impl Into<String>) -> Self {
        Self::new(order_id, true)
    }

    /// Modela la operacion `payment will succeed` dentro del ejemplo del patron.
    pub fn payment_will_succeed(order_id: impl Into<String>) -> Self {
        Self::new(order_id, false)
    }

    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(order_id: impl Into<String>, payment_fails: bool) -> Self {
        Self {
            order_id: order_id.into(),
            payment_fails,
            state: SagaState::Started,
            log: Vec::new(),
            failure_reason: None,
        }
    }

    /// Ejecuta el flujo principal del ejemplo.
    pub fn run(&mut self) -> SagaState {
        if self.state != SagaState::Started {
            return self.state;
        }

        let _order_id = &self.order_id;
        self.log.push(SagaLog::InventoryReserved);
        if self.payment_fails {
            self.log.push(SagaLog::PaymentFailed);
            self.log.push(SagaLog::InventoryReleased);
            self.failure_reason = Some("payment declined".to_string());
            self.state = SagaState::Compensated;
        } else {
            self.log.push(SagaLog::PaymentCaptured);
            self.state = SagaState::Completed;
        }
        self.state
    }

    /// Modela la operacion `log` dentro del ejemplo del patron.
    pub fn log(&self) -> &[SagaLog] {
        &self.log
    }

    /// Modela la operacion `failure reason` dentro del ejemplo del patron.
    pub fn failure_reason(&self) -> Option<&str> {
        self.failure_reason.as_deref()
    }
}
