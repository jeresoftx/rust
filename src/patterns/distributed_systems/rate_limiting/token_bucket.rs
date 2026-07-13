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

#[derive(Debug)]
/// Tipo publico `TokenBucket` usado por el ejemplo para expresar el dominio del patron.
pub struct TokenBucket {
    capacity: usize,
    refill_per_tick: usize,
    tokens: usize,
    last_refill_tick: u64,
}

impl TokenBucket {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(capacity: usize, refill_per_tick: usize) -> Self {
        Self {
            capacity,
            refill_per_tick,
            tokens: capacity,
            last_refill_tick: 0,
        }
    }

    /// Modela la operacion `tokens` dentro del ejemplo del patron.
    pub fn tokens(&self) -> usize {
        self.tokens
    }

    /// Modela la operacion `refill at` dentro del ejemplo del patron.
    pub fn refill_at(&mut self, now_tick: u64) {
        if now_tick <= self.last_refill_tick {
            return;
        }

        let elapsed = now_tick - self.last_refill_tick;
        let new_tokens = elapsed as usize * self.refill_per_tick;
        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill_tick = now_tick;
    }

    /// Modela la operacion `allow at` dentro del ejemplo del patron.
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
