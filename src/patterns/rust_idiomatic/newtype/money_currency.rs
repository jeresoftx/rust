#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `CurrencyCode` usado por el ejemplo para expresar el dominio del patron.
pub struct CurrencyCode(String);

impl CurrencyCode {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let is_valid = value.len() == 3
            && value
                .chars()
                .all(|character| character.is_ascii_uppercase());

        if !is_valid {
            return Err("currency code must be three uppercase ASCII letters".to_string());
        }

        Ok(Self(value))
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Money` usado por el ejemplo para expresar el dominio del patron.
pub struct Money {
    cents: i64,
    currency: CurrencyCode,
}

impl Money {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(cents: i64, currency: CurrencyCode) -> Self {
        Self { cents, currency }
    }

    /// Modela la operacion `cents` dentro del ejemplo del patron.
    pub fn cents(&self) -> i64 {
        self.cents
    }

    /// Modela la operacion `currency` dentro del ejemplo del patron.
    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    /// Modela la operacion `add` dentro del ejemplo del patron.
    pub fn add(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err(format!(
                "cannot add {} money to {} money",
                self.currency.as_str(),
                other.currency.as_str()
            ));
        }

        Ok(Money::new(self.cents + other.cents, self.currency.clone()))
    }

    /// Modela la operacion `format` dentro del ejemplo del patron.
    pub fn format(&self) -> String {
        format!(
            "{} {}.{:02}",
            self.currency.as_str(),
            self.cents / 100,
            self.cents.abs() % 100
        )
    }
}
