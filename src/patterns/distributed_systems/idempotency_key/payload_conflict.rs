use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPayload {
    customer_id: String,
    amount_cents: u64,
}

impl RequestPayload {
    pub fn new(customer_id: impl Into<String>, amount_cents: u64) -> Self {
        Self {
            customer_id: customer_id.into(),
            amount_cents,
        }
    }

    fn fingerprint(&self) -> String {
        format!("{}:{}", self.customer_id, self.amount_cents)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredResponse {
    body: String,
}

impl StoredResponse {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictError {
    PayloadMismatch,
}

#[derive(Debug, Clone)]
struct StoredEntry {
    fingerprint: String,
    response: StoredResponse,
}

#[derive(Debug, Default)]
pub struct IdempotencyStore {
    entries: HashMap<IdempotencyKey, StoredEntry>,
    executions: usize,
}

impl IdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn executions(&self) -> usize {
        self.executions
    }
}
