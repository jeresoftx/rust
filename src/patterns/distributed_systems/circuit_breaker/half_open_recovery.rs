use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CircuitState` dentro del ejemplo.
pub enum CircuitState {
    /// Variante `Closed` del estado o error del ejemplo.
    Closed,
    /// Variante `Open` del estado o error del ejemplo.
    Open,
    /// Variante `HalfOpen` del estado o error del ejemplo.
    HalfOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `DependencyError` dentro del ejemplo.
pub enum DependencyError {
    /// Variante `Unavailable` del estado o error del ejemplo.
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CircuitError` dentro del ejemplo.
pub enum CircuitError {
    /// Variante `DependencyFailed` del estado o error del ejemplo.
    DependencyFailed(DependencyError),
    /// Variante `Open` del estado o error del ejemplo.
    Open,
}

#[derive(Debug)]
/// Tipo publico `SimulatedDependency` usado por el ejemplo para expresar el dominio del patron.
pub struct SimulatedDependency {
    responses: VecDeque<Result<String, DependencyError>>,
    calls: usize,
}

impl SimulatedDependency {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(responses: Vec<Result<String, DependencyError>>) -> Self {
        Self {
            responses: responses.into(),
            calls: 0,
        }
    }

    /// Modela la operacion `calls` dentro del ejemplo del patron.
    pub fn calls(&self) -> usize {
        self.calls
    }

    /// Operacion `call` definida por la abstraccion del ejemplo.
    fn call(&mut self) -> Result<String, DependencyError> {
        self.calls += 1;
        self.responses
            .pop_front()
            .unwrap_or(Err(DependencyError::Unavailable))
    }
}

#[derive(Debug)]
/// Tipo publico `CircuitBreaker` usado por el ejemplo para expresar el dominio del patron.
pub struct CircuitBreaker {
    failure_threshold: usize,
    cooldown_ticks: u64,
    consecutive_failures: usize,
    state: CircuitState,
    opened_at_tick: Option<u64>,
}

impl CircuitBreaker {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(failure_threshold: usize, cooldown_ticks: u64) -> Self {
        Self {
            failure_threshold,
            cooldown_ticks,
            consecutive_failures: 0,
            state: CircuitState::Closed,
            opened_at_tick: None,
        }
    }

    /// Modela la operacion `consecutive failures` dentro del ejemplo del patron.
    pub fn consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }

    /// Modela la operacion `state at` dentro del ejemplo del patron.
    pub fn state_at(&self, now_tick: u64) -> CircuitState {
        match (self.state, self.opened_at_tick) {
            (CircuitState::Open, Some(opened_at))
                if now_tick >= opened_at + self.cooldown_ticks =>
            {
                CircuitState::HalfOpen
            }
            _ => self.state,
        }
    }

    /// Modela la operacion `call` dentro del ejemplo del patron.
    pub fn call(
        &mut self,
        dependency: &mut SimulatedDependency,
        now_tick: u64,
    ) -> Result<String, CircuitError> {
        let current_state = self.state_at(now_tick);
        if current_state == CircuitState::Open {
            return Err(CircuitError::Open);
        }

        match dependency.call() {
            Ok(value) => {
                self.state = CircuitState::Closed;
                self.opened_at_tick = None;
                self.consecutive_failures = 0;
                Ok(value)
            }
            Err(error) => {
                self.consecutive_failures += 1;
                if current_state == CircuitState::HalfOpen
                    || self.consecutive_failures >= self.failure_threshold
                {
                    self.state = CircuitState::Open;
                    self.opened_at_tick = Some(now_tick);
                }
                Err(CircuitError::DependencyFailed(error))
            }
        }
    }
}
