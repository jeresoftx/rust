#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfrastructureError {
    ConnectionLost,
    RowNotFound { id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    TemporarilyUnavailable { operation: String },
    NotFound { entity: String, id: String },
}

impl AppError {
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
pub struct CustomerProfile {
    id: String,
    name: String,
}

impl CustomerProfile {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn greeting(&self) -> String {
        format!("Hola, {}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerRepository {
    profiles: Vec<CustomerProfile>,
    available: bool,
}

impl CustomerRepository {
    pub fn with_profiles(profiles: Vec<CustomerProfile>) -> Self {
        Self {
            profiles,
            available: true,
        }
    }

    pub fn unavailable() -> Self {
        Self {
            profiles: Vec::new(),
            available: false,
        }
    }

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
