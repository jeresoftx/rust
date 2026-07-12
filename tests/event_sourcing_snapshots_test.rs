use design_patterns_rust::patterns::architecture::event_sourcing::snapshots::{
    Counter, CounterEvent, CounterSnapshot, RebuildReport,
};

#[test]
fn event_sourcing_snapshot_rebuilds_from_snapshot_plus_later_events() {
    let snapshot = CounterSnapshot::new(42, 100);
    let events = vec![
        CounterEvent::incremented(43, 5),
        CounterEvent::decremented(44, 2),
        CounterEvent::incremented(45, 7),
    ];

    let report = Counter::rebuild_from_snapshot(snapshot, &events).unwrap();

    assert_eq!(report.counter().value(), 110);
    assert_eq!(report.counter().version(), 45);
    assert_eq!(report.events_replayed(), 3);
}

#[test]
fn event_sourcing_snapshot_ignores_events_already_included_in_snapshot() {
    let snapshot = CounterSnapshot::new(10, 50);
    let events = vec![
        CounterEvent::incremented(9, 999),
        CounterEvent::incremented(10, 999),
        CounterEvent::incremented(11, 5),
    ];

    let report = Counter::rebuild_from_snapshot(snapshot, &events).unwrap();

    assert_eq!(report, RebuildReport::new(Counter::restored(11, 55), 1));
}

#[test]
fn event_sourcing_snapshot_detects_version_gaps_after_snapshot() {
    let snapshot = CounterSnapshot::new(7, 20);
    let events = vec![CounterEvent::incremented(9, 3)];

    let result = Counter::rebuild_from_snapshot(snapshot, &events);

    assert!(result.is_err());
}
