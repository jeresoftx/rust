#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("usr_") {
            return Err("UserId must start with usr_".to_string());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(String);

impl OrderId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("ord_") {
            return Err("OrderId must start with ord_".to_string());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(String);

impl ProductId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("prd_") {
            return Err("ProductId must start with prd_".to_string());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderAssignment {
    user_id: UserId,
    order_id: OrderId,
}

impl OrderAssignment {
    pub fn new(user_id: UserId, order_id: OrderId) -> Self {
        Self { user_id, order_id }
    }

    pub fn audit_key(&self) -> String {
        format!("{}->{}", self.user_id.as_str(), self.order_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductReservation {
    order_id: OrderId,
    product_id: ProductId,
    quantity: u32,
}

impl ProductReservation {
    pub fn new(order_id: OrderId, product_id: ProductId, quantity: u32) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} reserves {} of {}",
            self.order_id.as_str(),
            self.quantity,
            self.product_id.as_str()
        )
    }
}
