use design_patterns_rust::patterns::distributed_systems::bulkhead::critical_isolation::{
    BulkheadError, OperationClass, WorkflowBulkhead,
};

#[test]
fn saturated_non_critical_work_does_not_block_critical_work() {
    let mut bulkhead = WorkflowBulkhead::new(1, 2);

    let _report_a = bulkhead.acquire(OperationClass::NonCritical).unwrap();
    let _report_b = bulkhead.acquire(OperationClass::NonCritical).unwrap();

    assert_eq!(
        bulkhead.acquire(OperationClass::NonCritical),
        Err(BulkheadError::ClassFull(OperationClass::NonCritical))
    );

    let critical = bulkhead.acquire(OperationClass::Critical).unwrap();

    assert_eq!(bulkhead.active(OperationClass::NonCritical), 2);
    assert_eq!(bulkhead.active(OperationClass::Critical), 1);

    bulkhead.release(critical);
}

#[test]
fn failures_are_tracked_per_operation_class() {
    let mut bulkhead = WorkflowBulkhead::new(1, 1);

    bulkhead.record_failure(OperationClass::NonCritical);
    bulkhead.record_failure(OperationClass::NonCritical);

    assert_eq!(bulkhead.failures(OperationClass::NonCritical), 2);
    assert_eq!(bulkhead.failures(OperationClass::Critical), 0);
    assert!(bulkhead.acquire(OperationClass::Critical).is_ok());
}

#[test]
fn release_returns_capacity_to_matching_operation_class() {
    let mut bulkhead = WorkflowBulkhead::new(1, 1);

    let critical = bulkhead.acquire(OperationClass::Critical).unwrap();
    let non_critical = bulkhead.acquire(OperationClass::NonCritical).unwrap();

    assert_eq!(
        bulkhead.acquire(OperationClass::Critical),
        Err(BulkheadError::ClassFull(OperationClass::Critical))
    );
    assert_eq!(
        bulkhead.acquire(OperationClass::NonCritical),
        Err(BulkheadError::ClassFull(OperationClass::NonCritical))
    );

    bulkhead.release(non_critical);

    assert!(bulkhead.acquire(OperationClass::NonCritical).is_ok());
    assert_eq!(
        bulkhead.acquire(OperationClass::Critical),
        Err(BulkheadError::ClassFull(OperationClass::Critical))
    );

    bulkhead.release(critical);
}
