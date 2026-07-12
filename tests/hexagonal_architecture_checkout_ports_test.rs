use design_patterns_rust::patterns::architecture::hexagonal_architecture::checkout_ports_adapters::{
    adapters::{InMemoryInventory, RecordingPaymentGateway},
    application::{Checkout, CheckoutRequest},
    domain::{CheckoutError, CheckoutReceipt, OrderLine},
};

#[test]
fn hexagonal_architecture_checkout_uses_ports_and_adapters() {
    let inventory = InMemoryInventory::with_stock(vec![("keyboard", 5, 4_500)]);
    let payments = RecordingPaymentGateway::approved();
    let mut checkout = Checkout::new(inventory, payments);

    let receipt = checkout
        .execute(CheckoutRequest::new(
            "ORD-1",
            "card-1",
            vec![OrderLine::new("keyboard", 2)],
        ))
        .expect("checkout should succeed");

    assert_eq!(receipt, CheckoutReceipt::new("ORD-1", 9_000));
    assert_eq!(checkout.remaining_stock("keyboard"), 3);
    assert_eq!(checkout.charged_amounts(), vec![9_000]);
}

#[test]
fn hexagonal_architecture_checkout_rejects_insufficient_inventory_before_payment() {
    let inventory = InMemoryInventory::with_stock(vec![("monitor", 1, 20_000)]);
    let payments = RecordingPaymentGateway::approved();
    let mut checkout = Checkout::new(inventory, payments);

    let error = checkout
        .execute(CheckoutRequest::new(
            "ORD-2",
            "card-2",
            vec![OrderLine::new("monitor", 2)],
        ))
        .unwrap_err();

    assert_eq!(
        error,
        CheckoutError::InsufficientInventory {
            sku: "monitor".to_string(),
            requested: 2,
            available: 1
        }
    );
    assert!(checkout.charged_amounts().is_empty());
}

#[test]
fn hexagonal_architecture_checkout_can_swap_payment_adapter_behavior() {
    let inventory = InMemoryInventory::with_stock(vec![("mouse", 3, 2_500)]);
    let payments = RecordingPaymentGateway::rejected();
    let mut checkout = Checkout::new(inventory, payments);

    let error = checkout
        .execute(CheckoutRequest::new(
            "ORD-3",
            "card-3",
            vec![OrderLine::new("mouse", 1)],
        ))
        .unwrap_err();

    assert_eq!(error, CheckoutError::PaymentRejected);
    assert_eq!(checkout.remaining_stock("mouse"), 3);
    assert_eq!(checkout.charged_amounts(), vec![2_500]);
}
