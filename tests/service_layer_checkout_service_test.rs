use design_patterns_rust::patterns::architecture::service_layer::checkout_service::{
    CheckoutError, CheckoutRequest, CheckoutService, DiscountPolicy, InMemoryInventoryRepository,
    InMemoryOrderRepository, Order, PaymentGateway,
};

#[test]
fn service_layer_checkout_coordinates_inventory_discount_payment_and_order_repository() {
    let mut inventory = InMemoryInventoryRepository::default();
    inventory.seed("SKU-BOOK", 5, 2_000);
    let orders = InMemoryOrderRepository::default();
    let payment = PaymentGateway::default();
    let policy = DiscountPolicy::percent_over_threshold(10, 5_000);
    let mut service = CheckoutService::new(inventory, orders, payment, policy);

    let receipt = service
        .checkout(CheckoutRequest::new(
            "ORD-1",
            vec![("SKU-BOOK".to_string(), 3)],
            "tok_ok",
        ))
        .expect("checkout should succeed");

    assert_eq!(receipt.total_cents(), 5_400);
    assert_eq!(service.stock("SKU-BOOK"), 2);
    assert_eq!(
        service.find_order("ORD-1"),
        Some(Order::new("ORD-1", 6_000, 600, 5_400))
    );
    assert_eq!(service.payment_log(), vec!["charged:tok_ok:5400"]);
}

#[test]
fn service_layer_checkout_rejects_insufficient_stock_without_payment_or_order() {
    let mut inventory = InMemoryInventoryRepository::default();
    inventory.seed("SKU-BOOK", 1, 2_000);
    let orders = InMemoryOrderRepository::default();
    let payment = PaymentGateway::default();
    let policy = DiscountPolicy::percent_over_threshold(10, 5_000);
    let mut service = CheckoutService::new(inventory, orders, payment, policy);

    let result = service.checkout(CheckoutRequest::new(
        "ORD-1",
        vec![("SKU-BOOK".to_string(), 3)],
        "tok_ok",
    ));

    assert_eq!(result, Err(CheckoutError::InsufficientStock));
    assert_eq!(service.stock("SKU-BOOK"), 1);
    assert_eq!(service.find_order("ORD-1"), None);
    assert!(service.payment_log().is_empty());
}

#[test]
fn service_layer_checkout_rejects_failed_payment_without_saving_order_or_reserving_stock() {
    let mut inventory = InMemoryInventoryRepository::default();
    inventory.seed("SKU-BOOK", 5, 2_000);
    let orders = InMemoryOrderRepository::default();
    let payment = PaymentGateway::default();
    let policy = DiscountPolicy::percent_over_threshold(10, 5_000);
    let mut service = CheckoutService::new(inventory, orders, payment, policy);

    let result = service.checkout(CheckoutRequest::new(
        "ORD-1",
        vec![("SKU-BOOK".to_string(), 3)],
        "tok_fail",
    ));

    assert_eq!(result, Err(CheckoutError::PaymentDeclined));
    assert_eq!(service.stock("SKU-BOOK"), 5);
    assert_eq!(service.find_order("ORD-1"), None);
    assert_eq!(service.payment_log(), vec!["declined:tok_fail:5400"]);
}
