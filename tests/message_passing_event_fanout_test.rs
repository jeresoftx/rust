use design_patterns_rust::patterns::rust_idiomatic::message_passing::event_fanout::{
    DomainEvent, EventFanout,
};

#[test]
fn message_passing_fanout_delivers_event_to_every_consumer() {
    let mut fanout = EventFanout::new();
    let audit = fanout.subscribe("audit-log");
    let email = fanout.subscribe("email-service");

    fanout.publish(DomainEvent::new("OrderPaid", "ORD-1"));

    assert_eq!(audit.name(), "audit-log");
    assert_eq!(email.name(), "email-service");
    assert_eq!(audit.drain(), vec![DomainEvent::new("OrderPaid", "ORD-1")]);
    assert_eq!(email.drain(), vec![DomainEvent::new("OrderPaid", "ORD-1")]);
}

#[test]
fn message_passing_fanout_preserves_order_per_consumer() {
    let mut fanout = EventFanout::new();
    let analytics = fanout.subscribe("analytics");

    fanout.publish(DomainEvent::new("CartCreated", "CART-1"));
    fanout.publish(DomainEvent::new("CartCheckedOut", "CART-1"));

    assert_eq!(
        analytics.drain(),
        vec![
            DomainEvent::new("CartCreated", "CART-1"),
            DomainEvent::new("CartCheckedOut", "CART-1")
        ]
    );
}

#[test]
fn message_passing_fanout_ignores_closed_consumers() {
    let mut fanout = EventFanout::new();
    let closed = fanout.subscribe("closed-consumer");
    let billing = fanout.subscribe("billing");
    drop(closed);

    fanout.publish(DomainEvent::new("InvoiceIssued", "INV-7"));

    assert_eq!(
        billing.drain(),
        vec![DomainEvent::new("InvoiceIssued", "INV-7")]
    );
}
