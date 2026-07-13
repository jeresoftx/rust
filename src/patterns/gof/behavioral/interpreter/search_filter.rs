/// Tipo publico `SearchRecord` usado por el ejemplo para expresar el dominio del patron.
pub struct SearchRecord {
    status: String,
    total: u32,
}

impl SearchRecord {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(status: impl Into<String>, total: u32) -> Self {
        Self {
            status: status.into(),
            total,
        }
    }
}

/// Conjunto de estados o errores publicos de `SearchFilter` dentro del ejemplo.
pub enum SearchFilter {
    /// Variante `Equals` del estado o error del ejemplo.
    Equals {
        /// Valor publico `field` asociado a la variante `Equals`.
        field: String,
        /// Valor publico `value` asociado a la variante `Equals`.
        value: String,
    },
    /// Variante `GreaterThan` del estado o error del ejemplo.
    GreaterThan {
        /// Valor publico `field` asociado a la variante `GreaterThan`.
        field: String,
        /// Valor publico `value` asociado a la variante `GreaterThan`.
        value: u32,
    },
}

impl SearchFilter {
    /// Modela la operacion `parse` dentro del ejemplo del patron.
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("filter must have field operator value".to_string());
        }

        match parts[1] {
            "=" => Ok(Self::Equals {
                field: parts[0].to_string(),
                value: parts[2].to_string(),
            }),
            ">" => Ok(Self::GreaterThan {
                field: parts[0].to_string(),
                value: parts[2]
                    .parse()
                    .map_err(|_| "greater-than value must be numeric".to_string())?,
            }),
            operator => Err(format!("unsupported operator {operator}")),
        }
    }

    /// Modela la operacion `matches` dentro del ejemplo del patron.
    pub fn matches(&self, record: &SearchRecord) -> bool {
        match self {
            Self::Equals { field, value } if field == "status" => record.status == *value,
            Self::GreaterThan { field, value } if field == "total" => record.total > *value,
            _ => false,
        }
    }
}
