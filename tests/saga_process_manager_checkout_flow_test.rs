use design_patterns_rust::patterns::distributed_systems::saga_process_manager::checkout_flow::{
    CheckoutSaga, SagaLog, SagaState,
};

#[test]
fn coordinates_reservation_payment_and_shipping() {
    let mut saga = CheckoutSaga::new("order-1");

    let state = saga.run();

    assert_eq!(state, SagaState::Completed);
    assert_eq!(
        saga.log(),
        &[
            SagaLog::InventoryReserved,
            SagaLog::PaymentCaptured,
            SagaLog::ShipmentScheduled,
        ]
    );
}

#[test]
fn completed_saga_does_not_repeat_steps() {
    let mut saga = CheckoutSaga::new("order-1");

    saga.run();
    saga.run();

    assert_eq!(saga.log().len(), 3);
    assert_eq!(saga.state(), SagaState::Completed);
}

#[test]
fn exposes_order_id_for_process_tracking() {
    let saga = CheckoutSaga::new("order-42");

    assert_eq!(saga.order_id(), "order-42");
    assert_eq!(saga.state(), SagaState::Started);
}
