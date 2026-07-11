use design_patterns_rust::patterns::gof::behavioral::strategy::shipping::{
    DistanceBasedShipping, FlatRateShipping, FreeShippingOverSubtotal, Shipment, ShippingCalculator,
};

#[test]
fn strategy_calculates_flat_rate_shipping() {
    let shipment = Shipment::new(35_000, 2, 8);
    let calculator = ShippingCalculator::new(FlatRateShipping::new(9_900));

    assert_eq!(calculator.quote_cents(&shipment), 9_900);
}

#[test]
fn strategy_calculates_shipping_by_distance_and_weight() {
    let shipment = Shipment::new(42_000, 3, 12);
    let calculator = ShippingCalculator::new(DistanceBasedShipping::new(2_500, 120, 450));

    assert_eq!(calculator.quote_cents(&shipment), 5_290);
}

#[test]
fn strategy_grants_free_shipping_after_subtotal_threshold() {
    let premium_order = Shipment::new(72_000, 1, 5);
    let small_order = Shipment::new(49_999, 1, 5);
    let calculator = ShippingCalculator::new(FreeShippingOverSubtotal::new(50_000, 7_900));

    assert_eq!(calculator.quote_cents(&premium_order), 0);
    assert_eq!(calculator.quote_cents(&small_order), 7_900);
}
