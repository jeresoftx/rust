#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !value.starts_with("tok_") || value.len() < 16 {
            return Err(
                "token must start with tok_ and contain at least 16 characters".to_string(),
            );
        }

        Ok(Self(value))
    }

    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.0)
    }

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
pub struct LoginSession {
    email: EmailAddress,
    token: AuthToken,
}

impl LoginSession {
    pub fn new(email: EmailAddress, token: AuthToken) -> Self {
        Self { email, token }
    }

    pub fn principal(&self) -> &str {
        self.email.as_str()
    }

    pub fn audit_label(&self) -> String {
        format!("{} using {}", self.email.as_str(), self.token.redacted())
    }
}
