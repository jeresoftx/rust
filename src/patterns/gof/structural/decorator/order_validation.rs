#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    lines: Vec<OrderLine>,
    total_cents: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct OrderLine {
    sku: String,
    quantity: u32,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, lines: Vec<(&str, u32)>, total_cents: u64) -> Self {
        Self {
            id: id.into(),
            lines: lines
                .into_iter()
                .map(|(sku, quantity)| OrderLine {
                    sku: sku.to_string(),
                    quantity,
                })
                .collect(),
            total_cents,
        }
    }
}

/// Contrato publico `ValidationStep` que desacopla las piezas del ejemplo.
pub trait ValidationStep {
    /// Operacion `validate` definida por la abstraccion del ejemplo.
    fn validate(&self, order: &Order) -> Result<Vec<String>, String>;
}

/// Tipo publico `ValidOrder` usado por el ejemplo para expresar el dominio del patron.
pub struct ValidOrder;

impl ValidationStep for ValidOrder {
    /// Operacion `validate` definida por la abstraccion del ejemplo.
    fn validate(&self, order: &Order) -> Result<Vec<String>, String> {
        Ok(vec![format!("order={} status=valid", order.id)])
    }
}

/// Tipo publico `NonEmptyOrderValidator` usado por el ejemplo para expresar el dominio del patron.
pub struct NonEmptyOrderValidator<V> {
    inner: V,
}

impl<V> NonEmptyOrderValidator<V> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: V) -> Self {
        Self { inner }
    }
}

impl<V> ValidationStep for NonEmptyOrderValidator<V>
where
    V: ValidationStep,
{
    /// Operacion `validate` definida por la abstraccion del ejemplo.
    fn validate(&self, order: &Order) -> Result<Vec<String>, String> {
        if order.lines.is_empty() {
            return Err("order must have at least one line".to_string());
        }

        let mut checks = self.inner.validate(order)?;
        checks.push("non-empty".to_string());
        Ok(checks)
    }
}

/// Tipo publico `MinimumAmountValidator` usado por el ejemplo para expresar el dominio del patron.
pub struct MinimumAmountValidator<V> {
    inner: V,
    minimum_cents: u64,
}

impl<V> MinimumAmountValidator<V> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: V, minimum_cents: u64) -> Self {
        Self {
            inner,
            minimum_cents,
        }
    }
}

impl<V> ValidationStep for MinimumAmountValidator<V>
where
    V: ValidationStep,
{
    /// Operacion `validate` definida por la abstraccion del ejemplo.
    fn validate(&self, order: &Order) -> Result<Vec<String>, String> {
        if order.total_cents < self.minimum_cents {
            return Err(format!(
                "order total must be at least {} cents",
                self.minimum_cents
            ));
        }

        let mut checks = self.inner.validate(order)?;
        checks.push("min-amount".to_string());
        Ok(checks)
    }
}

/// Tipo publico `InventoryValidator` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryValidator<V> {
    inner: V,
    available_skus: Vec<String>,
}

impl<V> InventoryValidator<V> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: V, available_skus: Vec<&str>) -> Self {
        Self {
            inner,
            available_skus: available_skus.into_iter().map(str::to_string).collect(),
        }
    }
}

impl<V> ValidationStep for InventoryValidator<V>
where
    V: ValidationStep,
{
    /// Operacion `validate` definida por la abstraccion del ejemplo.
    fn validate(&self, order: &Order) -> Result<Vec<String>, String> {
        for line in &order.lines {
            if !self.available_skus.contains(&line.sku) {
                return Err(format!("missing inventory for {}", line.sku));
            }
        }

        let mut checks = self.inner.validate(order)?;
        checks.push("inventory".to_string());
        Ok(checks)
    }
}

/// Modela la operacion `validate order` dentro del ejemplo del patron.
pub fn validate_order(validator: &dyn ValidationStep, order: &Order) -> String {
    match validator.validate(order) {
        Ok(checks) => {
            let (status, validations) = checks
                .split_first()
                .expect("valid order should include a status");
            format!("{} checks={}", status, validations.join(","))
        }
        Err(error) => format!("order={} status=invalid error={}", order.id, error),
    }
}
