use design_patterns_rust::patterns::architecture::domain_driven_design_tactical::domain_events::{
    DomainEvent, InternalIntegrationHandler, Order, OrderConfirmed, OrderId,
};

#[test]
fn ddd_domain_events_are_recorded_when_aggregate_changes_state() {
    let mut order = Order::new(OrderId::new("ORD-2001"));
    order.add_item("SKU-BOOK", 2);

    order.confirm().unwrap();

    assert_eq!(order.status(), "confirmed");
    assert_eq!(
        order.pull_events(),
        vec![DomainEvent::OrderConfirmed(OrderConfirmed::new(
            "ORD-2001", 2
        ))]
    );
    assert!(order.pull_events().is_empty());
}

#[test]
fn ddd_domain_events_feed_internal_integrations_without_coupling_the_aggregate() {
    let mut order = Order::new(OrderId::new("ORD-2002"));
    order.add_item("SKU-BOOK", 1);
    order.confirm().unwrap();

    let mut handler = InternalIntegrationHandler::default();
    handler.handle_many(order.pull_events());

    assert_eq!(
        handler.actions(),
        vec![
            "reserve inventory for ORD-2002 with 1 items",
            "notify billing for ORD-2002"
        ]
    );
}

#[test]
fn ddd_domain_events_prevent_duplicate_confirmation_events() {
    let mut order = Order::new(OrderId::new("ORD-2003"));
    order.add_item("SKU-BOOK", 1);

    order.confirm().unwrap();
    assert!(order.confirm().is_err());

    let events = order.pull_events();
    assert_eq!(events.len(), 1);
}
