use design_patterns_rust::patterns::gof::behavioral::strategy::discounts::{
    Checkout, FixedAmountDiscount, PercentageDiscount, VolumeDiscount,
};

#[test]
fn strategy_applies_percentage_discount_to_checkout_total() {
    let checkout = Checkout::new(vec![120.0, 80.0], PercentageDiscount::new(10.0));

    assert_eq!(checkout.subtotal(), 200.0);
    assert_eq!(checkout.total(), 180.0);
}

#[test]
fn strategy_clamps_fixed_amount_discount_at_zero() {
    let checkout = Checkout::new(vec![15.0, 5.0], FixedAmountDiscount::new(50.0));

    assert_eq!(checkout.subtotal(), 20.0);
    assert_eq!(checkout.total(), 0.0);
}

#[test]
fn strategy_applies_volume_discount_only_after_threshold() {
    let small_checkout = Checkout::new(vec![70.0, 80.0], VolumeDiscount::new(200.0, 15.0));
    let large_checkout = Checkout::new(vec![120.0, 110.0], VolumeDiscount::new(200.0, 15.0));

    assert_eq!(small_checkout.total(), 150.0);
    assert_eq!(large_checkout.total(), 195.5);
}
