use std::thread;

use design_patterns_rust::patterns::rust_idiomatic::actor_like_workers::metrics_actor::{
    MetricEvent, MetricTotal, MetricsActor,
};

#[test]
fn actor_like_metrics_actor_aggregates_events_by_name() {
    let actor = MetricsActor::start();

    actor.record(MetricEvent::new("orders.created", 1));
    actor.record(MetricEvent::new("orders.created", 2));
    actor.record(MetricEvent::new("payments.failed", 1));

    assert_eq!(actor.total("orders.created"), 3);
    assert_eq!(actor.total("payments.failed"), 1);
    assert_eq!(actor.total("missing.metric"), 0);

    actor.shutdown();
}

#[test]
fn actor_like_metrics_actor_returns_sorted_snapshot() {
    let actor = MetricsActor::start();

    actor.record(MetricEvent::new("payments.failed", 1));
    actor.record(MetricEvent::new("orders.created", 2));

    assert_eq!(
        actor.snapshot(),
        vec![
            MetricTotal::new("orders.created", 2),
            MetricTotal::new("payments.failed", 1)
        ]
    );

    actor.shutdown();
}

#[test]
fn actor_like_metrics_actor_accepts_events_from_cloned_clients() {
    let actor = MetricsActor::start();
    let handles = (0..4)
        .map(|_| {
            let client = actor.client();
            thread::spawn(move || {
                for _ in 0..25 {
                    client.record(MetricEvent::new("jobs.processed", 1));
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().expect("metrics producer should finish");
    }

    assert_eq!(actor.total("jobs.processed"), 100);

    actor.shutdown();
}
