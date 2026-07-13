//! User Api.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::layered_architecture::user_api as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `NewUser` usado por el ejemplo para expresar el dominio del patron.
    pub struct NewUser {
        name: String,
        email: String,
    }

    impl NewUser {
        /// Crea una instancia valida para el ejemplo del patron.
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

        /// Modela la operacion `name` dentro del ejemplo del patron.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Modela la operacion `email` dentro del ejemplo del patron.
        pub fn email(&self) -> &str {
            &self.email
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `User` usado por el ejemplo para expresar el dominio del patron.
    pub struct User {
        id: String,
        name: String,
        email: String,
    }

    impl User {
        /// Modela la operacion `registered` dentro del ejemplo del patron.
        pub fn registered(id: impl Into<String>, user: NewUser) -> Self {
            Self {
                id: id.into(),
                name: user.name,
                email: user.email,
            }
        }

        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `name` dentro del ejemplo del patron.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Modela la operacion `email` dentro del ejemplo del patron.
        pub fn email(&self) -> &str {
            &self.email
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `DomainError` dentro del ejemplo.
    pub enum DomainError {
        /// Variante `NameRequired` del estado o error del ejemplo.
        NameRequired,
        /// Variante `InvalidEmail` del estado o error del ejemplo.
        InvalidEmail,
    }

    impl DomainError {
        /// Modela la operacion `message` dentro del ejemplo del patron.
        pub fn message(&self) -> &'static str {
            match self {
                Self::NameRequired => "name is required",
                Self::InvalidEmail => "email must contain @",
            }
        }
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::{DomainError, NewUser, User};

    /// Contrato publico `UserRepository` que desacopla las piezas del ejemplo.
    pub trait UserRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, user: NewUser) -> User;
        /// Operacion `count` definida por la abstraccion del ejemplo.
        fn count(&self) -> usize;
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `RegisterUser` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterUser<R> {
        repository: R,
    }

    impl<R> RegisterUser<R>
    where
        R: UserRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(repository: R) -> Self {
            Self { repository }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(
            &mut self,
            name: impl Into<String>,
            email: impl Into<String>,
        ) -> Result<User, DomainError> {
            let new_user = NewUser::new(name, email)?;
            Ok(self.repository.save(new_user))
        }

        /// Modela la operacion `total users` dentro del ejemplo del patron.
        pub fn total_users(&self) -> usize {
            self.repository.count()
        }
    }
}

/// Modulo del ejemplo `infrastructure` dentro del catalogo de patrones.
pub mod infrastructure {
    use super::application::UserRepository;
    use super::domain::{NewUser, User};

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `InMemoryUserRepository` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryUserRepository {
        users: Vec<User>,
        next_id: u32,
    }

    impl UserRepository for InMemoryUserRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, user: NewUser) -> User {
            self.next_id += 1;
            let registered = User::registered(format!("USR-{}", self.next_id), user);
            self.users.push(registered.clone());
            registered
        }

        /// Operacion `count` definida por la abstraccion del ejemplo.
        fn count(&self) -> usize {
            self.users.len()
        }
    }
}

/// Modulo del ejemplo `presentation` dentro del catalogo de patrones.
pub mod presentation {
    use super::application::{RegisterUser, UserRepository};
    use super::domain::User;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CreateUserRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateUserRequest {
        name: String,
        email: String,
    }

    impl CreateUserRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
            }
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `UserController` usado por el ejemplo para expresar el dominio del patron.
    pub struct UserController<R> {
        register_user: RegisterUser<R>,
    }

    impl<R> UserController<R>
    where
        R: UserRepository,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(register_user: RegisterUser<R>) -> Self {
            Self { register_user }
        }

        /// Modela la operacion `create user` dentro del ejemplo del patron.
        pub fn create_user(&mut self, request: CreateUserRequest) -> CreateUserResponse {
            match self.register_user.execute(request.name, request.email) {
                Ok(user) => CreateUserResponse::created(UserResponse::from(user)),
                Err(error) => CreateUserResponse::bad_request(error.message()),
            }
        }

        /// Modela la operacion `total users` dentro del ejemplo del patron.
        pub fn total_users(&self) -> usize {
            self.register_user.total_users()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CreateUserResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct CreateUserResponse {
        status_code: u16,
        body: Option<UserResponse>,
        error: Option<String>,
    }

    impl CreateUserResponse {
        /// Operacion `created` definida por la abstraccion del ejemplo.
        fn created(body: UserResponse) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        /// Operacion `bad request` definida por la abstraccion del ejemplo.
        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &UserResponse {
            self.body.as_ref().expect("response should contain a body")
        }

        /// Modela la operacion `error` dentro del ejemplo del patron.
        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `UserResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct UserResponse {
        id: String,
        name: String,
        email: String,
    }

    impl UserResponse {
        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `name` dentro del ejemplo del patron.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Modela la operacion `email` dentro del ejemplo del patron.
        pub fn email(&self) -> &str {
            &self.email
        }

        /// Modela la operacion `display label` dentro del ejemplo del patron.
        pub fn display_label(&self) -> String {
            format!("{} <{}>", self.name, self.email)
        }
    }

    impl From<User> for UserResponse {
        /// Operacion `from` definida por la abstraccion del ejemplo.
        fn from(user: User) -> Self {
            Self {
                id: user.id().to_string(),
                name: user.name().to_string(),
                email: user.email().to_string(),
            }
        }
    }
}
