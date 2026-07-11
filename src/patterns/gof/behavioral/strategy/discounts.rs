pub trait DiscountStrategy {
    fn total_after_discount(&self, subtotal: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct PercentageDiscount {
    percent: f64,
}

impl PercentageDiscount {
    pub fn new(percent: f64) -> Self {
        Self { percent }
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        subtotal * (1.0 - self.percent / 100.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedAmountDiscount {
    amount: f64,
}

impl FixedAmountDiscount {
    pub fn new(amount: f64) -> Self {
        Self { amount }
    }
}

impl DiscountStrategy for FixedAmountDiscount {
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        (subtotal - self.amount).max(0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VolumeDiscount {
    minimum_subtotal: f64,
    percent: f64,
}

impl VolumeDiscount {
    pub fn new(minimum_subtotal: f64, percent: f64) -> Self {
        Self {
            minimum_subtotal,
            percent,
        }
    }
}

impl DiscountStrategy for VolumeDiscount {
    fn total_after_discount(&self, subtotal: f64) -> f64 {
        if subtotal >= self.minimum_subtotal {
            subtotal * (1.0 - self.percent / 100.0)
        } else {
            subtotal
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn new(items: Vec<f64>, discount: S) -> Self {
        Self { items, discount }
    }

    pub fn subtotal(&self) -> f64 {
        self.items.iter().sum()
    }

    pub fn total(&self) -> f64 {
        self.discount.total_after_discount(self.subtotal())
    }
}
