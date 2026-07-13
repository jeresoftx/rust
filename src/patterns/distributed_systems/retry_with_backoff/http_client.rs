use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `HttpError` dentro del ejemplo.
pub enum HttpError {
    /// Variante `ServiceUnavailable` del estado o error del ejemplo.
    ServiceUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `BackoffPolicy` usado por el ejemplo para expresar el dominio del patron.
pub struct BackoffPolicy {
    max_attempts: usize,
    initial_delay_ms: u64,
}

impl BackoffPolicy {
    /// Modela la operacion `exponential` dentro del ejemplo del patron.
    pub fn exponential(max_attempts: usize, initial_delay_ms: u64) -> Self {
        Self {
            max_attempts,
            initial_delay_ms,
        }
    }

    /// Operacion `delay for retry` definida por la abstraccion del ejemplo.
    fn delay_for_retry(&self, retry_index: usize) -> u64 {
        self.initial_delay_ms * 2_u64.pow(retry_index as u32)
    }
}

#[derive(Debug)]
/// Tipo publico `SimulatedHttpClient` usado por el ejemplo para expresar el dominio del patron.
pub struct SimulatedHttpClient {
    responses: VecDeque<Result<String, HttpError>>,
    attempts: usize,
}

impl SimulatedHttpClient {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(responses: Vec<Result<String, HttpError>>) -> Self {
        Self {
            responses: responses.into(),
            attempts: 0,
        }
    }

    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&mut self, _path: &str) -> Result<String, HttpError> {
        self.attempts += 1;
        self.responses
            .pop_front()
            .unwrap_or(Err(HttpError::ServiceUnavailable))
    }
}

#[derive(Debug)]
/// Tipo publico `RetryHttpClient` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryHttpClient {
    inner: SimulatedHttpClient,
    policy: BackoffPolicy,
    recorded_delays_ms: Vec<u64>,
}

impl RetryHttpClient {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: SimulatedHttpClient, policy: BackoffPolicy) -> Self {
        Self {
            inner,
            policy,
            recorded_delays_ms: Vec::new(),
        }
    }

    /// Modela la operacion `get` dentro del ejemplo del patron.
    pub fn get(&mut self, path: &str) -> Result<String, HttpError> {
        let max_attempts = self.policy.max_attempts.max(1);

        for attempt_index in 0..max_attempts {
            match self.inner.get(path) {
                Ok(response) => return Ok(response),
                Err(error) if attempt_index + 1 == max_attempts => return Err(error),
                Err(_) => self
                    .recorded_delays_ms
                    .push(self.policy.delay_for_retry(attempt_index)),
            }
        }

        unreachable!("max_attempts is normalized to at least one")
    }

    /// Modela la operacion `attempts` dentro del ejemplo del patron.
    pub fn attempts(&self) -> usize {
        self.inner.attempts
    }

    /// Modela la operacion `recorded delays ms` dentro del ejemplo del patron.
    pub fn recorded_delays_ms(&self) -> Vec<u64> {
        self.recorded_delays_ms.clone()
    }
}
