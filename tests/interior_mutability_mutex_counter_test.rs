use std::sync::Arc;
use std::thread;

use design_patterns_rust::patterns::rust_idiomatic::interior_mutability::mutex_counter::{
    MetricSnapshot, MetricsCounter,
};

#[test]
fn interior_mutability_mutex_counter_tracks_named_metrics() {
    let counter = MetricsCounter::new();

    counter.increment("orders.created");
    counter.increment_by("orders.created", 2);
    counter.increment("payments.failed");

    assert_eq!(counter.value("orders.created"), 3);
    assert_eq!(counter.value("payments.failed"), 1);
    assert_eq!(counter.value("missing.metric"), 0);
}

#[test]
fn interior_mutability_mutex_counter_returns_sorted_snapshot() {
    let counter = MetricsCounter::new();

    counter.increment("payments.failed");
    counter.increment_by("orders.created", 2);

    assert_eq!(
        counter.snapshot(),
        vec![
            MetricSnapshot::new("orders.created", 2),
            MetricSnapshot::new("payments.failed", 1)
        ]
    );
}

#[test]
fn interior_mutability_mutex_counter_is_safe_to_share_between_threads() {
    let counter = Arc::new(MetricsCounter::new());
    let handles = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    counter.increment("jobs.processed");
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().expect("worker should finish");
    }

    assert_eq!(counter.value("jobs.processed"), 400);
}
