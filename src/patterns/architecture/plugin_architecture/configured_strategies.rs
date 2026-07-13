use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `PricingConfig` usado por el ejemplo para expresar el dominio del patron.
pub struct PricingConfig {
    strategy_key: String,
    value: u32,
}

impl PricingConfig {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(strategy_key: impl Into<String>, value: u32) -> Self {
        Self {
            strategy_key: strategy_key.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PricingError` dentro del ejemplo.
pub enum PricingError {
    /// Variante `StrategyNotFound` del estado o error del ejemplo.
    StrategyNotFound(String),
}

trait PricingStrategy {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str;
    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, subtotal_cents: u32, config_value: u32) -> u32;
}

#[derive(Default)]
/// Tipo publico `PricingRegistry` usado por el ejemplo para expresar el dominio del patron.
pub struct PricingRegistry {
    strategies: HashMap<String, Box<dyn PricingStrategy>>,
}

impl PricingRegistry {
    /// Modela la operacion `with default strategies` dentro del ejemplo del patron.
    pub fn with_default_strategies() -> Self {
        let mut registry = Self::default();
        registry.register(PercentageDiscount);
        registry.register(FixedAmountDiscount);
        registry
    }

    /// Operacion `register` definida por la abstraccion del ejemplo.
    fn register(&mut self, strategy: impl PricingStrategy + 'static) {
        self.strategies
            .insert(strategy.key().to_string(), Box::new(strategy));
    }

    /// Operacion `resolve` definida por la abstraccion del ejemplo.
    fn resolve(&self, key: &str) -> Option<&dyn PricingStrategy> {
        self.strategies.get(key).map(Box::as_ref)
    }
}

/// Tipo publico `DiscountEngine` usado por el ejemplo para expresar el dominio del patron.
pub struct DiscountEngine {
    registry: PricingRegistry,
    config: PricingConfig,
}

impl DiscountEngine {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(registry: PricingRegistry, config: PricingConfig) -> Self {
        Self { registry, config }
    }

    /// Modela la operacion `price after discount` dentro del ejemplo del patron.
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
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str {
        "percentage"
    }

    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, subtotal_cents: u32, percent: u32) -> u32 {
        subtotal_cents.saturating_sub(subtotal_cents * percent / 100)
    }
}

struct FixedAmountDiscount;

impl PricingStrategy for FixedAmountDiscount {
    /// Operacion `key` definida por la abstraccion del ejemplo.
    fn key(&self) -> &'static str {
        "fixed"
    }

    /// Operacion `apply` definida por la abstraccion del ejemplo.
    fn apply(&self, subtotal_cents: u32, amount_cents: u32) -> u32 {
        subtotal_cents.saturating_sub(amount_cents)
    }
}
