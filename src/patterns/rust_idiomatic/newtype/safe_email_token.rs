#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `EmailAddress` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailAddress(String);

impl EmailAddress {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into().trim().to_lowercase();
        let Some((local, domain)) = value.split_once('@') else {
            return Err("email must contain a local part and domain".to_string());
        };

        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Err("email must contain a local part and domain".to_string());
        }

        Ok(Self(value))
    }

    /// Devuelve la representacion textual del valor.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Tipo publico `AuthToken` usado por el ejemplo para expresar el dominio del patron.
pub struct AuthToken(String);

impl AuthToken {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("tok_") || value.len() < 16 {
            return Err(
                "token must start with tok_ and contain at least 16 characters".to_string(),
            );
        }

        Ok(Self(value))
    }

    /// Modela la operacion `authorization header` dentro del ejemplo del patron.
    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.0)
    }

    /// Modela la operacion `redacted` dentro del ejemplo del patron.
    pub fn redacted(&self) -> String {
        let prefix: String = self.0.chars().take(9).collect();
        let suffix: String = self
            .0
            .chars()
            .rev()
            .take(4)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        format!("{prefix}...{suffix}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `LoginSession` usado por el ejemplo para expresar el dominio del patron.
pub struct LoginSession {
    email: EmailAddress,
    token: AuthToken,
}

impl LoginSession {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(email: EmailAddress, token: AuthToken) -> Self {
        Self { email, token }
    }

    /// Modela la operacion `principal` dentro del ejemplo del patron.
    pub fn principal(&self) -> &str {
        self.email.as_str()
    }

    /// Modela la operacion `audit label` dentro del ejemplo del patron.
    pub fn audit_label(&self) -> String {
        format!("{} using {}", self.email.as_str(), self.token.redacted())
    }
}
