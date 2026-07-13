#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `InfrastructureError` dentro del ejemplo.
pub enum InfrastructureError {
    /// Variante `ConnectionLost` del estado o error del ejemplo.
    ConnectionLost,
    /// Variante `RowNotFound` del estado o error del ejemplo.
    RowNotFound {
        /// Valor publico `id` asociado a la variante `RowNotFound`.
        id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `AppError` dentro del ejemplo.
pub enum AppError {
    /// Variante `TemporarilyUnavailable` del estado o error del ejemplo.
    TemporarilyUnavailable {
        /// Valor publico `operation` asociado a la variante `TemporarilyUnavailable`.
        operation: String,
    },
    /// Variante `NotFound` del estado o error del ejemplo.
    NotFound {
        /// Valor publico `entity` asociado a la variante `NotFound`.
        entity: String,
        /// Valor publico `id` asociado a la variante `NotFound`.
        id: String,
    },
}

impl AppError {
    /// Modela la operacion `message` dentro del ejemplo del patron.
    pub fn message(&self) -> String {
        match self {
            Self::TemporarilyUnavailable { operation } => {
                format!("{operation} is temporarily unavailable")
            }
            Self::NotFound { entity, id } => format!("{entity} {id} was not found"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CustomerProfile` usado por el ejemplo para expresar el dominio del patron.
pub struct CustomerProfile {
    id: String,
    name: String,
}

impl CustomerProfile {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Modela la operacion `id` dentro del ejemplo del patron.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Modela la operacion `greeting` dentro del ejemplo del patron.
    pub fn greeting(&self) -> String {
        format!("Hola, {}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `CustomerRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct CustomerRepository {
    profiles: Vec<CustomerProfile>,
    available: bool,
}

impl CustomerRepository {
    /// Modela la operacion `with profiles` dentro del ejemplo del patron.
    pub fn with_profiles(profiles: Vec<CustomerProfile>) -> Self {
        Self {
            profiles,
            available: true,
        }
    }

    /// Modela la operacion `unavailable` dentro del ejemplo del patron.
    pub fn unavailable() -> Self {
        Self {
            profiles: Vec::new(),
            available: false,
        }
    }

    /// Operacion `find customer` definida por la abstraccion del ejemplo.
    fn find_customer(&self, id: &str) -> Result<CustomerProfile, InfrastructureError> {
        if !self.available {
            return Err(InfrastructureError::ConnectionLost);
        }

        self.profiles
            .iter()
            .find(|profile| profile.id == id)
            .cloned()
            .ok_or_else(|| InfrastructureError::RowNotFound { id: id.to_string() })
    }
}

/// Modela la operacion `load customer profile` dentro del ejemplo del patron.
pub fn load_customer_profile(
    repository: &CustomerRepository,
    id: &str,
) -> Result<CustomerProfile, AppError> {
    repository.find_customer(id).map_err(|error| match error {
        InfrastructureError::ConnectionLost => AppError::TemporarilyUnavailable {
            operation: "load customer profile".to_string(),
        },
        InfrastructureError::RowNotFound { id } => AppError::NotFound {
            entity: "customer".to_string(),
            id,
        },
    })
}
