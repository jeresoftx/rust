use design_patterns_rust::patterns::rust_idiomatic::typestate::paid_order::Order;

#[test]
fn typestate_ships_order_only_after_payment() {
    let receipt = Order::new("ORD-1")
        .add_item("Keyboard")
        .add_item("Mouse")
        .pay("pay_123")
        .ship("TRACK-9");

    assert_eq!(
        receipt.summary(),
        "ORD-1 paid with pay_123 shipped as TRACK-9"
    );
    assert_eq!(receipt.items(), ["Keyboard", "Mouse"]);
}

#[test]
fn typestate_cancels_order_before_payment() {
    let cancellation = Order::new("ORD-2")
        .add_item("Monitor")
        .cancel("customer request");

    assert_eq!(cancellation.summary(), "ORD-2 cancelled: customer request");
    assert_eq!(cancellation.items(), ["Monitor"]);
}

#[test]
fn typestate_keeps_items_across_payment_transition() {
    let paid_order = Order::new("ORD-3")
        .add_item("Desk")
        .add_item("Chair")
        .pay("pay_999");

    assert_eq!(paid_order.id(), "ORD-3");
    assert_eq!(paid_order.payment_id(), "pay_999");
    assert_eq!(paid_order.items(), ["Desk", "Chair"]);
}
