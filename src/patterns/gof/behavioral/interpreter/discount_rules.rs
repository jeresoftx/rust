pub struct CartContext {
    total: u32,
    customer_segment: String,
}

impl CartContext {
    pub fn new(total: u32, customer_segment: impl Into<String>) -> Self {
        Self {
            total,
            customer_segment: customer_segment.into(),
        }
    }
}

pub enum DiscountRule {
    MinTotal(u32),
    CustomerSegment(String),
    And {
        left: Box<DiscountRule>,
        right: Box<DiscountRule>,
        discount: u8,
    },
    Or {
        left: Box<DiscountRule>,
        right: Box<DiscountRule>,
        discount: u8,
    },
}

impl DiscountRule {
    pub fn min_total(total: u32) -> Self {
        Self::MinTotal(total)
    }

    pub fn customer_segment(segment: impl Into<String>) -> Self {
        Self::CustomerSegment(segment.into())
    }

    pub fn and(left: Self, right: Self, discount: u8) -> Self {
        Self::And {
            left: Box::new(left),
            right: Box::new(right),
            discount,
        }
    }

    pub fn or(left: Self, right: Self, discount: u8) -> Self {
        Self::Or {
            left: Box::new(left),
            right: Box::new(right),
            discount,
        }
    }

    pub fn discount_for(&self, cart: &CartContext) -> u8 {
        match self {
            Self::And {
                left,
                right,
                discount,
            } if left.matches(cart) && right.matches(cart) => *discount,
            Self::Or {
                left,
                right,
                discount,
            } if left.matches(cart) || right.matches(cart) => *discount,
            _ => 0,
        }
    }

    fn matches(&self, cart: &CartContext) -> bool {
        match self {
            Self::MinTotal(total) => cart.total >= *total,
            Self::CustomerSegment(segment) => cart.customer_segment == *segment,
            Self::And { left, right, .. } => left.matches(cart) && right.matches(cart),
            Self::Or { left, right, .. } => left.matches(cart) || right.matches(cart),
        }
    }
}
