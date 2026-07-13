/// Contrato publico `DiscountStrategy` que desacopla las piezas del ejemplo.
pub trait DiscountStrategy {
    /// Operacion `total after discount` definida por la abstraccion del ejemplo.
    fn total_after_discount(&self, subtotal: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `PercentageDiscount` usado por el ejemplo para expresar el dominio del patron.
pub struct PercentageDiscount {
    percent: f64,
}

impl PercentageDiscount {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(percent: f64) -> Self {
        Self { percent }
    }
}

impl DiscountStrategy for PercentageDiscount {
    /// Operacion `total after discount` definida por la abstraccion del ejemplo.
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        subtotal * (1.0 - self.percent / 100.0)
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `FixedAmountDiscount` usado por el ejemplo para expresar el dominio del patron.
pub struct FixedAmountDiscount {
    amount: f64,
}

impl FixedAmountDiscount {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(amount: f64) -> Self {
        Self { amount }
    }
}

impl DiscountStrategy for FixedAmountDiscount {
    /// Operacion `total after discount` definida por la abstraccion del ejemplo.
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        (subtotal - self.amount).max(0.0)
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `VolumeDiscount` usado por el ejemplo para expresar el dominio del patron.
pub struct VolumeDiscount {
    minimum_subtotal: f64,
    percent: f64,
}

impl VolumeDiscount {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(minimum_subtotal: f64, percent: f64) -> Self {
        Self {
            minimum_subtotal,
            percent,
        }
    }
}

impl DiscountStrategy for VolumeDiscount {
    /// Operacion `total after discount` definida por la abstraccion del ejemplo.
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        if subtotal >= self.minimum_subtotal {
            subtotal * (1.0 - self.percent / 100.0)
        } else {
            subtotal
        }
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `Checkout` usado por el ejemplo para expresar el dominio del patron.
pub struct Checkout<S>
where
    S: DiscountStrategy,
{
    items: Vec<f64>,
    discount: S,
}

impl<S> Checkout<S>
where
    S: DiscountStrategy,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(items: Vec<f64>, discount: S) -> Self {
        Self { items, discount }
    }

    /// Modela la operacion `subtotal` dentro del ejemplo del patron.
    pub fn subtotal(&self) -> f64 {
        self.items.iter().sum()
    }

    /// Modela la operacion `total` dentro del ejemplo del patron.
    pub fn total(&self) -> f64 {
        self.discount.total_after_discount(self.subtotal())
    }
}
