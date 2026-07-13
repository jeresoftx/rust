/// Contrato publico `PaymentGateway` que desacopla las piezas del ejemplo.
pub trait PaymentGateway {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, customer_id: &str, amount_cents: u64) -> String;
}

/// Tipo publico `ExternalPayClient` usado por el ejemplo para expresar el dominio del patron.
pub struct ExternalPayClient {
    account_id: String,
}

impl ExternalPayClient {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
        }
    }

    /// Modela la operacion `submit payment request` dentro del ejemplo del patron.
    pub fn submit_payment_request(&self, buyer_ref: &str, total_in_cents: u64) -> String {
        format!(
            "provider=external-pay account={} customer={} amount={} status=approved",
            self.account_id, buyer_ref, total_in_cents
        )
    }
}

/// Tipo publico `ExternalPayGatewayAdapter` usado por el ejemplo para expresar el dominio del patron.
pub struct ExternalPayGatewayAdapter {
    client: ExternalPayClient,
}

impl ExternalPayGatewayAdapter {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(client: ExternalPayClient) -> Self {
        Self { client }
    }
}

impl PaymentGateway for ExternalPayGatewayAdapter {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, customer_id: &str, amount_cents: u64) -> String {
        self.client
            .submit_payment_request(customer_id, amount_cents)
    }
}

/// Modela la operacion `charge customer` dentro del ejemplo del patron.
pub fn charge_customer(
    gateway: &dyn PaymentGateway,
    customer_id: &str,
    amount_cents: u64,
) -> String {
    gateway.charge(customer_id, amount_cents)
}
