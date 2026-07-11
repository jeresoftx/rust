use design_patterns_rust::patterns::gof::behavioral::mediator::checkout_orchestrator::{
    CheckoutMediator, CheckoutRequest,
};

#[test]
fn mediator_orchestrates_checkout_modules_in_order() {
    let mut mediator = CheckoutMediator::new();
    mediator.stock_item("book", 2);

    let result = mediator
        .checkout(CheckoutRequest::new("order-100", "book", 1, 4500))
        .expect("checkout should complete");

    assert_eq!(result.order_id, "order-100");
    assert_eq!(result.tracking_code, "SHIP-order-100");
    assert_eq!(
        mediator.events(),
        vec![
            "inventory:reserved:book:1".to_string(),
            "payment:charged:4500".to_string(),
            "shipping:created:SHIP-order-100".to_string(),
            "notification:sent:order-100".to_string(),
        ]
    );
    assert_eq!(mediator.stock_for("book"), 1);
}

#[test]
fn mediator_stops_checkout_when_inventory_is_missing() {
    let mut mediator = CheckoutMediator::new();
    mediator.stock_item("book", 0);

    let error = mediator
        .checkout(CheckoutRequest::new("order-101", "book", 1, 4500))
        .unwrap_err();

    assert_eq!(error, "inventory is not available");
    assert_eq!(
        mediator.events(),
        vec!["inventory:rejected:book".to_string()]
    );
}

#[test]
fn mediator_releases_inventory_when_payment_fails() {
    let mut mediator = CheckoutMediator::new();
    mediator.stock_item("book", 1);

    let error = mediator
        .checkout(CheckoutRequest::new("order-102", "book", 1, 0))
        .unwrap_err();

    assert_eq!(error, "payment amount must be greater than zero");
    assert_eq!(mediator.stock_for("book"), 1);
    assert_eq!(
        mediator.events(),
        vec![
            "inventory:reserved:book:1".to_string(),
            "payment:rejected".to_string(),
            "inventory:released:book:1".to_string(),
        ]
    );
}
