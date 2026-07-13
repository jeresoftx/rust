#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OrderId` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderId(String);

impl OrderId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Sku` usado por el ejemplo para expresar el dominio del patron.
pub struct Sku(String);

impl Sku {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, OrderError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(OrderError::InvalidSku);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Money` usado por el ejemplo para expresar el dominio del patron.
pub struct Money {
    cents: u32,
    currency: Currency,
}

impl Money {
    /// Modela la operacion `usd` dentro del ejemplo del patron.
    pub fn usd(cents: u32) -> Self {
        Self {
            cents,
            currency: Currency::Usd,
        }
    }

    /// Modela la operacion `times` dentro del ejemplo del patron.
    pub fn times(self, quantity: u32) -> Self {
        Self {
            cents: self.cents * quantity,
            currency: self.currency,
        }
    }
}

impl Add for Money {
    /// Tipo asociado `Output` producido por la abstraccion del ejemplo.
    type Output = Self;

    /// Operacion `add` definida por la abstraccion del ejemplo.
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.currency, other.currency, "currency must match");
        Self {
            cents: self.cents + other.cents,
            currency: self.currency,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `Currency` dentro del ejemplo.
pub enum Currency {
    /// Variante `Usd` del estado o error del ejemplo.
    Usd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderStatus` dentro del ejemplo.
pub enum OrderStatus {
    /// Variante `Draft` del estado o error del ejemplo.
    Draft,
    /// Variante `Confirmed` del estado o error del ejemplo.
    Confirmed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderLine {
    sku: Sku,
    quantity: u32,
    unit_price: Money,
}

impl OrderLine {
    /// Operacion `new` definida por la abstraccion del ejemplo.
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

    /// Operacion `subtotal` definida por la abstraccion del ejemplo.
    fn subtotal(&self) -> Money {
        self.unit_price.times(self.quantity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: OrderId,
    status: OrderStatus,
    lines: Vec<OrderLine>,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: OrderId) -> Self {
        Self {
            id,
            status: OrderStatus::Draft,
            lines: Vec::new(),
        }
    }

    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &OrderId {
        &self.id
    }

    /// Modela la operacion `status` dentro del ejemplo del patron.
    pub fn status(&self) -> OrderStatus {
        self.status
    }

    /// Modela la operacion `add item` dentro del ejemplo del patron.
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

    /// Modela la operacion `confirm` dentro del ejemplo del patron.
    pub fn confirm(&mut self) -> Result<(), OrderError> {
        if self.lines.is_empty() {
            return Err(OrderError::CannotConfirmEmptyOrder);
        }

        self.status = OrderStatus::Confirmed;
        Ok(())
    }

    /// Modela la operacion `total` dentro del ejemplo del patron.
    pub fn total(&self) -> Money {
        self.lines
            .iter()
            .map(OrderLine::subtotal)
            .fold(Money::usd(0), Add::add)
    }

    /// Modela la operacion `line count` dentro del ejemplo del patron.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderError` dentro del ejemplo.
pub enum OrderError {
    /// Variante `InvalidSku` del estado o error del ejemplo.
    InvalidSku,
    /// Variante `QuantityMustBePositive` del estado o error del ejemplo.
    QuantityMustBePositive,
    /// Variante `CannotConfirmEmptyOrder` del estado o error del ejemplo.
    CannotConfirmEmptyOrder,
    /// Variante `CannotChangeConfirmedOrder` del estado o error del ejemplo.
    CannotChangeConfirmedOrder,
}
use std::ops::Add;
