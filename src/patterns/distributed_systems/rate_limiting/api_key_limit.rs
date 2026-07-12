use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitDecision {
    Allowed { remaining: usize },
    Rejected { retry_after_ticks: u64 },
}

#[derive(Debug, Clone, Copy)]
struct BucketState {
    tokens: usize,
    last_refill_tick: u64,
}

#[derive(Debug)]
pub struct ApiKeyRateLimiter {
    capacity: usize,
    refill_per_tick: usize,
    buckets: HashMap<ApiKey, BucketState>,
}

impl ApiKeyRateLimiter {
    pub fn new(capacity: usize, refill_per_tick: usize) -> Self {
        Self {
            capacity,
            refill_per_tick,
            buckets: HashMap::new(),
        }
    }

    pub fn allow(&mut self, key: &ApiKey, now_tick: u64) -> RateLimitDecision {
        let capacity = self.capacity;
        let refill_per_tick = self.refill_per_tick;
        let bucket = self.buckets.entry(key.clone()).or_insert(BucketState {
            tokens: capacity,
            last_refill_tick: now_tick,
        });

        refill_bucket(bucket, capacity, refill_per_tick, now_tick);

        if bucket.tokens == 0 {
            return RateLimitDecision::Rejected {
                retry_after_ticks: 1,
            };
        }

        bucket.tokens -= 1;
        RateLimitDecision::Allowed {
            remaining: bucket.tokens,
        }
    }

    pub fn remaining(&self, key: &ApiKey) -> usize {
        self.buckets
            .get(key)
            .map(|bucket| bucket.tokens)
            .unwrap_or(self.capacity)
    }

    pub fn known_keys(&self) -> usize {
        self.buckets.len()
    }
}

fn refill_bucket(bucket: &mut BucketState, capacity: usize, refill_per_tick: usize, now_tick: u64) {
    if now_tick <= bucket.last_refill_tick {
        return;
    }

    let elapsed = now_tick - bucket.last_refill_tick;
    let new_tokens = elapsed as usize * refill_per_tick;
    bucket.tokens = (bucket.tokens + new_tokens).min(capacity);
    bucket.last_refill_tick = now_tick;
}
