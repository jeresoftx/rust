#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `RateLimitResponse` dentro del ejemplo.
pub enum RateLimitResponse {
    /// Variante `Allowed` del estado o error del ejemplo.
    Allowed {
        /// Variante `remaining` del estado o error del ejemplo.
        remaining: usize,
    },
    /// Variante `Limited` del estado o error del ejemplo.
    Limited {
        /// Valor publico `retry_after_ticks` asociado a la variante del enum.
        retry_after_ticks: u64,
        /// Valor publico `remaining` asociado a la variante del enum.
        remaining: usize,
    },
}

#[derive(Debug)]
/// Tipo publico `RetryAfterLimiter` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryAfterLimiter {
    capacity: usize,
    refill_interval_ticks: u64,
    tokens: usize,
    last_refill_tick: u64,
}

impl RetryAfterLimiter {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(capacity: usize, refill_interval_ticks: u64) -> Self {
        Self {
            capacity,
            refill_interval_ticks,
            tokens: capacity,
            last_refill_tick: 0,
        }
    }

    /// Modela la operacion `check at` dentro del ejemplo del patron.
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

    /// Operacion `refill at` definida por la abstraccion del ejemplo.
    fn refill_at(&mut self, now_tick: u64) {
        if now_tick < self.last_refill_tick + self.refill_interval_ticks {
            return;
        }

        let elapsed_intervals = (now_tick - self.last_refill_tick) / self.refill_interval_ticks;
        self.tokens = (self.tokens + elapsed_intervals as usize).min(self.capacity);
        self.last_refill_tick += elapsed_intervals * self.refill_interval_ticks;
    }

    /// Operacion `retry after ticks` definida por la abstraccion del ejemplo.
    fn retry_after_ticks(&self, now_tick: u64) -> u64 {
        let next_refill_tick = self.last_refill_tick + self.refill_interval_ticks;
        next_refill_tick.saturating_sub(now_tick)
    }
}
