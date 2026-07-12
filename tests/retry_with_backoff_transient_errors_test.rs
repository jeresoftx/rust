use design_patterns_rust::patterns::distributed_systems::retry_with_backoff::transient_errors::{
    BackoffPolicy, OperationError, RetryExecutor, SimulatedOperation,
};

#[test]
fn retry_executor_retries_transient_errors_until_success() {
    let operation = SimulatedOperation::new(vec![
        Err(OperationError::Transient("timeout".to_string())),
        Err(OperationError::Transient("connection reset".to_string())),
        Ok("processed".to_string()),
    ]);
    let policy = BackoffPolicy::fixed(4, 25);
    let mut executor = RetryExecutor::new(operation, policy);

    let result = executor.run();

    assert_eq!(result, Ok("processed".to_string()));
    assert_eq!(executor.attempts(), 3);
    assert_eq!(executor.recorded_delays_ms(), vec![25, 25]);
}

#[test]
fn retry_executor_fails_fast_on_permanent_error() {
    let operation = SimulatedOperation::new(vec![Err(OperationError::Permanent(
        "invalid payload".to_string(),
    ))]);
    let policy = BackoffPolicy::fixed(4, 25);
    let mut executor = RetryExecutor::new(operation, policy);

    let result = executor.run();

    assert_eq!(
        result,
        Err(OperationError::Permanent("invalid payload".to_string()))
    );
    assert_eq!(executor.attempts(), 1);
    assert!(executor.recorded_delays_ms().is_empty());
}

#[test]
fn retry_executor_returns_last_transient_error_after_budget_is_exhausted() {
    let operation = SimulatedOperation::new(vec![
        Err(OperationError::Transient("timeout".to_string())),
        Err(OperationError::Transient("still unavailable".to_string())),
    ]);
    let policy = BackoffPolicy::fixed(2, 10);
    let mut executor = RetryExecutor::new(operation, policy);

    let result = executor.run();

    assert_eq!(
        result,
        Err(OperationError::Transient("still unavailable".to_string()))
    );
    assert_eq!(executor.attempts(), 2);
    assert_eq!(executor.recorded_delays_ms(), vec![10]);
}
