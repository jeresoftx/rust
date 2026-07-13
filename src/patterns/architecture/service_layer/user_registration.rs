use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `User` usado por el ejemplo para expresar el dominio del patron.
pub struct User {
    id: String,
    name: String,
    email: String,
}

impl User {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RegistrationRequest` usado por el ejemplo para expresar el dominio del patron.
pub struct RegistrationRequest {
    user_id: String,
    name: String,
    email: String,
}

impl RegistrationRequest {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        user_id: impl Into<String>,
        name: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RegistrationResponse` usado por el ejemplo para expresar el dominio del patron.
pub struct RegistrationResponse {
    user_id: String,
}

impl RegistrationResponse {
    /// Modela la operacion `user id` dentro del ejemplo del patron.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `RegistrationError` dentro del ejemplo.
pub enum RegistrationError {
    /// Variante `InvalidEmail` del estado o error del ejemplo.
    InvalidEmail,
    /// Variante `UserAlreadyExists` del estado o error del ejemplo.
    UserAlreadyExists,
}

#[derive(Debug, Default)]
/// Tipo publico `InMemoryUserRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
}

impl InMemoryUserRepository {
    /// Operacion `exists` definida por la abstraccion del ejemplo.
    fn exists(&self, user_id: &str) -> bool {
        self.users.contains_key(user_id)
    }

    /// Operacion `save` definida por la abstraccion del ejemplo.
    fn save(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, user_id: &str) -> Option<User> {
        self.users.get(user_id).cloned()
    }
}

#[derive(Debug, Default)]
/// Tipo publico `EmailGateway` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailGateway {
    sent_emails: Vec<String>,
}

impl EmailGateway {
    /// Operacion `send welcome` definida por la abstraccion del ejemplo.
    fn send_welcome(&mut self, email: &str) {
        self.sent_emails.push(format!("welcome:{email}"));
    }

    /// Operacion `sent emails` definida por la abstraccion del ejemplo.
    fn sent_emails(&self) -> Vec<String> {
        self.sent_emails.clone()
    }
}

#[derive(Debug)]
/// Tipo publico `RegistrationService` usado por el ejemplo para expresar el dominio del patron.
pub struct RegistrationService {
    repository: InMemoryUserRepository,
    email_gateway: EmailGateway,
}

impl RegistrationService {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(repository: InMemoryUserRepository, email_gateway: EmailGateway) -> Self {
        Self {
            repository,
            email_gateway,
        }
    }

    /// Modela la operacion `register` dentro del ejemplo del patron.
    pub fn register(
        &mut self,
        request: RegistrationRequest,
    ) -> Result<RegistrationResponse, RegistrationError> {
        if !request.email.contains('@') {
            return Err(RegistrationError::InvalidEmail);
        }

        if self.repository.exists(&request.user_id) {
            return Err(RegistrationError::UserAlreadyExists);
        }

        let user = User::new(&request.user_id, &request.name, &request.email);
        self.repository.save(user);
        self.email_gateway.send_welcome(&request.email);

        Ok(RegistrationResponse {
            user_id: request.user_id,
        })
    }

    /// Modela la operacion `find user` dentro del ejemplo del patron.
    pub fn find_user(&self, user_id: &str) -> Option<User> {
        self.repository.find(user_id)
    }

    /// Modela la operacion `sent emails` dentro del ejemplo del patron.
    pub fn sent_emails(&self) -> Vec<String> {
        self.email_gateway.sent_emails()
    }
}
