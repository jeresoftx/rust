use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PricingConfig {
    strategy_key: String,
    value: u32,
}

impl PricingConfig {
    pub fn new(strategy_key: impl Into<String>, value: u32) -> Self {
        Self {
            strategy_key: strategy_key.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PricingError {
    StrategyNotFound(String),
}

trait PricingStrategy {
    fn key(&self) -> &'static str;
    fn apply(&self, subtotal_cents: u32, config_value: u32) -> u32;
}

#[derive(Default)]
pub struct PricingRegistry {
    strategies: HashMap<String, Box<dyn PricingStrategy>>,
}

impl PricingRegistry {
    pub fn with_default_strategies() -> Self {
        let mut registry = Self::default();
        registry.register(PercentageDiscount);
        registry.register(FixedAmountDiscount);
        registry
    }

    fn register(&mut self, strategy: impl PricingStrategy + 'static) {
        self.strategies
            .insert(strategy.key().to_string(), Box::new(strategy));
    }

    fn resolve(&self, key: &str) -> Option<&dyn PricingStrategy> {
        self.strategies.get(key).map(Box::as_ref)
    }
}

pub struct DiscountEngine {
    registry: PricingRegistry,
    config: PricingConfig,
}

impl DiscountEngine {
    pub fn new(registry: PricingRegistry, config: PricingConfig) -> Self {
        Self { registry, config }
    }

    pub fn price_after_discount(&self, subtotal_cents: u32) -> Result<u32, PricingError> {
        let strategy = self
            .registry
            .resolve(&self.config.strategy_key)
            .ok_or_else(|| PricingError::StrategyNotFound(self.config.strategy_key.clone()))?;

        Ok(strategy.apply(subtotal_cents, self.config.value))
    }
}

struct PercentageDiscount;

impl PricingStrategy for PercentageDiscount {
    fn key(&self) -> &'static str {
        "percentage"
    }

    fn apply(&self, subtotal_cents: u32, percent: u32) -> u32 {
        subtotal_cents.saturating_sub(subtotal_cents * percent / 100)
    }
}

struct FixedAmountDiscount;

impl PricingStrategy for FixedAmountDiscount {
    fn key(&self) -> &'static str {
        "fixed"
    }

    fn apply(&self, subtotal_cents: u32, amount_cents: u32) -> u32 {
        subtotal_cents.saturating_sub(amount_cents)
    }
}
