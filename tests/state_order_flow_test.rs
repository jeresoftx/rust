use design_patterns_rust::patterns::gof::behavioral::state::order_flow::{Order, OrderState};

#[test]
fn state_moves_order_through_payment_and_shipping() {
    let mut order = Order::new("order-100");

    assert_eq!(order.state(), OrderState::Pending);

    order.pay().expect("pending order can be paid");
    assert_eq!(order.state(), OrderState::Paid);

    order.ship().expect("paid order can be shipped");
    assert_eq!(order.state(), OrderState::Shipped);
    assert_eq!(
        order.history(),
        vec![
            "created:order-100".to_string(),
            "paid:order-100".to_string(),
            "shipped:order-100".to_string(),
        ]
    );
}

#[test]
fn state_rejects_invalid_order_transitions() {
    let mut order = Order::new("order-101");

    assert_eq!(order.ship().unwrap_err(), "only paid orders can be shipped");

    order.cancel().expect("pending order can be cancelled");
    assert_eq!(order.state(), OrderState::Cancelled);
    assert_eq!(order.pay().unwrap_err(), "cancelled orders cannot be paid");
}
