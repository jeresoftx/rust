use design_patterns_rust::patterns::gof::behavioral::observer::order_events::{
    OrderCreated, OrderEventBus,
};

#[test]
fn observer_notifies_all_order_created_subscribers() {
    let mut bus = OrderEventBus::new();
    bus.subscribe("audit-log");
    bus.subscribe("fulfillment");

    bus.publish(OrderCreated::new("order-100", "customer-9", 4500));

    assert_eq!(
        bus.received_by("audit-log"),
        vec!["order-created:order-100:customer-9:4500".to_string()]
    );
    assert_eq!(
        bus.received_by("fulfillment"),
        vec!["order-created:order-100:customer-9:4500".to_string()]
    );
}

#[test]
fn observer_only_notifies_current_subscribers() {
    let mut bus = OrderEventBus::new();
    bus.subscribe("audit-log");
    bus.publish(OrderCreated::new("order-100", "customer-9", 4500));
    bus.subscribe("analytics");
    bus.publish(OrderCreated::new("order-101", "customer-9", 3000));

    assert_eq!(
        bus.received_by("analytics"),
        vec!["order-created:order-101:customer-9:3000".to_string()]
    );
}
