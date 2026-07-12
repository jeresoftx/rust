use design_patterns_rust::patterns::architecture::domain_driven_design_tactical::order_aggregate::{
    Money, Order, OrderError, OrderId, OrderStatus, Sku,
};

#[test]
fn ddd_order_aggregate_uses_value_objects_and_calculates_total() {
    let mut order = Order::new(OrderId::new("ORD-1001"));

    order
        .add_item(Sku::new("SKU-BOOK").unwrap(), 2, Money::usd(1_500))
        .unwrap();
    order
        .add_item(Sku::new("SKU-PEN").unwrap(), 3, Money::usd(250))
        .unwrap();

    assert_eq!(order.id().as_str(), "ORD-1001");
    assert_eq!(order.status(), OrderStatus::Draft);
    assert_eq!(order.total(), Money::usd(3_750));
    assert_eq!(order.line_count(), 2);
}

#[test]
fn ddd_order_aggregate_protects_order_invariants() {
    let mut order = Order::new(OrderId::new("ORD-1002"));

    let result = order.add_item(Sku::new("SKU-BOOK").unwrap(), 0, Money::usd(1_500));

    assert_eq!(result, Err(OrderError::QuantityMustBePositive));
    assert_eq!(order.total(), Money::usd(0));
}

#[test]
fn ddd_order_aggregate_confirms_only_non_empty_draft_orders() {
    let mut order = Order::new(OrderId::new("ORD-1003"));

    assert_eq!(order.confirm(), Err(OrderError::CannotConfirmEmptyOrder));

    order
        .add_item(Sku::new("SKU-BOOK").unwrap(), 1, Money::usd(1_500))
        .unwrap();
    order.confirm().unwrap();

    assert_eq!(order.status(), OrderStatus::Confirmed);
    assert_eq!(
        order.add_item(Sku::new("SKU-PEN").unwrap(), 1, Money::usd(250)),
        Err(OrderError::CannotChangeConfirmedOrder)
    );
}

#[test]
fn ddd_value_objects_reject_invalid_skus() {
    assert_eq!(Sku::new(""), Err(OrderError::InvalidSku));
    assert_eq!(Sku::new("  "), Err(OrderError::InvalidSku));
}
