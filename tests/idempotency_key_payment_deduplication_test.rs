use design_patterns_rust::patterns::distributed_systems::idempotency_key::payment_deduplication::{
    ChargeResult, IdempotencyKey, PaymentProcessor, PaymentRequest,
};

#[test]
fn repeated_payment_with_same_key_returns_original_charge() {
    let mut processor = PaymentProcessor::new();
    let key = IdempotencyKey::new("checkout-001");
    let request = PaymentRequest::new("customer-1", 5000);

    let first = processor.charge(key.clone(), request.clone()).unwrap();
    let second = processor.charge(key, request).unwrap();

    assert_eq!(first, second);
    assert_eq!(first, ChargeResult::new("ch_1", 5000));
    assert_eq!(processor.executed_charges(), 1);
}

#[test]
fn different_keys_create_different_charges() {
    let mut processor = PaymentProcessor::new();
    let request = PaymentRequest::new("customer-1", 5000);

    let first = processor
        .charge(IdempotencyKey::new("checkout-001"), request.clone())
        .unwrap();
    let second = processor
        .charge(IdempotencyKey::new("checkout-002"), request)
        .unwrap();

    assert_eq!(first, ChargeResult::new("ch_1", 5000));
    assert_eq!(second, ChargeResult::new("ch_2", 5000));
    assert_eq!(processor.executed_charges(), 2);
}

#[test]
fn same_key_with_same_payload_keeps_original_result_after_many_retries() {
    let mut processor = PaymentProcessor::new();
    let key = IdempotencyKey::new("checkout-001");
    let request = PaymentRequest::new("customer-1", 5000);

    let first = processor.charge(key.clone(), request.clone()).unwrap();
    let second = processor.charge(key.clone(), request.clone()).unwrap();
    let third = processor.charge(key, request).unwrap();

    assert_eq!(first, second);
    assert_eq!(second, third);
    assert_eq!(processor.executed_charges(), 1);
}
