use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `ApiKey` usado por el ejemplo para expresar el dominio del patron.
pub struct ApiKey(String);

impl ApiKey {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `RateLimitDecision` dentro del ejemplo.
pub enum RateLimitDecision {
    /// Variante `Allowed` del estado o error del ejemplo.
    Allowed {
        /// Valor publico `remaining` asociado a la variante `Allowed`.
        remaining: usize,
    },
    /// Variante `Rejected` del estado o error del ejemplo.
    Rejected {
        /// Valor publico `retry_after_ticks` asociado a la variante `Rejected`.
        retry_after_ticks: u64,
    },
}

#[derive(Debug, Clone, Copy)]
struct BucketState {
    tokens: usize,
    last_refill_tick: u64,
}

#[derive(Debug)]
/// Tipo publico `ApiKeyRateLimiter` usado por el ejemplo para expresar el dominio del patron.
pub struct ApiKeyRateLimiter {
    capacity: usize,
    refill_per_tick: usize,
    buckets: HashMap<ApiKey, BucketState>,
}

impl ApiKeyRateLimiter {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(capacity: usize, refill_per_tick: usize) -> Self {
        Self {
            capacity,
            refill_per_tick,
            buckets: HashMap::new(),
        }
    }

    /// Modela la operacion `allow` dentro del ejemplo del patron.
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

    /// Modela la operacion `remaining` dentro del ejemplo del patron.
    pub fn remaining(&self, key: &ApiKey) -> usize {
        self.buckets
            .get(key)
            .map(|bucket| bucket.tokens)
            .unwrap_or(self.capacity)
    }

    /// Modela la operacion `known keys` dentro del ejemplo del patron.
    pub fn known_keys(&self) -> usize {
        self.buckets.len()
    }
}

/// Operacion `refill bucket` definida por la abstraccion del ejemplo.
fn refill_bucket(bucket: &mut BucketState, capacity: usize, refill_per_tick: usize, now_tick: u64) {
    if now_tick <= bucket.last_refill_tick {
        return;
    }

    let elapsed = now_tick - bucket.last_refill_tick;
    let new_tokens = elapsed as usize * refill_per_tick;
    bucket.tokens = (bucket.tokens + new_tokens).min(capacity);
    bucket.last_refill_tick = now_tick;
}
