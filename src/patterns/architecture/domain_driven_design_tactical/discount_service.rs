#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    cents: u32,
}

impl Money {
    pub fn usd(cents: u32) -> Self {
        Self { cents }
    }

    pub fn percent(self, percent: u8) -> Self {
        Self {
            cents: self.cents * u32::from(percent) / 100,
        }
    }

    pub fn subtract(self, other: Self) -> Self {
        Self {
            cents: self.cents.saturating_sub(other.cents),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerSegment {
    Regular,
    Premium,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cart {
    subtotal: Money,
}

impl Cart {
    pub fn new(subtotal: Money) -> Self {
        Self { subtotal }
    }

    pub fn subtotal(&self) -> Money {
        self.subtotal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coupon {
    code: String,
    percent: u8,
}

impl Coupon {
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
pub enum CouponError {
    InvalidPercentage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscountBreakdown {
    subtotal: Money,
    discount: Money,
    reason: String,
}

impl DiscountBreakdown {
    pub fn new(subtotal: Money, discount: Money, reason: impl Into<String>) -> Self {
        Self {
            subtotal,
            discount,
            reason: reason.into(),
        }
    }

    pub fn discount(&self) -> Money {
        self.discount
    }

    pub fn total_after_discount(&self) -> Money {
        self.subtotal.subtract(self.discount)
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiscountPolicyService;

impl DiscountPolicyService {
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
