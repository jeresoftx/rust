use design_patterns_rust::patterns::distributed_systems::circuit_breaker::consecutive_failures::{
    CircuitBreaker, CircuitError, CircuitState, DependencyError, SimulatedDependency,
};

#[test]
fn opens_circuit_after_consecutive_failures() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Err(DependencyError::Unavailable),
        Ok("recovered".to_string()),
    ]);
    let mut breaker = CircuitBreaker::new(2);

    assert_eq!(
        breaker.call(&mut dependency),
        Err(CircuitError::DependencyFailed(DependencyError::Unavailable))
    );
    assert_eq!(breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.consecutive_failures(), 1);

    assert_eq!(
        breaker.call(&mut dependency),
        Err(CircuitError::DependencyFailed(DependencyError::Unavailable))
    );
    assert_eq!(breaker.state(), CircuitState::Open);
    assert_eq!(breaker.consecutive_failures(), 2);

    assert_eq!(breaker.call(&mut dependency), Err(CircuitError::Open));
    assert_eq!(dependency.calls(), 2);
}

#[test]
fn successful_call_resets_consecutive_failures() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("ok".to_string()),
        Err(DependencyError::Unavailable),
    ]);
    let mut breaker = CircuitBreaker::new(2);

    let _ = breaker.call(&mut dependency);
    assert_eq!(breaker.consecutive_failures(), 1);

    assert_eq!(breaker.call(&mut dependency), Ok("ok".to_string()));
    assert_eq!(breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.consecutive_failures(), 0);

    let _ = breaker.call(&mut dependency);
    assert_eq!(breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.consecutive_failures(), 1);
}

#[test]
fn open_circuit_rejects_without_calling_dependency() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("would not be called".to_string()),
    ]);
    let mut breaker = CircuitBreaker::new(1);

    let _ = breaker.call(&mut dependency);
    assert_eq!(breaker.state(), CircuitState::Open);

    assert_eq!(breaker.call(&mut dependency), Err(CircuitError::Open));
    assert_eq!(dependency.calls(), 1);
}
