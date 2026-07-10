use design_patterns_rust::patterns::gof::structural::adapter::payment_gateway::{
    ExternalPayClient, ExternalPayGatewayAdapter, PaymentGateway, charge_customer,
};

#[test]
fn adapter_charges_customer_through_internal_gateway_trait() {
    let gateway = ExternalPayGatewayAdapter::new(ExternalPayClient::new("mx-live"));

    let receipt = charge_customer(&gateway, "customer-42", 1499);

    assert_eq!(
        receipt,
        "provider=external-pay account=mx-live customer=customer-42 amount=1499 status=approved"
    );
}

#[test]
fn adapter_exposes_external_payment_client_as_payment_gateway() {
    let gateway: Box<dyn PaymentGateway> = Box::new(ExternalPayGatewayAdapter::new(
        ExternalPayClient::new("mx-live"),
    ));

    assert_eq!(
        gateway.charge("customer-99", 2500),
        "provider=external-pay account=mx-live customer=customer-99 amount=2500 status=approved"
    );
}
