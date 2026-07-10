pub trait PaymentGateway {
    fn charge(&self, customer_id: &str, amount_cents: u64) -> String;
}

pub struct ExternalPayClient {
    account_id: String,
}

impl ExternalPayClient {
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
        }
    }

    pub fn submit_payment_request(&self, buyer_ref: &str, total_in_cents: u64) -> String {
        format!(
            "provider=external-pay account={} customer={} amount={} status=approved",
            self.account_id, buyer_ref, total_in_cents
        )
    }
}

pub struct ExternalPayGatewayAdapter {
    client: ExternalPayClient,
}

impl ExternalPayGatewayAdapter {
    pub fn new(client: ExternalPayClient) -> Self {
        Self { client }
    }
}

impl PaymentGateway for ExternalPayGatewayAdapter {
    fn charge(&self, customer_id: &str, amount_cents: u64) -> String {
        self.client
            .submit_payment_request(customer_id, amount_cents)
    }
}

pub fn charge_customer(
    gateway: &dyn PaymentGateway,
    customer_id: &str,
    amount_cents: u64,
) -> String {
    gateway.charge(customer_id, amount_cents)
}
