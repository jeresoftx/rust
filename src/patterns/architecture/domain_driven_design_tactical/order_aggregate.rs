#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderId(String);

impl OrderId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sku(String);

impl Sku {
    pub fn new(value: impl Into<String>) -> Result<Self, OrderError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(OrderError::InvalidSku);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    cents: u32,
    currency: Currency,
}

impl Money {
    pub fn usd(cents: u32) -> Self {
        Self {
            cents,
            currency: Currency::Usd,
        }
    }

    pub fn times(self, quantity: u32) -> Self {
        Self {
            cents: self.cents * quantity,
            currency: self.currency,
        }
    }

    pub fn add(self, other: Self) -> Self {
        assert_eq!(self.currency, other.currency, "currency must match");
        Self {
            cents: self.cents + other.cents,
            currency: self.currency,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Usd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Confirmed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderLine {
    sku: Sku,
    quantity: u32,
    unit_price: Money,
}

impl OrderLine {
    fn new(sku: Sku, quantity: u32, unit_price: Money) -> Result<Self, OrderError> {
        if quantity == 0 {
            return Err(OrderError::QuantityMustBePositive);
        }

        Ok(Self {
            sku,
            quantity,
            unit_price,
        })
    }

    fn subtotal(&self) -> Money {
        self.unit_price.times(self.quantity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    status: OrderStatus,
    lines: Vec<OrderLine>,
}

impl Order {
    pub fn new(id: OrderId) -> Self {
        Self {
            id,
            status: OrderStatus::Draft,
            lines: Vec::new(),
        }
    }

    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn add_item(
        &mut self,
        sku: Sku,
        quantity: u32,
        unit_price: Money,
    ) -> Result<(), OrderError> {
        if self.status == OrderStatus::Confirmed {
            return Err(OrderError::CannotChangeConfirmedOrder);
        }

        self.lines.push(OrderLine::new(sku, quantity, unit_price)?);
        Ok(())
    }

    pub fn confirm(&mut self) -> Result<(), OrderError> {
        if self.lines.is_empty() {
            return Err(OrderError::CannotConfirmEmptyOrder);
        }

        self.status = OrderStatus::Confirmed;
        Ok(())
    }

    pub fn total(&self) -> Money {
        self.lines
            .iter()
            .map(OrderLine::subtotal)
            .fold(Money::usd(0), Money::add)
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderError {
    InvalidSku,
    QuantityMustBePositive,
    CannotConfirmEmptyOrder,
    CannotChangeConfirmedOrder,
}
