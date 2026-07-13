use std::collections::HashMap;
use std::sync::Arc;

/// Tipo publico `CurrencyCatalog` usado por el ejemplo para expresar el dominio del patron.
pub struct CurrencyCatalog {
    currencies: HashMap<String, Arc<Currency>>,
}

impl Default for CurrencyCatalog {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        let mut currencies = HashMap::new();
        currencies.insert("USD".to_string(), Arc::new(Currency::new("USD", "$", 2)));
        currencies.insert("MXN".to_string(), Arc::new(Currency::new("MXN", "$", 2)));
        currencies.insert("JPY".to_string(), Arc::new(Currency::new("JPY", "¥", 0)));

        Self { currencies }
    }
}

impl CurrencyCatalog {
    /// Modela la operacion `currency` dentro del ejemplo del patron.
    pub fn currency(&self, code: &str) -> Option<Arc<Currency>> {
        self.currencies.get(code).cloned()
    }
}

/// Tipo publico `Currency` usado por el ejemplo para expresar el dominio del patron.
pub struct Currency {
    code: String,
    symbol: String,
    decimals: u8,
}

impl Currency {
    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(code: impl Into<String>, symbol: impl Into<String>, decimals: u8) -> Self {
        Self {
            code: code.into(),
            symbol: symbol.into(),
            decimals,
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} symbol={} decimals={}",
            self.code, self.symbol, self.decimals
        )
    }
}

/// Tipo publico `MoneyAmount` usado por el ejemplo para expresar el dominio del patron.
pub struct MoneyAmount {
    minor_units: u64,
    currency: Arc<Currency>,
}

impl MoneyAmount {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(minor_units: u64, currency: Arc<Currency>) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    /// Modela la operacion `format` dentro del ejemplo del patron.
    pub fn format(&self) -> String {
        let divisor = 10_u64.pow(self.currency.decimals as u32);
        let major = self.minor_units / divisor;
        let minor = self.minor_units % divisor;

        if self.currency.decimals == 0 {
            return format!("{}{} {}", self.currency.symbol, major, self.currency.code);
        }

        format!(
            "{}{}.{:0width$} {}",
            self.currency.symbol,
            major,
            minor,
            self.currency.code,
            width = self.currency.decimals as usize
        )
    }
}
