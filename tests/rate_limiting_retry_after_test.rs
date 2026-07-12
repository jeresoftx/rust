use design_patterns_rust::patterns::distributed_systems::rate_limiting::retry_after_response::{
    RateLimitResponse, RetryAfterLimiter,
};

#[test]
fn rejected_response_includes_retry_after_ticks() {
    let mut limiter = RetryAfterLimiter::new(2, 5);

    let _ = limiter.check_at(0);
    let _ = limiter.check_at(0);

    assert_eq!(
        limiter.check_at(0),
        RateLimitResponse::Limited {
            retry_after_ticks: 5,
            remaining: 0,
        }
    );
}

#[test]
fn retry_after_decreases_as_refill_tick_approaches() {
    let mut limiter = RetryAfterLimiter::new(1, 10);

    let _ = limiter.check_at(0);

    assert_eq!(
        limiter.check_at(3),
        RateLimitResponse::Limited {
            retry_after_ticks: 7,
            remaining: 0,
        }
    );
    assert_eq!(
        limiter.check_at(9),
        RateLimitResponse::Limited {
            retry_after_ticks: 1,
            remaining: 0,
        }
    );
}

#[test]
fn accepts_again_when_retry_after_has_elapsed() {
    let mut limiter = RetryAfterLimiter::new(1, 10);

    let _ = limiter.check_at(0);
    assert_eq!(
        limiter.check_at(10),
        RateLimitResponse::Allowed { remaining: 0 }
    );
}
