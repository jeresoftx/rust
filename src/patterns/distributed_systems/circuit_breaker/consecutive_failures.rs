use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CircuitState` dentro del ejemplo.
pub enum CircuitState {
    /// Variante `Closed` del estado o error del ejemplo.
    Closed,
    /// Variante `Open` del estado o error del ejemplo.
    Open,
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
    consecutive_failures: usize,
    state: CircuitState,
}

impl CircuitBreaker {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(failure_threshold: usize) -> Self {
        Self {
            failure_threshold,
            consecutive_failures: 0,
            state: CircuitState::Closed,
        }
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> CircuitState {
        self.state
    }

    /// Modela la operacion `consecutive failures` dentro del ejemplo del patron.
    pub fn consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }

    /// Modela la operacion `call` dentro del ejemplo del patron.
    pub fn call(&mut self, dependency: &mut SimulatedDependency) -> Result<String, CircuitError> {
        if self.state == CircuitState::Open {
            return Err(CircuitError::Open);
        }

        match dependency.call() {
            Ok(value) => {
                self.consecutive_failures = 0;
                Ok(value)
            }
            Err(error) => {
                self.consecutive_failures += 1;
                if self.consecutive_failures >= self.failure_threshold {
                    self.state = CircuitState::Open;
                }
                Err(CircuitError::DependencyFailed(error))
            }
        }
    }
}
