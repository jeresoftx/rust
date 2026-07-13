use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PersistedSagaState` dentro del ejemplo.
pub enum PersistedSagaState {
    /// Variante `Started` del estado o error del ejemplo.
    Started,
    /// Variante `InventoryReserved` del estado o error del ejemplo.
    InventoryReserved,
    /// Variante `PaymentCaptured` del estado o error del ejemplo.
    PaymentCaptured,
    /// Variante `Completed` del estado o error del ejemplo.
    Completed,
}

#[derive(Debug, Default)]
/// Tipo publico `SagaRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct SagaRepository {
    states: HashMap<String, PersistedSagaState>,
}

impl SagaRepository {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&mut self, order_id: &str, state: PersistedSagaState) {
        self.states.insert(order_id.to_string(), state);
    }

    /// Modela la operacion `load` dentro del ejemplo del patron.
    pub fn load(&self, order_id: &str) -> Option<PersistedSagaState> {
        self.states.get(order_id).copied()
    }
}

#[derive(Debug)]
/// Tipo publico `StatefulSaga` usado por el ejemplo para expresar el dominio del patron.
pub struct StatefulSaga {
    order_id: String,
    state: PersistedSagaState,
}

impl StatefulSaga {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            state: PersistedSagaState::Started,
        }
    }

    /// Modela la operacion `resume` dentro del ejemplo del patron.
    pub fn resume(order_id: impl Into<String>, repository: &SagaRepository) -> Option<Self> {
        let order_id = order_id.into();
        repository
            .load(&order_id)
            .map(|state| Self { order_id, state })
    }

    /// Modela la operacion `advance` dentro del ejemplo del patron.
    pub fn advance(&mut self, repository: &mut SagaRepository) {
        self.state = match self.state {
            PersistedSagaState::Started => PersistedSagaState::InventoryReserved,
            PersistedSagaState::InventoryReserved => PersistedSagaState::PaymentCaptured,
            PersistedSagaState::PaymentCaptured => PersistedSagaState::Completed,
            PersistedSagaState::Completed => PersistedSagaState::Completed,
        };
        repository.save(&self.order_id, self.state);
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> PersistedSagaState {
        self.state
    }
}
