#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitResponse {
    Allowed {
        remaining: usize,
    },
    Limited {
        retry_after_ticks: u64,
        remaining: usize,
    },
}

#[derive(Debug)]
pub struct RetryAfterLimiter {
    capacity: usize,
    refill_interval_ticks: u64,
    tokens: usize,
    last_refill_tick: u64,
}

impl RetryAfterLimiter {
    pub fn new(capacity: usize, refill_interval_ticks: u64) -> Self {
        Self {
            capacity,
            refill_interval_ticks,
            tokens: capacity,
            last_refill_tick: 0,
        }
    }

    pub fn check_at(&mut self, now_tick: u64) -> RateLimitResponse {
        self.refill_at(now_tick);

        if self.tokens == 0 {
            return RateLimitResponse::Limited {
                retry_after_ticks: self.retry_after_ticks(now_tick),
                remaining: 0,
            };
        }

        self.tokens -= 1;
        RateLimitResponse::Allowed {
            remaining: self.tokens,
        }
    }

    fn refill_at(&mut self, now_tick: u64) {
        if now_tick < self.last_refill_tick + self.refill_interval_ticks {
            return;
        }

        let elapsed_intervals = (now_tick - self.last_refill_tick) / self.refill_interval_ticks;
        self.tokens = (self.tokens + elapsed_intervals as usize).min(self.capacity);
        self.last_refill_tick += elapsed_intervals * self.refill_interval_ticks;
    }

    fn retry_after_ticks(&self, now_tick: u64) -> u64 {
        let next_refill_tick = self.last_refill_tick + self.refill_interval_ticks;
        next_refill_tick.saturating_sub(now_tick)
    }
}
