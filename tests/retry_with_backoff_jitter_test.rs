use design_patterns_rust::patterns::distributed_systems::retry_with_backoff::deterministic_jitter::{
    JitteredBackoffPolicy, RetrySchedule,
};

#[test]
fn jittered_backoff_adds_deterministic_offset_per_node() {
    let policy = JitteredBackoffPolicy::new(100, 25);

    let node_a = RetrySchedule::new("worker-a", policy.clone()).delays_for_retries(3);
    let node_b = RetrySchedule::new("worker-b", policy).delays_for_retries(3);

    assert_eq!(node_a, vec![108, 208, 408]);
    assert_eq!(node_b, vec![109, 209, 409]);
}

#[test]
fn jittered_backoff_is_stable_for_the_same_node() {
    let policy = JitteredBackoffPolicy::new(50, 10);

    let first = RetrySchedule::new("worker-a", policy.clone()).delays_for_retries(4);
    let second = RetrySchedule::new("worker-a", policy).delays_for_retries(4);

    assert_eq!(first, second);
    assert_eq!(first, vec![58, 108, 208, 408]);
}

#[test]
fn jittered_backoff_caps_jitter_to_configured_range() {
    let policy = JitteredBackoffPolicy::new(100, 3);

    let delays = RetrySchedule::new("worker-z", policy).delays_for_retries(2);

    assert_eq!(delays, vec![102, 202]);
}
