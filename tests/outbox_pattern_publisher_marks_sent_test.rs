use design_patterns_rust::patterns::distributed_systems::outbox_pattern::publisher_marks_sent::{
    FakeBroker, OutboxPublisher, OutboxStore,
};

#[test]
fn publisher_sends_pending_messages_and_marks_them_sent() {
    let mut store = OutboxStore::with_pending(["msg-1", "msg-2"]);
    let mut broker = FakeBroker::new();
    let mut publisher = OutboxPublisher::new(&mut broker);

    let sent = publisher.publish_pending(&mut store);

    assert_eq!(sent, 2);
    assert_eq!(
        broker.published_ids(),
        &["msg-1".to_string(), "msg-2".to_string()]
    );
    assert!(store.pending_messages().is_empty());
    assert_eq!(store.sent_messages().len(), 2);
}

#[test]
fn publisher_ignores_already_sent_messages() {
    let mut store = OutboxStore::with_pending(["msg-1"]);
    let mut broker = FakeBroker::new();

    {
        let mut publisher = OutboxPublisher::new(&mut broker);
        publisher.publish_pending(&mut store);
        publisher.publish_pending(&mut store);
    }

    assert_eq!(broker.published_ids(), &["msg-1".to_string()]);
}

#[test]
fn publisher_reports_zero_when_there_is_no_pending_work() {
    let mut store = OutboxStore::new();
    let mut broker = FakeBroker::new();
    let mut publisher = OutboxPublisher::new(&mut broker);

    assert_eq!(publisher.publish_pending(&mut store), 0);
    assert!(broker.published_ids().is_empty());
}
