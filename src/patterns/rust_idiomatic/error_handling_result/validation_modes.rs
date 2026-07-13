#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Registration` usado por el ejemplo para expresar el dominio del patron.
pub struct Registration {
    name: String,
    email: String,
    password: String,
}

impl Registration {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `RegistrationError` dentro del ejemplo.
pub enum RegistrationError {
    /// Variante `NameRequired` del estado o error del ejemplo.
    NameRequired,
    /// Variante `InvalidEmail` del estado o error del ejemplo.
    InvalidEmail,
    /// Variante `WeakPassword` del estado o error del ejemplo.
    WeakPassword,
}

impl RegistrationError {
    /// Modela la operacion `message` dentro del ejemplo del patron.
    pub fn message(&self) -> &'static str {
        match self {
            Self::NameRequired => "name is required",
            Self::InvalidEmail => "email must contain @",
            Self::WeakPassword => "password must have at least 8 characters",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ValidationErrors` usado por el ejemplo para expresar el dominio del patron.
pub struct ValidationErrors {
    errors: Vec<RegistrationError>,
}

impl ValidationErrors {
    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(errors: Vec<RegistrationError>) -> Self {
        Self { errors }
    }

    /// Modela la operacion `messages` dentro del ejemplo del patron.
    pub fn messages(&self) -> Vec<&'static str> {
        self.errors.iter().map(RegistrationError::message).collect()
    }
}

/// Modela la operacion `validate registration fail fast` dentro del ejemplo del patron.
pub fn validate_registration_fail_fast(
    registration: &Registration,
) -> Result<(), RegistrationError> {
    validate_name(registration)?;
    validate_email(registration)?;
    validate_password(registration)?;

    Ok(())
}

/// Modela la operacion `validate registration accumulated` dentro del ejemplo del patron.
pub fn validate_registration_accumulated(
    registration: &Registration,
) -> Result<(), ValidationErrors> {
    let errors = [
        validate_name(registration),
        validate_email(registration),
        validate_password(registration),
    ]
    .into_iter()
    .filter_map(Result::err)
    .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationErrors::new(errors))
    }
}

/// Operacion `validate name` definida por la abstraccion del ejemplo.
fn validate_name(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.name.trim().is_empty() {
        Err(RegistrationError::NameRequired)
    } else {
        Ok(())
    }
}

/// Operacion `validate email` definida por la abstraccion del ejemplo.
fn validate_email(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.email.contains('@') {
        Ok(())
    } else {
        Err(RegistrationError::InvalidEmail)
    }
}

/// Operacion `validate password` definida por la abstraccion del ejemplo.
fn validate_password(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.password.chars().count() >= 8 {
        Ok(())
    } else {
        Err(RegistrationError::WeakPassword)
    }
}
