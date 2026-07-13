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
/// Tipo publico `RequestPayload` usado por el ejemplo para expresar el dominio del patron.
pub struct RequestPayload {
    customer_id: String,
    amount_cents: u64,
}

impl RequestPayload {
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
/// Tipo publico `StoredResponse` usado por el ejemplo para expresar el dominio del patron.
pub struct StoredResponse {
    body: String,
}

impl StoredResponse {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ConflictError` dentro del ejemplo.
pub enum ConflictError {
    /// Variante `PayloadMismatch` del estado o error del ejemplo.
    PayloadMismatch,
}

#[derive(Debug, Clone)]
struct StoredEntry {
    fingerprint: String,
    response: StoredResponse,
}

#[derive(Debug, Default)]
/// Tipo publico `IdempotencyStore` usado por el ejemplo para expresar el dominio del patron.
pub struct IdempotencyStore {
    entries: HashMap<IdempotencyKey, StoredEntry>,
    executions: usize,
}

impl IdempotencyStore {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self::default()
    }

    /// Modela la operacion `record or replay` dentro del ejemplo del patron.
    pub fn record_or_replay(
        &mut self,
        key: IdempotencyKey,
        payload: RequestPayload,
    ) -> Result<StoredResponse, ConflictError> {
        let fingerprint = payload.fingerprint();
        if let Some(entry) = self.entries.get(&key) {
            if entry.fingerprint == fingerprint {
                return Ok(entry.response.clone());
            }

            return Err(ConflictError::PayloadMismatch);
        }

        self.executions += 1;
        let response = StoredResponse::new(format!("accepted-{}", self.executions));
        self.entries.insert(
            key,
            StoredEntry {
                fingerprint,
                response: response.clone(),
            },
        );
        Ok(response)
    }

    /// Modela la operacion `executions` dentro del ejemplo del patron.
    pub fn executions(&self) -> usize {
        self.executions
    }
}
