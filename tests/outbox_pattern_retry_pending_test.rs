use design_patterns_rust::patterns::distributed_systems::outbox_pattern::retry_pending::{
    FlakyBroker, RetryPublisher, RetryStore,
};

#[test]
fn failed_publish_keeps_message_pending_for_retry() {
    let mut store = RetryStore::with_pending(["msg-1"]);
    let mut broker = FlakyBroker::fail_first_then_succeed();
    let mut publisher = RetryPublisher::new(&mut broker);

    let first = publisher.publish_pending(&mut store);

    assert_eq!(first.sent, 0);
    assert_eq!(first.failed, 1);
    assert_eq!(store.pending_ids(), &["msg-1".to_string()]);
}

#[test]
fn retry_marks_message_sent_after_later_success() {
    let mut store = RetryStore::with_pending(["msg-1"]);
    let mut broker = FlakyBroker::fail_first_then_succeed();

    {
        let mut publisher = RetryPublisher::new(&mut broker);
        publisher.publish_pending(&mut store);
        let second = publisher.publish_pending(&mut store);
        assert_eq!(second.sent, 1);
        assert_eq!(second.failed, 0);
    }

    assert!(store.pending_ids().is_empty());
    assert_eq!(broker.attempts_for("msg-1"), 2);
}

#[test]
fn successful_messages_are_not_retried() {
    let mut store = RetryStore::with_pending(["msg-1", "msg-2"]);
    let mut broker = FlakyBroker::always_succeeds();
    let mut publisher = RetryPublisher::new(&mut broker);

    publisher.publish_pending(&mut store);
    publisher.publish_pending(&mut store);

    assert_eq!(broker.attempts_for("msg-1"), 1);
    assert_eq!(broker.attempts_for("msg-2"), 1);
}
