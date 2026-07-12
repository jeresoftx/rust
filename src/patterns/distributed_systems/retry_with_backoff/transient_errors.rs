use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationError {
    Transient(String),
    Permanent(String),
}

impl OperationError {
    fn is_transient(&self) -> bool {
        matches!(self, OperationError::Transient(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackoffPolicy {
    max_attempts: usize,
    delay_ms: u64,
}

impl BackoffPolicy {
    pub fn fixed(max_attempts: usize, delay_ms: u64) -> Self {
        Self {
            max_attempts,
            delay_ms,
        }
    }
}

#[derive(Debug)]
pub struct SimulatedOperation {
    outcomes: VecDeque<Result<String, OperationError>>,
    attempts: usize,
}

impl SimulatedOperation {
    pub fn new(outcomes: Vec<Result<String, OperationError>>) -> Self {
        Self {
            outcomes: outcomes.into(),
            attempts: 0,
        }
    }

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
pub struct RetryExecutor {
    operation: SimulatedOperation,
    policy: BackoffPolicy,
    recorded_delays_ms: Vec<u64>,
}

impl RetryExecutor {
    pub fn new(operation: SimulatedOperation, policy: BackoffPolicy) -> Self {
        Self {
            operation,
            policy,
            recorded_delays_ms: Vec::new(),
        }
    }

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

    pub fn attempts(&self) -> usize {
        self.operation.attempts
    }

    pub fn recorded_delays_ms(&self) -> Vec<u64> {
        self.recorded_delays_ms.clone()
    }
}
