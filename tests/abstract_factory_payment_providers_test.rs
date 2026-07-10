use design_patterns_rust::patterns::gof::creational::abstract_factory::payment_providers::{
    PaypalLikeFactory, StripeLikeFactory, checkout,
};

#[test]
fn abstract_factory_builds_a_stripe_like_payment_family() {
    let factory = StripeLikeFactory;

    let result = checkout(&factory, "order-1001", 25_99);

    assert_eq!(
        result,
        "gateway=stripe | authorization=pi_order-1001_2599 | receipt=receipt.stripe/order-1001"
    );
}

#[test]
fn abstract_factory_builds_a_paypal_like_payment_family() {
    let factory = PaypalLikeFactory;

    let result = checkout(&factory, "order-1001", 25_99);

    assert_eq!(
        result,
        "gateway=paypal | authorization=paypal-order-1001-2599 | receipt=paypal.me/receipts/order-1001"
    );
}
