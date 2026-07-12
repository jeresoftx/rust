#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagaState {
    Started,
    Completed,
    Compensated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagaLog {
    InventoryReserved,
    PaymentCaptured,
    PaymentFailed,
    InventoryReleased,
}

#[derive(Debug)]
pub struct CompensatingSaga {
    order_id: String,
    payment_fails: bool,
    state: SagaState,
    log: Vec<SagaLog>,
    failure_reason: Option<String>,
}

impl CompensatingSaga {
    pub fn payment_will_fail(order_id: impl Into<String>) -> Self {
        Self::new(order_id, true)
    }

    pub fn payment_will_succeed(order_id: impl Into<String>) -> Self {
        Self::new(order_id, false)
    }

    fn new(order_id: impl Into<String>, payment_fails: bool) -> Self {
        Self {
            order_id: order_id.into(),
            payment_fails,
            state: SagaState::Started,
            log: Vec::new(),
            failure_reason: None,
        }
    }

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

    pub fn log(&self) -> &[SagaLog] {
        &self.log
    }

    pub fn failure_reason(&self) -> Option<&str> {
        self.failure_reason.as_deref()
    }
}
