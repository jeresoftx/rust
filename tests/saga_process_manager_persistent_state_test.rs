use design_patterns_rust::patterns::distributed_systems::saga_process_manager::persistent_state::{
    PersistedSagaState, SagaRepository, StatefulSaga,
};

#[test]
fn persists_state_after_each_step() {
    let mut repository = SagaRepository::new();
    let mut saga = StatefulSaga::new("order-1");

    saga.advance(&mut repository);

    assert_eq!(
        repository.load("order-1"),
        Some(PersistedSagaState::InventoryReserved)
    );
}

#[test]
fn resumes_from_persisted_state_and_completes() {
    let mut repository = SagaRepository::new();
    repository.save("order-1", PersistedSagaState::InventoryReserved);

    let mut saga = StatefulSaga::resume("order-1", &repository).unwrap();
    saga.advance(&mut repository);
    saga.advance(&mut repository);

    assert_eq!(saga.state(), PersistedSagaState::Completed);
    assert_eq!(
        repository.load("order-1"),
        Some(PersistedSagaState::Completed)
    );
}

#[test]
fn missing_persisted_state_returns_none() {
    let repository = SagaRepository::new();

    assert!(StatefulSaga::resume("missing-order", &repository).is_none());
}
