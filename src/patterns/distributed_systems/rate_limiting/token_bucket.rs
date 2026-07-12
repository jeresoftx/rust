#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitDecision {
    Allowed { remaining: usize },
    Rejected { retry_after_ticks: u64 },
}

#[derive(Debug)]
pub struct TokenBucket {
    capacity: usize,
    refill_per_tick: usize,
    tokens: usize,
    last_refill_tick: u64,
}

impl TokenBucket {
    pub fn new(capacity: usize, refill_per_tick: usize) -> Self {
        Self {
            capacity,
            refill_per_tick,
            tokens: capacity,
            last_refill_tick: 0,
        }
    }

    pub fn tokens(&self) -> usize {
        self.tokens
    }

    pub fn refill_at(&mut self, now_tick: u64) {
        if now_tick <= self.last_refill_tick {
            return;
        }

        let elapsed = now_tick - self.last_refill_tick;
        let new_tokens = elapsed as usize * self.refill_per_tick;
        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill_tick = now_tick;
    }

    pub fn allow_at(&mut self, now_tick: u64) -> RateLimitDecision {
        self.refill_at(now_tick);

        if self.tokens == 0 {
            return RateLimitDecision::Rejected {
                retry_after_ticks: 1,
            };
        }

        self.tokens -= 1;
        RateLimitDecision::Allowed {
            remaining: self.tokens,
        }
    }
}
