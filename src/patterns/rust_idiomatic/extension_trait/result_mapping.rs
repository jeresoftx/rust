use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DomainError` usado por el ejemplo para expresar el dominio del patron.
pub struct DomainError {
    operation: String,
    message: String,
}

/// Contrato publico `ResultDomainExt` que desacopla las piezas del ejemplo.
pub trait ResultDomainExt<T, E> {
    /// Operacion `map domain` definida por la abstraccion del ejemplo.
    fn map_domain(self, operation: impl Into<String>) -> Result<T, DomainError>;

    /// Operacion `map domain with` definida por la abstraccion del ejemplo.
    fn map_domain_with(
        self,
        operation: impl Into<String>,
        message: impl FnOnce(E) -> String,
    ) -> Result<T, DomainError>;
}

impl<T, E> ResultDomainExt<T, E> for Result<T, E>
where
    E: Display,
{
    /// Operacion `map domain` definida por la abstraccion del ejemplo.
    fn map_domain(self, operation: impl Into<String>) -> Result<T, DomainError> {
        self.map_err(|error| DomainError {
            operation: operation.into(),
            message: error.to_string(),
        })
    }

    /// Operacion `map domain with` definida por la abstraccion del ejemplo.
    fn map_domain_with(
        self,
        operation: impl Into<String>,
        message: impl FnOnce(E) -> String,
    ) -> Result<T, DomainError> {
        self.map_err(|error| DomainError {
            operation: operation.into(),
            message: message(error),
        })
    }
}

impl DomainError {
    /// Modela la operacion `operation` dentro del ejemplo del patron.
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!("{} failed: {}", self.operation, self.message)
    }
}
