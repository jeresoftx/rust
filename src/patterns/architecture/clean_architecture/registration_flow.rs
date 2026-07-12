pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AccountStatus {
        Active,
        Pending,
    }

    impl AccountStatus {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Active => "active",
                Self::Pending => "pending",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegisteredUser {
        id: String,
        name: String,
        email: String,
        status: AccountStatus,
    }

    impl RegisteredUser {
        pub fn new(
            id: impl Into<String>,
            name: impl Into<String>,
            email: impl Into<String>,
            status: AccountStatus,
        ) -> Self {
            Self {
                id: id.into(),
                name: name.into(),
                email: email.into(),
                status,
            }
        }

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn email(&self) -> &str {
            &self.email
        }

        pub fn status(&self) -> &AccountStatus {
            &self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NewUser {
        name: String,
        email: String,
    }

    impl NewUser {
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Result<Self, EntityError> {
            let name = name.into();
            let email = email.into();

            if name.trim().is_empty() {
                return Err(EntityError::NameRequired);
            }

            if !email.contains('@') {
                return Err(EntityError::InvalidEmail);
            }

            Ok(Self { name, email })
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn email(&self) -> &str {
            &self.email
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum EntityError {
        NameRequired,
        InvalidEmail,
    }

    impl EntityError {
        pub fn message(&self) -> &'static str {
            match self {
                Self::NameRequired => "name is required",
                Self::InvalidEmail => "email is invalid",
            }
        }
    }
}

pub mod gateways {
    use super::entities::{AccountStatus, NewUser, RegisteredUser};

    pub trait UserGateway {
        fn save(&mut self, new_user: NewUser) -> RegisteredUser;
        fn all(&self) -> Vec<RegisteredUser>;
    }

    #[derive(Debug, Default, Clone)]
    pub struct InMemoryUserGateway {
        users: Vec<RegisteredUser>,
        next_id: usize,
    }

    impl UserGateway for InMemoryUserGateway {
        fn save(&mut self, new_user: NewUser) -> RegisteredUser {
            self.next_id += 1;
            let user = RegisteredUser::new(
                format!("USR-{}", self.next_id),
                new_user.name(),
                new_user.email(),
                AccountStatus::Active,
            );
            self.users.push(user.clone());
            user
        }

        fn all(&self) -> Vec<RegisteredUser> {
            self.users.clone()
        }
    }
}

pub mod use_cases {
    use super::entities::{EntityError, NewUser, RegisteredUser};
    use super::gateways::UserGateway;

    #[derive(Debug, Clone)]
    pub struct RegisterUser<G> {
        gateway: G,
    }

    impl<G> RegisterUser<G>
    where
        G: UserGateway,
    {
        pub fn new(gateway: G) -> Self {
            Self { gateway }
        }

        pub fn execute(
            &mut self,
            name: impl Into<String>,
            email: impl Into<String>,
        ) -> Result<RegisteredUser, EntityError> {
            let new_user = NewUser::new(name, email)?;
            Ok(self.gateway.save(new_user))
        }

        pub fn saved_users(&self) -> Vec<RegisteredUser> {
            self.gateway.all()
        }
    }
}

pub mod controllers {
    use super::entities::RegisteredUser;
    use super::gateways::UserGateway;
    use super::use_cases::RegisterUser;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegisterUserRequest {
        name: String,
        email: String,
    }

    impl RegisterUserRequest {
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegisteredUserResponse {
        id: String,
        status: String,
    }

    impl RegisteredUserResponse {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn status(&self) -> &str {
            &self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegisterUserResponse {
        status_code: u16,
        body: Option<RegisteredUserResponse>,
        error: Option<String>,
    }

    impl RegisterUserResponse {
        fn created(user: RegisteredUser) -> Self {
            Self {
                status_code: 201,
                body: Some(RegisteredUserResponse {
                    id: user.id().to_string(),
                    status: user.status().as_str().to_string(),
                }),
                error: None,
            }
        }

        fn validation_error(message: &'static str) -> Self {
            Self {
                status_code: 422,
                body: None,
                error: Some(message.to_string()),
            }
        }

        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &RegisteredUserResponse {
            self.body
                .as_ref()
                .expect("created response should include a body")
        }

        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone)]
    pub struct RegisterUserController<G> {
        use_case: RegisterUser<G>,
    }

    impl<G> RegisterUserController<G>
    where
        G: UserGateway,
    {
        pub fn new(use_case: RegisterUser<G>) -> Self {
            Self { use_case }
        }

        pub fn handle(&mut self, request: RegisterUserRequest) -> RegisterUserResponse {
            match self.use_case.execute(request.name, request.email) {
                Ok(user) => RegisterUserResponse::created(user),
                Err(error) => RegisterUserResponse::validation_error(error.message()),
            }
        }

        pub fn saved_users(&self) -> Vec<RegisteredUser> {
            self.use_case.saved_users()
        }
    }
}
