use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
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
    cooldown_ticks: u64,
    consecutive_failures: usize,
    state: CircuitState,
    opened_at_tick: Option<u64>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, cooldown_ticks: u64) -> Self {
        Self {
            failure_threshold,
            cooldown_ticks,
            consecutive_failures: 0,
            state: CircuitState::Closed,
            opened_at_tick: None,
        }
    }

    pub fn consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }

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
