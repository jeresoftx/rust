use design_patterns_rust::patterns::distributed_systems::retry_with_backoff::http_client::{
    BackoffPolicy, HttpError, RetryHttpClient, SimulatedHttpClient,
};

#[test]
fn retry_client_succeeds_after_transient_failures_with_exponential_backoff() {
    let inner = SimulatedHttpClient::new(vec![
        Err(HttpError::ServiceUnavailable),
        Err(HttpError::ServiceUnavailable),
        Ok("ok".to_string()),
    ]);
    let policy = BackoffPolicy::exponential(3, 100);
    let mut client = RetryHttpClient::new(inner, policy);

    let response = client.get("/orders").expect("third attempt should succeed");

    assert_eq!(response, "ok");
    assert_eq!(client.attempts(), 3);
    assert_eq!(client.recorded_delays_ms(), vec![100, 200]);
}

#[test]
fn retry_client_returns_last_error_when_attempts_are_exhausted() {
    let inner = SimulatedHttpClient::new(vec![
        Err(HttpError::ServiceUnavailable),
        Err(HttpError::ServiceUnavailable),
        Err(HttpError::ServiceUnavailable),
    ]);
    let policy = BackoffPolicy::exponential(3, 50);
    let mut client = RetryHttpClient::new(inner, policy);

    let result = client.get("/orders");

    assert_eq!(result, Err(HttpError::ServiceUnavailable));
    assert_eq!(client.attempts(), 3);
    assert_eq!(client.recorded_delays_ms(), vec![50, 100]);
}

#[test]
fn retry_client_does_not_wait_after_the_final_attempt() {
    let inner = SimulatedHttpClient::new(vec![Err(HttpError::ServiceUnavailable)]);
    let policy = BackoffPolicy::exponential(1, 100);
    let mut client = RetryHttpClient::new(inner, policy);

    let result = client.get("/orders");

    assert_eq!(result, Err(HttpError::ServiceUnavailable));
    assert_eq!(client.attempts(), 1);
    assert!(client.recorded_delays_ms().is_empty());
}
