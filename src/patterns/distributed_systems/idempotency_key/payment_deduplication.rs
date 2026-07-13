use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `IdempotencyKey` usado por el ejemplo para expresar el dominio del patron.
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `PaymentRequest` usado por el ejemplo para expresar el dominio del patron.
pub struct PaymentRequest {
    customer_id: String,
    amount_cents: u64,
}

impl PaymentRequest {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(customer_id: impl Into<String>, amount_cents: u64) -> Self {
        Self {
            customer_id: customer_id.into(),
            amount_cents,
        }
    }

    /// Operacion `fingerprint` definida por la abstraccion del ejemplo.
    fn fingerprint(&self) -> String {
        format!("{}:{}", self.customer_id, self.amount_cents)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ChargeResult` usado por el ejemplo para expresar el dominio del patron.
pub struct ChargeResult {
    charge_id: String,
    amount_cents: u64,
}

impl ChargeResult {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(charge_id: impl Into<String>, amount_cents: u64) -> Self {
        Self {
            charge_id: charge_id.into(),
            amount_cents,
        }
    }
}

#[derive(Debug, Clone)]
struct StoredPayment {
    fingerprint: String,
    result: ChargeResult,
}

#[derive(Debug, Default)]
/// Tipo publico `PaymentProcessor` usado por el ejemplo para expresar el dominio del patron.
pub struct PaymentProcessor {
    stored: HashMap<IdempotencyKey, StoredPayment>,
    executed_charges: usize,
}

impl PaymentProcessor {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `charge` dentro del ejemplo del patron.
    pub fn charge(
        &mut self,
        key: IdempotencyKey,
        request: PaymentRequest,
    ) -> Result<ChargeResult, String> {
        let fingerprint = request.fingerprint();
        if let Some(stored) = self.stored.get(&key) {
            if stored.fingerprint == fingerprint {
                return Ok(stored.result.clone());
            }

            return Err("idempotency key reused with a different payment".to_string());
        }

        self.executed_charges += 1;
        let result = ChargeResult::new(
            format!("ch_{}", self.executed_charges),
            request.amount_cents,
        );
        self.stored.insert(
            key,
            StoredPayment {
                fingerprint,
                result: result.clone(),
            },
        );
        Ok(result)
    }

    /// Modela la operacion `executed charges` dentro del ejemplo del patron.
    pub fn executed_charges(&self) -> usize {
        self.executed_charges
    }
}
