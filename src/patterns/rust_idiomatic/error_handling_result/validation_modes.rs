#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    name: String,
    email: String,
    password: String,
}

impl Registration {
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
pub enum RegistrationError {
    NameRequired,
    InvalidEmail,
    WeakPassword,
}

impl RegistrationError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::NameRequired => "name is required",
            Self::InvalidEmail => "email must contain @",
            Self::WeakPassword => "password must have at least 8 characters",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    errors: Vec<RegistrationError>,
}

impl ValidationErrors {
    fn new(errors: Vec<RegistrationError>) -> Self {
        Self { errors }
    }

    pub fn messages(&self) -> Vec<&'static str> {
        self.errors.iter().map(RegistrationError::message).collect()
    }
}

pub fn validate_registration_fail_fast(
    registration: &Registration,
) -> Result<(), RegistrationError> {
    validate_name(registration)?;
    validate_email(registration)?;
    validate_password(registration)?;

    Ok(())
}

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

fn validate_name(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.name.trim().is_empty() {
        Err(RegistrationError::NameRequired)
    } else {
        Ok(())
    }
}

fn validate_email(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.email.contains('@') {
        Ok(())
    } else {
        Err(RegistrationError::InvalidEmail)
    }
}

fn validate_password(registration: &Registration) -> Result<(), RegistrationError> {
    if registration.password.chars().count() >= 8 {
        Ok(())
    } else {
        Err(RegistrationError::WeakPassword)
    }
}
