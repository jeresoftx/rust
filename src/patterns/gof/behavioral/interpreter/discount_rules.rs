/// Tipo publico `CartContext` usado por el ejemplo para expresar el dominio del patron.
pub struct CartContext {
    total: u32,
    customer_segment: String,
}

impl CartContext {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(total: u32, customer_segment: impl Into<String>) -> Self {
        Self {
            total,
            customer_segment: customer_segment.into(),
        }
    }
}

/// Conjunto de estados o errores publicos de `DiscountRule` dentro del ejemplo.
pub enum DiscountRule {
    /// Variante `MinTotal` del estado o error del ejemplo.
    MinTotal(u32),
    /// Variante `CustomerSegment` del estado o error del ejemplo.
    CustomerSegment(String),
    /// Variante `And` del estado o error del ejemplo.
    And {
        /// Valor publico `left` asociado a la variante del enum.
        left: Box<DiscountRule>,
        /// Valor publico `right` asociado a la variante del enum.
        right: Box<DiscountRule>,
        /// Valor publico `discount` asociado a la variante del enum.
        discount: u8,
    },
    /// Variante `Or` del estado o error del ejemplo.
    Or {
        /// Valor publico `left` asociado a la variante del enum.
        left: Box<DiscountRule>,
        /// Valor publico `right` asociado a la variante del enum.
        right: Box<DiscountRule>,
        /// Valor publico `discount` asociado a la variante del enum.
        discount: u8,
    },
}

impl DiscountRule {
    /// Modela la operacion `min total` dentro del ejemplo del patron.
    pub fn min_total(total: u32) -> Self {
        Self::MinTotal(total)
    }

    /// Modela la operacion `customer segment` dentro del ejemplo del patron.
    pub fn customer_segment(segment: impl Into<String>) -> Self {
        Self::CustomerSegment(segment.into())
    }

    /// Modela la operacion `and` dentro del ejemplo del patron.
    pub fn and(left: Self, right: Self, discount: u8) -> Self {
        Self::And {
            left: Box::new(left),
            right: Box::new(right),
            discount,
        }
    }

    /// Modela la operacion `or` dentro del ejemplo del patron.
    pub fn or(left: Self, right: Self, discount: u8) -> Self {
        Self::Or {
            left: Box::new(left),
            right: Box::new(right),
            discount,
        }
    }

    /// Modela la operacion `discount for` dentro del ejemplo del patron.
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

    /// Operacion `matches` definida por la abstraccion del ejemplo.
    fn matches(&self, cart: &CartContext) -> bool {
        match self {
            Self::MinTotal(total) => cart.total >= *total,
            Self::CustomerSegment(segment) => cart.customer_segment == *segment,
            Self::And { left, right, .. } => left.matches(cart) && right.matches(cart),
            Self::Or { left, right, .. } => left.matches(cart) || right.matches(cart),
        }
    }
}
