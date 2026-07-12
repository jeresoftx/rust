use design_patterns_rust::patterns::distributed_systems::bulkhead::provider_pools::{
    BulkheadError, BulkheadPools, Provider,
};

#[test]
fn saturated_provider_pool_does_not_block_another_provider() {
    let mut pools = BulkheadPools::new(1, 2);

    let payments_permit = pools.acquire(Provider::Payments).unwrap();
    assert_eq!(
        pools.acquire(Provider::Payments),
        Err(BulkheadError::PoolFull(Provider::Payments))
    );

    let shipping_a = pools.acquire(Provider::Shipping).unwrap();
    let shipping_b = pools.acquire(Provider::Shipping).unwrap();

    assert_eq!(pools.active_for(Provider::Payments), 1);
    assert_eq!(pools.active_for(Provider::Shipping), 2);

    pools.release(shipping_a);
    pools.release(shipping_b);
    pools.release(payments_permit);
}

#[test]
fn releasing_a_provider_slot_only_frees_that_provider() {
    let mut pools = BulkheadPools::new(1, 1);

    let payments_permit = pools.acquire(Provider::Payments).unwrap();
    let _shipping_permit = pools.acquire(Provider::Shipping).unwrap();

    assert_eq!(
        pools.acquire(Provider::Payments),
        Err(BulkheadError::PoolFull(Provider::Payments))
    );
    assert_eq!(
        pools.acquire(Provider::Shipping),
        Err(BulkheadError::PoolFull(Provider::Shipping))
    );

    pools.release(payments_permit);

    assert!(pools.acquire(Provider::Payments).is_ok());
    assert_eq!(
        pools.acquire(Provider::Shipping),
        Err(BulkheadError::PoolFull(Provider::Shipping))
    );
}

#[test]
fn reports_capacity_per_provider() {
    let mut pools = BulkheadPools::new(2, 3);

    let _payments = pools.acquire(Provider::Payments).unwrap();
    let _shipping = pools.acquire(Provider::Shipping).unwrap();

    assert_eq!(pools.capacity_for(Provider::Payments), 2);
    assert_eq!(pools.capacity_for(Provider::Shipping), 3);
    assert_eq!(pools.remaining_for(Provider::Payments), 1);
    assert_eq!(pools.remaining_for(Provider::Shipping), 2);
}
