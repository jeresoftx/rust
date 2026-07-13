#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Money` usado por el ejemplo para expresar el dominio del patron.
pub struct Money {
    cents: u32,
}

impl Money {
    /// Modela la operacion `usd` dentro del ejemplo del patron.
    pub fn usd(cents: u32) -> Self {
        Self { cents }
    }

    /// Modela la operacion `percent` dentro del ejemplo del patron.
    pub fn percent(self, percent: u8) -> Self {
        Self {
            cents: self.cents * u32::from(percent) / 100,
        }
    }

    /// Modela la operacion `subtract` dentro del ejemplo del patron.
    pub fn subtract(self, other: Self) -> Self {
        Self {
            cents: self.cents.saturating_sub(other.cents),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CustomerSegment` dentro del ejemplo.
pub enum CustomerSegment {
    /// Variante `Regular` del estado o error del ejemplo.
    Regular,
    /// Variante `Premium` del estado o error del ejemplo.
    Premium,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Cart` usado por el ejemplo para expresar el dominio del patron.
pub struct Cart {
    subtotal: Money,
}

impl Cart {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(subtotal: Money) -> Self {
        Self { subtotal }
    }

    /// Modela la operacion `subtotal` dentro del ejemplo del patron.
    pub fn subtotal(&self) -> Money {
        self.subtotal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Coupon` usado por el ejemplo para expresar el dominio del patron.
pub struct Coupon {
    code: String,
    percent: u8,
}

impl Coupon {
    /// Modela la operacion `percent` dentro del ejemplo del patron.
    pub fn percent(code: impl Into<String>, percent: u8) -> Result<Self, CouponError> {
        if percent == 0 || percent >= 100 {
            return Err(CouponError::InvalidPercentage);
        }

        Ok(Self {
            code: code.into(),
            percent,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `CouponError` dentro del ejemplo.
pub enum CouponError {
    /// Variante `InvalidPercentage` del estado o error del ejemplo.
    InvalidPercentage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DiscountBreakdown` usado por el ejemplo para expresar el dominio del patron.
pub struct DiscountBreakdown {
    subtotal: Money,
    discount: Money,
    reason: String,
}

impl DiscountBreakdown {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(subtotal: Money, discount: Money, reason: impl Into<String>) -> Self {
        Self {
            subtotal,
            discount,
            reason: reason.into(),
        }
    }

    /// Modela la operacion `discount` dentro del ejemplo del patron.
    pub fn discount(&self) -> Money {
        self.discount
    }

    /// Modela la operacion `total after discount` dentro del ejemplo del patron.
    pub fn total_after_discount(&self) -> Money {
        self.subtotal.subtract(self.discount)
    }

    /// Modela la operacion `reason` dentro del ejemplo del patron.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Default, Clone)]
/// Tipo publico `DiscountPolicyService` usado por el ejemplo para expresar el dominio del patron.
pub struct DiscountPolicyService;

impl DiscountPolicyService {
    /// Modela la operacion `calculate discount` dentro del ejemplo del patron.
    pub fn calculate_discount(
        &self,
        segment: CustomerSegment,
        cart: &Cart,
        coupon: Option<&Coupon>,
    ) -> DiscountBreakdown {
        let segment_discount = match segment {
            CustomerSegment::Regular => {
                DiscountBreakdown::new(cart.subtotal(), Money::usd(0), "no discount")
            }
            CustomerSegment::Premium => DiscountBreakdown::new(
                cart.subtotal(),
                cart.subtotal().percent(15),
                "premium segment",
            ),
        };

        let Some(coupon) = coupon else {
            return segment_discount;
        };

        let coupon_discount = DiscountBreakdown::new(
            cart.subtotal(),
            cart.subtotal().percent(coupon.percent),
            format!("coupon {}", coupon.code),
        );

        if coupon_discount.discount.cents > segment_discount.discount.cents {
            coupon_discount
        } else {
            segment_discount
        }
    }
}
