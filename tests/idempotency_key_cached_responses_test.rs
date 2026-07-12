use design_patterns_rust::patterns::distributed_systems::idempotency_key::cached_responses::{
    ApiResponse, CachedResponseStore, IdempotencyKey, RequestPayload,
};

#[test]
fn returns_cached_response_for_equivalent_retry() {
    let mut store = CachedResponseStore::new();
    let key = IdempotencyKey::new("create-order-001");
    let payload = RequestPayload::new("customer-1", "sku-1", 2);

    let first = store.execute(key.clone(), payload.clone()).unwrap();
    let second = store.execute(key, payload).unwrap();

    assert_eq!(first, second);
    assert_eq!(first, ApiResponse::created("order-1"));
    assert_eq!(store.executions(), 1);
}

#[test]
fn caches_each_key_separately() {
    let mut store = CachedResponseStore::new();
    let payload = RequestPayload::new("customer-1", "sku-1", 2);

    let first = store
        .execute(IdempotencyKey::new("create-order-001"), payload.clone())
        .unwrap();
    let second = store
        .execute(IdempotencyKey::new("create-order-002"), payload)
        .unwrap();

    assert_eq!(first, ApiResponse::created("order-1"));
    assert_eq!(second, ApiResponse::created("order-2"));
    assert_eq!(store.cached_entries(), 2);
}

#[test]
fn cached_response_survives_many_retries() {
    let mut store = CachedResponseStore::new();
    let key = IdempotencyKey::new("create-order-001");
    let payload = RequestPayload::new("customer-1", "sku-1", 2);

    let first = store.execute(key.clone(), payload.clone()).unwrap();
    let second = store.execute(key.clone(), payload.clone()).unwrap();
    let third = store.execute(key, payload).unwrap();

    assert_eq!(first, second);
    assert_eq!(second, third);
    assert_eq!(store.cached_entries(), 1);
    assert_eq!(store.executions(), 1);
}
