#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `Provider` dentro del ejemplo.
pub enum Provider {
    /// Variante `Payments` del estado o error del ejemplo.
    Payments,
    /// Variante `Shipping` del estado o error del ejemplo.
    Shipping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `BulkheadError` dentro del ejemplo.
pub enum BulkheadError {
    /// Variante `PoolFull` del estado o error del ejemplo.
    PoolFull(Provider),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Permit` usado por el ejemplo para expresar el dominio del patron.
pub struct Permit {
    provider: Provider,
}

#[derive(Debug)]
/// Tipo publico `BulkheadPools` usado por el ejemplo para expresar el dominio del patron.
pub struct BulkheadPools {
    payments_capacity: usize,
    shipping_capacity: usize,
    active_payments: usize,
    active_shipping: usize,
}

impl BulkheadPools {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(payments_capacity: usize, shipping_capacity: usize) -> Self {
        Self {
            payments_capacity,
            shipping_capacity,
            active_payments: 0,
            active_shipping: 0,
        }
    }

    /// Modela la operacion `acquire` dentro del ejemplo del patron.
    pub fn acquire(&mut self, provider: Provider) -> Result<Permit, BulkheadError> {
        match provider {
            Provider::Payments if self.active_payments < self.payments_capacity => {
                self.active_payments += 1;
                Ok(Permit { provider })
            }
            Provider::Shipping if self.active_shipping < self.shipping_capacity => {
                self.active_shipping += 1;
                Ok(Permit { provider })
            }
            _ => Err(BulkheadError::PoolFull(provider)),
        }
    }

    /// Modela la operacion `release` dentro del ejemplo del patron.
    pub fn release(&mut self, permit: Permit) {
        match permit.provider {
            Provider::Payments => {
                self.active_payments = self.active_payments.saturating_sub(1);
            }
            Provider::Shipping => {
                self.active_shipping = self.active_shipping.saturating_sub(1);
            }
        }
    }

    /// Modela la operacion `capacity for` dentro del ejemplo del patron.
    pub fn capacity_for(&self, provider: Provider) -> usize {
        match provider {
            Provider::Payments => self.payments_capacity,
            Provider::Shipping => self.shipping_capacity,
        }
    }

    /// Modela la operacion `active for` dentro del ejemplo del patron.
    pub fn active_for(&self, provider: Provider) -> usize {
        match provider {
            Provider::Payments => self.active_payments,
            Provider::Shipping => self.active_shipping,
        }
    }

    /// Modela la operacion `remaining for` dentro del ejemplo del patron.
    pub fn remaining_for(&self, provider: Provider) -> usize {
        self.capacity_for(provider) - self.active_for(provider)
    }
}
