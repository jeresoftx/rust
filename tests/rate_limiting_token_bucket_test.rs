use design_patterns_rust::patterns::distributed_systems::rate_limiting::token_bucket::{
    RateLimitDecision, TokenBucket,
};

#[test]
fn allows_requests_while_tokens_are_available() {
    let mut bucket = TokenBucket::new(3, 1);

    assert_eq!(
        bucket.allow_at(0),
        RateLimitDecision::Allowed { remaining: 2 }
    );
    assert_eq!(
        bucket.allow_at(0),
        RateLimitDecision::Allowed { remaining: 1 }
    );
    assert_eq!(
        bucket.allow_at(0),
        RateLimitDecision::Allowed { remaining: 0 }
    );
}

#[test]
fn rejects_when_bucket_is_empty() {
    let mut bucket = TokenBucket::new(2, 1);

    let _ = bucket.allow_at(0);
    let _ = bucket.allow_at(0);

    assert_eq!(
        bucket.allow_at(0),
        RateLimitDecision::Rejected {
            retry_after_ticks: 1
        }
    );
    assert_eq!(bucket.tokens(), 0);
}

#[test]
fn refills_tokens_using_logical_ticks_without_exceeding_capacity() {
    let mut bucket = TokenBucket::new(3, 1);

    let _ = bucket.allow_at(0);
    let _ = bucket.allow_at(0);
    let _ = bucket.allow_at(0);

    assert_eq!(
        bucket.allow_at(2),
        RateLimitDecision::Allowed { remaining: 1 }
    );
    assert_eq!(bucket.tokens(), 1);

    bucket.refill_at(10);
    assert_eq!(bucket.tokens(), 3);
}
