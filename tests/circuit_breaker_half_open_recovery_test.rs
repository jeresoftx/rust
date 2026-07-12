use design_patterns_rust::patterns::distributed_systems::circuit_breaker::half_open_recovery::{
    CircuitBreaker, CircuitError, CircuitState, DependencyError, SimulatedDependency,
};

#[test]
fn half_open_success_closes_the_circuit_after_cooldown() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("ok".to_string()),
    ]);
    let mut breaker = CircuitBreaker::new(1, 10);

    let _ = breaker.call(&mut dependency, 0);
    assert_eq!(breaker.state_at(0), CircuitState::Open);

    assert_eq!(breaker.call(&mut dependency, 9), Err(CircuitError::Open));
    assert_eq!(dependency.calls(), 1);

    assert_eq!(breaker.state_at(10), CircuitState::HalfOpen);
    assert_eq!(breaker.call(&mut dependency, 10), Ok("ok".to_string()));
    assert_eq!(breaker.state_at(10), CircuitState::Closed);
    assert_eq!(dependency.calls(), 2);
}

#[test]
fn half_open_failure_reopens_the_circuit_and_restarts_cooldown() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Err(DependencyError::Unavailable),
        Ok("recovered".to_string()),
    ]);
    let mut breaker = CircuitBreaker::new(1, 10);

    let _ = breaker.call(&mut dependency, 0);
    assert_eq!(breaker.state_at(10), CircuitState::HalfOpen);

    assert_eq!(
        breaker.call(&mut dependency, 10),
        Err(CircuitError::DependencyFailed(DependencyError::Unavailable))
    );
    assert_eq!(breaker.state_at(10), CircuitState::Open);

    assert_eq!(breaker.call(&mut dependency, 19), Err(CircuitError::Open));
    assert_eq!(dependency.calls(), 2);

    assert_eq!(breaker.state_at(20), CircuitState::HalfOpen);
    assert_eq!(
        breaker.call(&mut dependency, 20),
        Ok("recovered".to_string())
    );
    assert_eq!(breaker.state_at(20), CircuitState::Closed);
}

#[test]
fn closed_state_resets_failures_after_success() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("ok".to_string()),
        Err(DependencyError::Unavailable),
    ]);
    let mut breaker = CircuitBreaker::new(2, 10);

    let _ = breaker.call(&mut dependency, 0);
    assert_eq!(breaker.consecutive_failures(), 1);

    assert_eq!(breaker.call(&mut dependency, 1), Ok("ok".to_string()));
    assert_eq!(breaker.consecutive_failures(), 0);
    assert_eq!(breaker.state_at(1), CircuitState::Closed);

    let _ = breaker.call(&mut dependency, 2);
    assert_eq!(breaker.consecutive_failures(), 1);
    assert_eq!(breaker.state_at(2), CircuitState::Closed);
}
