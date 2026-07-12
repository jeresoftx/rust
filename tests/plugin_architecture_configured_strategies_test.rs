use design_patterns_rust::patterns::architecture::plugin_architecture::configured_strategies::{
    DiscountEngine, PricingConfig, PricingError, PricingRegistry,
};

#[test]
fn registry_loads_percentage_strategy_from_configuration() {
    let registry = PricingRegistry::with_default_strategies();
    let config = PricingConfig::new("percentage", 15);
    let engine = DiscountEngine::new(registry, config);

    let total = engine
        .price_after_discount(10_000)
        .expect("strategy exists");

    assert_eq!(total, 8_500);
}

#[test]
fn registry_loads_fixed_amount_strategy_from_configuration() {
    let registry = PricingRegistry::with_default_strategies();
    let config = PricingConfig::new("fixed", 1_200);
    let engine = DiscountEngine::new(registry, config);

    let total = engine
        .price_after_discount(10_000)
        .expect("strategy exists");

    assert_eq!(total, 8_800);
}

#[test]
fn registry_never_returns_negative_totals_for_fixed_discounts() {
    let registry = PricingRegistry::with_default_strategies();
    let config = PricingConfig::new("fixed", 1_200);
    let engine = DiscountEngine::new(registry, config);

    let total = engine.price_after_discount(900).expect("strategy exists");

    assert_eq!(total, 0);
}

#[test]
fn registry_reports_unknown_strategy_from_configuration() {
    let registry = PricingRegistry::with_default_strategies();
    let config = PricingConfig::new("vip", 20);
    let engine = DiscountEngine::new(registry, config);

    let result = engine.price_after_discount(10_000);

    assert_eq!(
        result,
        Err(PricingError::StrategyNotFound("vip".to_string()))
    );
}
