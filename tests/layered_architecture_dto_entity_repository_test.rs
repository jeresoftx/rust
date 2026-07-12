use design_patterns_rust::patterns::architecture::layered_architecture::dto_entity_repository::{
    application::CreateOrder,
    domain::OrderStatus,
    infrastructure::InMemoryOrderRepository,
    presentation::{CreateOrderDto, OrderController, OrderLineDto},
};

#[test]
fn layered_architecture_keeps_dtos_entities_and_repositories_separate() {
    let repository = InMemoryOrderRepository::default();
    let use_case = CreateOrder::new(repository);
    let mut controller = OrderController::new(use_case);

    let response = controller.create_order(CreateOrderDto::new(
        "ORD-1",
        vec![
            OrderLineDto::new("keyboard", 2),
            OrderLineDto::new("mouse", 1),
        ],
    ));

    assert_eq!(response.status_code(), 201);
    assert_eq!(response.body().order_id(), "ORD-1");
    assert_eq!(response.body().summary(), "ORD-1 accepted with 3 units");
    assert_eq!(
        controller.stored_orders()[0].status(),
        OrderStatus::Accepted
    );
}

#[test]
fn layered_architecture_rejects_invalid_dto_before_repository_save() {
    let repository = InMemoryOrderRepository::default();
    let use_case = CreateOrder::new(repository);
    let mut controller = OrderController::new(use_case);

    let response = controller.create_order(CreateOrderDto::new("ORD-EMPTY", vec![]));

    assert_eq!(response.status_code(), 400);
    assert_eq!(response.error(), Some("order must have at least one line"));
    assert!(controller.stored_orders().is_empty());
}

#[test]
fn layered_architecture_repository_stores_domain_entities_not_response_dtos() {
    let repository = InMemoryOrderRepository::default();
    let use_case = CreateOrder::new(repository);
    let mut controller = OrderController::new(use_case);

    controller.create_order(CreateOrderDto::new(
        "ORD-2",
        vec![OrderLineDto::new("monitor", 2)],
    ));

    let stored = controller.stored_orders();

    assert_eq!(stored[0].id(), "ORD-2");
    assert_eq!(stored[0].total_units(), 2);
    assert_eq!(stored[0].status(), OrderStatus::Accepted);
}
