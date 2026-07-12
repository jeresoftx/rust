use design_patterns_rust::patterns::distributed_systems::outbox_pattern::unit_of_work::{
    InMemoryUnitOfWork, Order, OutboxMessage,
};

#[test]
fn saves_order_and_outbox_message_together() {
    let mut uow = InMemoryUnitOfWork::new();

    uow.create_order("order-1", "customer-1");

    assert_eq!(uow.orders(), &[Order::new("order-1", "customer-1")]);
    assert_eq!(
        uow.pending_messages(),
        &[OutboxMessage::new("msg-1", "OrderCreated", "order-1")]
    );
}

#[test]
fn failed_unit_of_work_does_not_save_partial_data() {
    let mut uow = InMemoryUnitOfWork::new();

    let result = uow.create_order_with_failure("order-1", "customer-1");

    assert!(result.is_err());
    assert!(uow.orders().is_empty());
    assert!(uow.pending_messages().is_empty());
}

#[test]
fn creates_one_message_per_order() {
    let mut uow = InMemoryUnitOfWork::new();

    uow.create_order("order-1", "customer-1");
    uow.create_order("order-2", "customer-2");

    assert_eq!(uow.orders().len(), 2);
    assert_eq!(uow.pending_messages().len(), 2);
    assert_eq!(uow.pending_messages()[1].aggregate_id(), "order-2");
}
