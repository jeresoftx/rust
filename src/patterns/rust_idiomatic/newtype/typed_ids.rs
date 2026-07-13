#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `UserId` usado por el ejemplo para expresar el dominio del patron.
pub struct UserId(String);

impl UserId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("usr_") {
            return Err("UserId must start with usr_".to_string());
        }
        Ok(Self(value))
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `OrderId` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderId(String);

impl OrderId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("ord_") {
            return Err("OrderId must start with ord_".to_string());
        }
        Ok(Self(value))
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `ProductId` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductId(String);

impl ProductId {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("prd_") {
            return Err("ProductId must start with prd_".to_string());
        }
        Ok(Self(value))
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OrderAssignment` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderAssignment {
    user_id: UserId,
    order_id: OrderId,
}

impl OrderAssignment {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(user_id: UserId, order_id: OrderId) -> Self {
        Self { user_id, order_id }
    }

    /// Modela la operacion `audit key` dentro del ejemplo del patron.
    pub fn audit_key(&self) -> String {
        format!("{}->{}", self.user_id.as_str(), self.order_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ProductReservation` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductReservation {
    order_id: OrderId,
    product_id: ProductId,
    quantity: u32,
}

impl ProductReservation {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: OrderId, product_id: ProductId, quantity: u32) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} reserves {} of {}",
            self.order_id.as_str(),
            self.quantity,
            self.product_id.as_str()
        )
    }
}
