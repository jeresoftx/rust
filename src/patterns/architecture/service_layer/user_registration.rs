use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationRequest {
    user_id: String,
    name: String,
    email: String,
}

impl RegistrationRequest {
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
pub struct RegistrationResponse {
    user_id: String,
}

impl RegistrationResponse {
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    InvalidEmail,
    UserAlreadyExists,
}

#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
}

impl InMemoryUserRepository {
    fn exists(&self, user_id: &str) -> bool {
        self.users.contains_key(user_id)
    }

    fn save(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    fn find(&self, user_id: &str) -> Option<User> {
        self.users.get(user_id).cloned()
    }
}

#[derive(Debug, Default)]
pub struct EmailGateway {
    sent_emails: Vec<String>,
}

impl EmailGateway {
    fn send_welcome(&mut self, email: &str) {
        self.sent_emails.push(format!("welcome:{email}"));
    }

    fn sent_emails(&self) -> Vec<String> {
        self.sent_emails.clone()
    }
}

#[derive(Debug)]
pub struct RegistrationService {
    repository: InMemoryUserRepository,
    email_gateway: EmailGateway,
}

impl RegistrationService {
    pub fn new(repository: InMemoryUserRepository, email_gateway: EmailGateway) -> Self {
        Self {
            repository,
            email_gateway,
        }
    }

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

    pub fn find_user(&self, user_id: &str) -> Option<User> {
        self.repository.find(user_id)
    }

    pub fn sent_emails(&self) -> Vec<String> {
        self.email_gateway.sent_emails()
    }
}
