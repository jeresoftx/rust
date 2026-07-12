use design_patterns_rust::patterns::distributed_systems::saga_process_manager::compensation::{
    CompensatingSaga, SagaLog, SagaState,
};

#[test]
fn payment_failure_releases_reserved_inventory() {
    let mut saga = CompensatingSaga::payment_will_fail("order-1");

    let state = saga.run();

    assert_eq!(state, SagaState::Compensated);
    assert_eq!(
        saga.log(),
        &[
            SagaLog::InventoryReserved,
            SagaLog::PaymentFailed,
            SagaLog::InventoryReleased,
        ]
    );
}

#[test]
fn compensation_keeps_failure_reason() {
    let mut saga = CompensatingSaga::payment_will_fail("order-1");

    saga.run();

    assert_eq!(saga.failure_reason(), Some("payment declined"));
}

#[test]
fn successful_payment_does_not_compensate() {
    let mut saga = CompensatingSaga::payment_will_succeed("order-1");

    let state = saga.run();

    assert_eq!(state, SagaState::Completed);
    assert!(!saga.log().contains(&SagaLog::InventoryReleased));
}
