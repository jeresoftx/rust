use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OperationError` dentro del ejemplo.
pub enum OperationError {
    /// Variante `Transient` del estado o error del ejemplo.
    Transient(String),
    /// Variante `Permanent` del estado o error del ejemplo.
    Permanent(String),
}

impl OperationError {
    /// Operacion `is transient` definida por la abstraccion del ejemplo.
    fn is_transient(&self) -> bool {
        matches!(self, OperationError::Transient(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `BackoffPolicy` usado por el ejemplo para expresar el dominio del patron.
pub struct BackoffPolicy {
    max_attempts: usize,
    delay_ms: u64,
}

impl BackoffPolicy {
    /// Modela la operacion `fixed` dentro del ejemplo del patron.
    pub fn fixed(max_attempts: usize, delay_ms: u64) -> Self {
        Self {
            max_attempts,
            delay_ms,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `SimulatedOperation` usado por el ejemplo para expresar el dominio del patron.
pub struct SimulatedOperation {
    outcomes: VecDeque<Result<String, OperationError>>,
    attempts: usize,
}

impl SimulatedOperation {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(outcomes: Vec<Result<String, OperationError>>) -> Self {
        Self {
            outcomes: outcomes.into(),
            attempts: 0,
        }
    }

    /// Operacion `run` definida por la abstraccion del ejemplo.
    fn run(&mut self) -> Result<String, OperationError> {
        self.attempts += 1;
        self.outcomes
            .pop_front()
            .unwrap_or(Err(OperationError::Transient(
                "sin respuesta simulada".to_string(),
            )))
    }
}

#[derive(Debug)]
/// Tipo publico `RetryExecutor` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryExecutor {
    operation: SimulatedOperation,
    policy: BackoffPolicy,
    recorded_delays_ms: Vec<u64>,
}

impl RetryExecutor {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(operation: SimulatedOperation, policy: BackoffPolicy) -> Self {
        Self {
            operation,
            policy,
            recorded_delays_ms: Vec::new(),
        }
    }

    /// Ejecuta el flujo principal del ejemplo.
    pub fn run(&mut self) -> Result<String, OperationError> {
        let max_attempts = self.policy.max_attempts.max(1);

        for attempt_index in 0..max_attempts {
            match self.operation.run() {
                Ok(value) => return Ok(value),
                Err(error) if !error.is_transient() => return Err(error),
                Err(error) if attempt_index + 1 == max_attempts => return Err(error),
                Err(_) => self.recorded_delays_ms.push(self.policy.delay_ms),
            }
        }

        unreachable!("max_attempts is normalized to at least one")
    }

    /// Modela la operacion `attempts` dentro del ejemplo del patron.
    pub fn attempts(&self) -> usize {
        self.operation.attempts
    }

    /// Modela la operacion `recorded delays ms` dentro del ejemplo del patron.
    pub fn recorded_delays_ms(&self) -> Vec<u64> {
        self.recorded_delays_ms.clone()
    }
}
