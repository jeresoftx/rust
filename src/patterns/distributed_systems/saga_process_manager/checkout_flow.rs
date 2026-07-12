#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagaState {
    Started,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagaLog {
    InventoryReserved,
    PaymentCaptured,
    ShipmentScheduled,
}

#[derive(Debug)]
pub struct CheckoutSaga {
    order_id: String,
    state: SagaState,
    log: Vec<SagaLog>,
}

impl CheckoutSaga {
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            state: SagaState::Started,
            log: Vec::new(),
        }
    }

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

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn state(&self) -> SagaState {
        self.state
    }

    pub fn log(&self) -> &[SagaLog] {
        &self.log
    }
}
