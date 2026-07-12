#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode(String);

impl CurrencyCode {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    cents: i64,
    currency: CurrencyCode,
}

impl Money {
    pub fn new(cents: i64, currency: CurrencyCode) -> Self {
        Self { cents, currency }
    }

    pub fn cents(&self) -> i64 {
        self.cents
    }

    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

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

    pub fn format(&self) -> String {
        format!(
            "{} {}.{:02}",
            self.currency.as_str(),
            self.cents / 100,
            self.cents.abs() % 100
        )
    }
}
