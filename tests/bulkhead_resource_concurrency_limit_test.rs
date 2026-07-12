use design_patterns_rust::patterns::distributed_systems::bulkhead::resource_concurrency_limit::{
    BulkheadError, Resource, ResourceBulkhead,
};

#[test]
fn rejects_work_when_resource_limit_is_reached() {
    let mut bulkhead = ResourceBulkhead::new(2, 1);

    let _db_a = bulkhead.acquire(Resource::Database).unwrap();
    let _db_b = bulkhead.acquire(Resource::Database).unwrap();

    assert_eq!(
        bulkhead.acquire(Resource::Database),
        Err(BulkheadError::ResourceFull(Resource::Database))
    );
    assert_eq!(bulkhead.active(Resource::Database), 2);
    assert_eq!(bulkhead.rejected(Resource::Database), 1);
}

#[test]
fn release_returns_capacity_to_the_same_resource() {
    let mut bulkhead = ResourceBulkhead::new(1, 1);

    let db_permit = bulkhead.acquire(Resource::Database).unwrap();
    assert_eq!(
        bulkhead.acquire(Resource::Database),
        Err(BulkheadError::ResourceFull(Resource::Database))
    );

    bulkhead.release(db_permit);

    assert!(bulkhead.acquire(Resource::Database).is_ok());
    assert_eq!(bulkhead.active(Resource::Database), 1);
}

#[test]
fn independent_resources_keep_their_own_limits() {
    let mut bulkhead = ResourceBulkhead::new(1, 2);

    let _db = bulkhead.acquire(Resource::Database).unwrap();
    let _search_a = bulkhead.acquire(Resource::SearchIndex).unwrap();
    let _search_b = bulkhead.acquire(Resource::SearchIndex).unwrap();

    assert_eq!(
        bulkhead.acquire(Resource::Database),
        Err(BulkheadError::ResourceFull(Resource::Database))
    );
    assert_eq!(
        bulkhead.acquire(Resource::SearchIndex),
        Err(BulkheadError::ResourceFull(Resource::SearchIndex))
    );

    assert_eq!(bulkhead.active(Resource::Database), 1);
    assert_eq!(bulkhead.active(Resource::SearchIndex), 2);
    assert_eq!(bulkhead.rejected(Resource::Database), 1);
    assert_eq!(bulkhead.rejected(Resource::SearchIndex), 1);
}
