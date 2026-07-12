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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    pub dependency_calls: usize,
    pub successes: usize,
    pub failures: usize,
    pub open_rejections: usize,
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
    metrics: Metrics,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize) -> Self {
        Self {
            failure_threshold,
            consecutive_failures: 0,
            state: CircuitState::Closed,
            metrics: Metrics::default(),
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }

    pub fn metrics(&self) -> Metrics {
        self.metrics
    }

    pub fn call(&mut self, dependency: &mut SimulatedDependency) -> Result<String, CircuitError> {
        if self.state == CircuitState::Open {
            self.metrics.open_rejections += 1;
            return Err(CircuitError::Open);
        }

        self.metrics.dependency_calls += 1;
        match dependency.call() {
            Ok(value) => {
                self.metrics.successes += 1;
                self.consecutive_failures = 0;
                Ok(value)
            }
            Err(error) => {
                self.metrics.failures += 1;
                self.consecutive_failures += 1;
                if self.consecutive_failures >= self.failure_threshold {
                    self.state = CircuitState::Open;
                }
                Err(CircuitError::DependencyFailed(error))
            }
        }
    }
}
