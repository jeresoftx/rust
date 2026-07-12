use design_patterns_rust::patterns::distributed_systems::idempotency_key::payload_conflict::{
    ConflictError, IdempotencyKey, IdempotencyStore, RequestPayload, StoredResponse,
};

#[test]
fn rejects_same_key_with_different_payload() {
    let mut store = IdempotencyStore::new();
    let key = IdempotencyKey::new("request-001");
    let original = RequestPayload::new("customer-1", 1000);
    let changed = RequestPayload::new("customer-1", 2000);

    let _ = store.record_or_replay(key.clone(), original).unwrap();

    assert_eq!(
        store.record_or_replay(key, changed),
        Err(ConflictError::PayloadMismatch)
    );
}

#[test]
fn same_key_and_same_payload_replays_original_response() {
    let mut store = IdempotencyStore::new();
    let key = IdempotencyKey::new("request-001");
    let payload = RequestPayload::new("customer-1", 1000);

    let first = store
        .record_or_replay(key.clone(), payload.clone())
        .unwrap();
    let second = store.record_or_replay(key, payload).unwrap();

    assert_eq!(first, second);
    assert_eq!(first, StoredResponse::new("accepted-1"));
    assert_eq!(store.executions(), 1);
}

#[test]
fn conflict_does_not_replace_original_response() {
    let mut store = IdempotencyStore::new();
    let key = IdempotencyKey::new("request-001");
    let original = RequestPayload::new("customer-1", 1000);
    let changed = RequestPayload::new("customer-1", 2000);

    let first = store
        .record_or_replay(key.clone(), original.clone())
        .unwrap();
    let conflict = store.record_or_replay(key.clone(), changed);
    let replay = store.record_or_replay(key, original).unwrap();

    assert_eq!(conflict, Err(ConflictError::PayloadMismatch));
    assert_eq!(replay, first);
    assert_eq!(store.executions(), 1);
}
