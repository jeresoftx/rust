pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NewUser {
        name: String,
        email: String,
    }

    impl NewUser {
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Result<Self, DomainError> {
            let name = name.into();
            let email = email.into();

            if name.trim().is_empty() {
                return Err(DomainError::NameRequired);
            }

            if !email.contains('@') {
                return Err(DomainError::InvalidEmail);
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
    pub struct User {
        id: String,
        name: String,
        email: String,
    }

    impl User {
        pub fn registered(id: impl Into<String>, user: NewUser) -> Self {
            Self {
                id: id.into(),
                name: user.name,
                email: user.email,
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
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomainError {
        NameRequired,
        InvalidEmail,
    }

    impl DomainError {
        pub fn message(&self) -> &'static str {
            match self {
                Self::NameRequired => "name is required",
                Self::InvalidEmail => "email must contain @",
            }
        }
    }
}

pub mod application {
    use super::domain::{DomainError, NewUser, User};

    pub trait UserRepository {
        fn save(&mut self, user: NewUser) -> User;
        fn count(&self) -> usize;
    }

    #[derive(Debug, Clone)]
    pub struct RegisterUser<R> {
        repository: R,
    }

    impl<R> RegisterUser<R>
    where
        R: UserRepository,
    {
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        pub fn execute(
            &mut self,
            name: impl Into<String>,
            email: impl Into<String>,
        ) -> Result<User, DomainError> {
            let new_user = NewUser::new(name, email)?;
            Ok(self.repository.save(new_user))
        }

        pub fn total_users(&self) -> usize {
            self.repository.count()
        }
    }
}

pub mod infrastructure {
    use super::application::UserRepository;
    use super::domain::{NewUser, User};

    #[derive(Debug, Clone, Default)]
    pub struct InMemoryUserRepository {
        users: Vec<User>,
        next_id: u32,
    }

    impl UserRepository for InMemoryUserRepository {
        fn save(&mut self, user: NewUser) -> User {
            self.next_id += 1;
            let registered = User::registered(format!("USR-{}", self.next_id), user);
            self.users.push(registered.clone());
            registered
        }

        fn count(&self) -> usize {
            self.users.len()
        }
    }
}

pub mod presentation {
    use super::application::{RegisterUser, UserRepository};
    use super::domain::User;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CreateUserRequest {
        name: String,
        email: String,
    }

    impl CreateUserRequest {
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UserController<R> {
        register_user: RegisterUser<R>,
    }

    impl<R> UserController<R>
    where
        R: UserRepository,
    {
        pub fn new(register_user: RegisterUser<R>) -> Self {
            Self { register_user }
        }

        pub fn create_user(&mut self, request: CreateUserRequest) -> CreateUserResponse {
            match self.register_user.execute(request.name, request.email) {
                Ok(user) => CreateUserResponse::created(UserResponse::from(user)),
                Err(error) => CreateUserResponse::bad_request(error.message()),
            }
        }

        pub fn total_users(&self) -> usize {
            self.register_user.total_users()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CreateUserResponse {
        status_code: u16,
        body: Option<UserResponse>,
        error: Option<String>,
    }

    impl CreateUserResponse {
        fn created(body: UserResponse) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &UserResponse {
            self.body.as_ref().expect("response should contain a body")
        }

        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UserResponse {
        id: String,
        name: String,
        email: String,
    }

    impl UserResponse {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn email(&self) -> &str {
            &self.email
        }

        pub fn display_label(&self) -> String {
            format!("{} <{}>", self.name, self.email)
        }
    }

    impl From<User> for UserResponse {
        fn from(user: User) -> Self {
            Self {
                id: user.id().to_string(),
                name: user.name().to_string(),
                email: user.email().to_string(),
            }
        }
    }
}
