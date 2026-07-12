use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainError {
    operation: String,
    message: String,
}

pub trait ResultDomainExt<T, E> {
    fn map_domain(self, operation: impl Into<String>) -> Result<T, DomainError>;

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
    fn map_domain(self, operation: impl Into<String>) -> Result<T, DomainError> {
        self.map_err(|error| DomainError {
            operation: operation.into(),
            message: error.to_string(),
        })
    }

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
    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn summary(&self) -> String {
        format!("{} failed: {}", self.operation, self.message)
    }
}
