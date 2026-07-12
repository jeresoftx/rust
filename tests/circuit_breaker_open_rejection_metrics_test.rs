use design_patterns_rust::patterns::distributed_systems::circuit_breaker::open_rejection_metrics::{
    CircuitBreaker, CircuitError, CircuitState, DependencyError, Metrics, SimulatedDependency,
};

#[test]
fn records_open_rejections_without_calling_dependency() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("would not be called".to_string()),
    ]);
    let mut breaker = CircuitBreaker::new(1);

    let _ = breaker.call(&mut dependency);
    assert_eq!(breaker.state(), CircuitState::Open);

    assert_eq!(breaker.call(&mut dependency), Err(CircuitError::Open));
    assert_eq!(breaker.call(&mut dependency), Err(CircuitError::Open));

    assert_eq!(dependency.calls(), 1);
    assert_eq!(
        breaker.metrics(),
        Metrics {
            dependency_calls: 1,
            successes: 0,
            failures: 1,
            open_rejections: 2,
        }
    );
}

#[test]
fn records_successes_and_failures_separately() {
    let mut dependency = SimulatedDependency::new(vec![
        Err(DependencyError::Unavailable),
        Ok("ok".to_string()),
        Err(DependencyError::Unavailable),
    ]);
    let mut breaker = CircuitBreaker::new(2);

    let _ = breaker.call(&mut dependency);
    assert_eq!(breaker.call(&mut dependency), Ok("ok".to_string()));
    let _ = breaker.call(&mut dependency);

    assert_eq!(breaker.state(), CircuitState::Closed);
    assert_eq!(
        breaker.metrics(),
        Metrics {
            dependency_calls: 3,
            successes: 1,
            failures: 2,
            open_rejections: 0,
        }
    );
}

#[test]
fn exposes_rejection_count_for_alerting() {
    let mut dependency = SimulatedDependency::new(vec![Err(DependencyError::Unavailable)]);
    let mut breaker = CircuitBreaker::new(1);

    let _ = breaker.call(&mut dependency);
    for _ in 0..5 {
        let _ = breaker.call(&mut dependency);
    }

    assert_eq!(breaker.metrics().open_rejections, 5);
    assert_eq!(dependency.calls(), 1);
}
