use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistedSagaState {
    Started,
    InventoryReserved,
    PaymentCaptured,
    Completed,
}

#[derive(Debug, Default)]
pub struct SagaRepository {
    states: HashMap<String, PersistedSagaState>,
}

impl SagaRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&mut self, order_id: &str, state: PersistedSagaState) {
        self.states.insert(order_id.to_string(), state);
    }

    pub fn load(&self, order_id: &str) -> Option<PersistedSagaState> {
        self.states.get(order_id).copied()
    }
}

#[derive(Debug)]
pub struct StatefulSaga {
    order_id: String,
    state: PersistedSagaState,
}

impl StatefulSaga {
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            state: PersistedSagaState::Started,
        }
    }

    pub fn resume(order_id: impl Into<String>, repository: &SagaRepository) -> Option<Self> {
        let order_id = order_id.into();
        repository
            .load(&order_id)
            .map(|state| Self { order_id, state })
    }

    pub fn advance(&mut self, repository: &mut SagaRepository) {
        self.state = match self.state {
            PersistedSagaState::Started => PersistedSagaState::InventoryReserved,
            PersistedSagaState::InventoryReserved => PersistedSagaState::PaymentCaptured,
            PersistedSagaState::PaymentCaptured => PersistedSagaState::Completed,
            PersistedSagaState::Completed => PersistedSagaState::Completed,
        };
        repository.save(&self.order_id, self.state);
    }

    pub fn state(&self) -> PersistedSagaState {
        self.state
    }
}
