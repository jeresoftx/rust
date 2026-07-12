use design_patterns_rust::patterns::rust_idiomatic::error_handling_result::checkout_domain::{
    CheckoutError, CheckoutRequest, InventoryItem, PaymentMethod, checkout,
};

#[test]
fn result_error_handling_completes_successful_checkout() {
    let request = CheckoutRequest::new("ORD-1")
        .with_item("keyboard", 2)
        .with_payment(PaymentMethod::Card { approved: true });
    let inventory = vec![InventoryItem::new("keyboard", 5, 4_500)];

    let receipt = checkout(&request, &inventory).expect("checkout should succeed");

    assert_eq!(receipt.order_id(), "ORD-1");
    assert_eq!(receipt.total_cents(), 9_000);
    assert_eq!(receipt.summary(), "ORD-1 charged $90.00");
}

#[test]
fn result_error_handling_rejects_empty_cart_with_domain_error() {
    let request =
        CheckoutRequest::new("ORD-2").with_payment(PaymentMethod::Card { approved: true });
    let error = checkout(&request, &[]).unwrap_err();

    assert_eq!(error, CheckoutError::EmptyCart);
    assert_eq!(error.message(), "cart has no items");
}

#[test]
fn result_error_handling_reports_insufficient_inventory() {
    let request = CheckoutRequest::new("ORD-3")
        .with_item("monitor", 3)
        .with_payment(PaymentMethod::Card { approved: true });
    let inventory = vec![InventoryItem::new("monitor", 1, 20_000)];

    let error = checkout(&request, &inventory).unwrap_err();

    assert_eq!(
        error,
        CheckoutError::InsufficientInventory {
            sku: "monitor".to_string(),
            requested: 3,
            available: 1
        }
    );
    assert_eq!(
        error.message(),
        "sku monitor requested 3 units but only 1 are available"
    );
}

#[test]
fn result_error_handling_reports_rejected_payment() {
    let request = CheckoutRequest::new("ORD-4")
        .with_item("mouse", 1)
        .with_payment(PaymentMethod::Card { approved: false });
    let inventory = vec![InventoryItem::new("mouse", 3, 2_500)];

    let error = checkout(&request, &inventory).unwrap_err();

    assert_eq!(error, CheckoutError::PaymentRejected);
    assert_eq!(error.message(), "payment was rejected");
}
