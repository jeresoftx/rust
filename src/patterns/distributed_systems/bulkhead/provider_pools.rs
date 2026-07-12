#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Payments,
    Shipping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkheadError {
    PoolFull(Provider),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permit {
    provider: Provider,
}

#[derive(Debug)]
pub struct BulkheadPools {
    payments_capacity: usize,
    shipping_capacity: usize,
    active_payments: usize,
    active_shipping: usize,
}

impl BulkheadPools {
    pub fn new(payments_capacity: usize, shipping_capacity: usize) -> Self {
        Self {
            payments_capacity,
            shipping_capacity,
            active_payments: 0,
            active_shipping: 0,
        }
    }

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

    pub fn capacity_for(&self, provider: Provider) -> usize {
        match provider {
            Provider::Payments => self.payments_capacity,
            Provider::Shipping => self.shipping_capacity,
        }
    }

    pub fn active_for(&self, provider: Provider) -> usize {
        match provider {
            Provider::Payments => self.active_payments,
            Provider::Shipping => self.active_shipping,
        }
    }

    pub fn remaining_for(&self, provider: Provider) -> usize {
        self.capacity_for(provider) - self.active_for(provider)
    }
}
