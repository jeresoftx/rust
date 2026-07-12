use design_patterns_rust::patterns::distributed_systems::rate_limiting::api_key_limit::{
    ApiKey, ApiKeyRateLimiter, RateLimitDecision,
};

#[test]
fn limits_each_api_key_independently() {
    let mut limiter = ApiKeyRateLimiter::new(2, 1);
    let app_a = ApiKey::new("app-a");
    let app_b = ApiKey::new("app-b");

    assert_eq!(
        limiter.allow(&app_a, 0),
        RateLimitDecision::Allowed { remaining: 1 }
    );
    assert_eq!(
        limiter.allow(&app_a, 0),
        RateLimitDecision::Allowed { remaining: 0 }
    );
    assert_eq!(
        limiter.allow(&app_a, 0),
        RateLimitDecision::Rejected {
            retry_after_ticks: 1
        }
    );

    assert_eq!(
        limiter.allow(&app_b, 0),
        RateLimitDecision::Allowed { remaining: 1 }
    );
}

#[test]
fn refills_only_the_key_being_checked() {
    let mut limiter = ApiKeyRateLimiter::new(2, 1);
    let app_a = ApiKey::new("app-a");
    let app_b = ApiKey::new("app-b");

    let _ = limiter.allow(&app_a, 0);
    let _ = limiter.allow(&app_a, 0);
    let _ = limiter.allow(&app_b, 0);

    assert_eq!(
        limiter.allow(&app_a, 1),
        RateLimitDecision::Allowed { remaining: 0 }
    );
    assert_eq!(limiter.remaining(&app_a), 0);
    assert_eq!(limiter.remaining(&app_b), 1);
}

#[test]
fn tracks_known_keys_and_remaining_quota() {
    let mut limiter = ApiKeyRateLimiter::new(3, 1);
    let app_a = ApiKey::new("app-a");
    let app_b = ApiKey::new("app-b");

    let _ = limiter.allow(&app_a, 0);
    let _ = limiter.allow(&app_b, 0);
    let _ = limiter.allow(&app_b, 0);

    assert_eq!(limiter.known_keys(), 2);
    assert_eq!(limiter.remaining(&app_a), 2);
    assert_eq!(limiter.remaining(&app_b), 1);
}
