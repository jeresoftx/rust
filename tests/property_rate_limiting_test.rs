use design_patterns_rust::patterns::distributed_systems::rate_limiting::api_key_limit::{
    ApiKey, ApiKeyRateLimiter,
};
use design_patterns_rust::patterns::distributed_systems::rate_limiting::token_bucket::{
    RateLimitDecision, TokenBucket,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn token_bucket_never_refills_above_capacity(
        capacity in 1_usize..100,
        refill_per_tick in 0_usize..50,
        ticks in prop::collection::vec(0_u64..1_000, 1..50),
    ) {
        let mut bucket = TokenBucket::new(capacity, refill_per_tick);

        for tick in ticks {
            bucket.refill_at(tick);
            prop_assert!(bucket.tokens() <= capacity);
        }
    }

    #[test]
    fn allowed_token_bucket_request_reports_remaining_below_capacity(
        capacity in 1_usize..100,
        refill_per_tick in 0_usize..50,
        tick in 0_u64..1_000,
    ) {
        let mut bucket = TokenBucket::new(capacity, refill_per_tick);

        if let RateLimitDecision::Allowed { remaining } = bucket.allow_at(tick) {
            prop_assert!(remaining < capacity);
        }
    }

    #[test]
    fn api_key_limiter_keeps_keys_independent(capacity in 1_usize..20, requests in 0_usize..20) {
        let alpha = ApiKey::new("alpha");
        let beta = ApiKey::new("beta");
        let mut limiter = ApiKeyRateLimiter::new(capacity, 0);

        for _ in 0..requests {
            limiter.allow(&alpha, 0);
        }

        prop_assert_eq!(limiter.remaining(&beta), capacity);
        prop_assert!(limiter.known_keys() <= 1);
    }
}
