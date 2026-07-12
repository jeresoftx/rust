use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyError {
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitError {
    DependencyFailed(DependencyError),
    Open,
}

#[derive(Debug)]
pub struct SimulatedDependency {
    responses: VecDeque<Result<String, DependencyError>>,
    calls: usize,
}

impl SimulatedDependency {
    pub fn new(responses: Vec<Result<String, DependencyError>>) -> Self {
        Self {
            responses: responses.into(),
            calls: 0,
        }
    }

    pub fn calls(&self) -> usize {
        self.calls
    }

    fn call(&mut self) -> Result<String, DependencyError> {
        self.calls += 1;
        self.responses
            .pop_front()
            .unwrap_or(Err(DependencyError::Unavailable))
    }
}

#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    consecutive_failures: usize,
    state: CircuitState,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize) -> Self {
        Self {
            failure_threshold,
            consecutive_failures: 0,
            state: CircuitState::Closed,
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }

    pub fn consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }

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
