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
    sku: String,
    quantity: u32,
}

impl RequestPayload {
    pub fn new(customer_id: impl Into<String>, sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            customer_id: customer_id.into(),
            sku: sku.into(),
            quantity,
        }
    }

    fn fingerprint(&self) -> String {
        format!("{}:{}:{}", self.customer_id, self.sku, self.quantity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiResponse {
    status: u16,
    body: String,
}

impl ApiResponse {
    pub fn created(order_id: impl Into<String>) -> Self {
        Self {
            status: 201,
            body: order_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct StoredResponse {
    fingerprint: String,
    response: ApiResponse,
}

#[derive(Debug, Default)]
pub struct CachedResponseStore {
    responses: HashMap<IdempotencyKey, StoredResponse>,
    executions: usize,
}

impl CachedResponseStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(
        &mut self,
        key: IdempotencyKey,
        payload: RequestPayload,
    ) -> Result<ApiResponse, String> {
        let fingerprint = payload.fingerprint();
        if let Some(stored) = self.responses.get(&key) {
            if stored.fingerprint == fingerprint {
                return Ok(stored.response.clone());
            }

            return Err("idempotency key reused with a different request".to_string());
        }

        self.executions += 1;
        let response = ApiResponse::created(format!("order-{}", self.executions));
        self.responses.insert(
            key,
            StoredResponse {
                fingerprint,
                response: response.clone(),
            },
        );
        Ok(response)
    }

    pub fn executions(&self) -> usize {
        self.executions
    }

    pub fn cached_entries(&self) -> usize {
        self.responses.len()
    }
}
