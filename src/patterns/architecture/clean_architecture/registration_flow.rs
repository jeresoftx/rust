//! Registration Flow.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::clean_architecture::registration_flow as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `entities` dentro del catalogo de patrones.
pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `AccountStatus` dentro del ejemplo.
    pub enum AccountStatus {
        /// Variante `Active` del estado o error del ejemplo.
        Active,
        /// Variante `Pending` del estado o error del ejemplo.
        Pending,
    }

    impl AccountStatus {
        /// Devuelve la representacion textual del valor.
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Active => "active",
                Self::Pending => "pending",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `RegisteredUser` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisteredUser {
        id: String,
        name: String,
        email: String,
        status: AccountStatus,
    }

    impl RegisteredUser {
        /// Crea una instancia valida para el ejemplo del patron.
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

        /// Modela la operacion `status` dentro del ejemplo del patron.
        pub fn status(&self) -> &AccountStatus {
            &self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `NewUser` usado por el ejemplo para expresar el dominio del patron.
    pub struct NewUser {
        name: String,
        email: String,
    }

    impl NewUser {
        /// Crea una instancia valida para el ejemplo del patron.
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
    /// Conjunto de estados o errores publicos de `EntityError` dentro del ejemplo.
    pub enum EntityError {
        /// Variante `NameRequired` del estado o error del ejemplo.
        NameRequired,
        /// Variante `InvalidEmail` del estado o error del ejemplo.
        InvalidEmail,
    }

    impl EntityError {
        /// Modela la operacion `message` dentro del ejemplo del patron.
        pub fn message(&self) -> &'static str {
            match self {
                Self::NameRequired => "name is required",
                Self::InvalidEmail => "email is invalid",
            }
        }
    }
}

/// Modulo del ejemplo `gateways` dentro del catalogo de patrones.
pub mod gateways {
    use super::entities::{AccountStatus, NewUser, RegisteredUser};

    /// Contrato publico `UserGateway` que desacopla las piezas del ejemplo.
    pub trait UserGateway {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, new_user: NewUser) -> RegisteredUser;
        /// Operacion `all` definida por la abstraccion del ejemplo.
        fn all(&self) -> Vec<RegisteredUser>;
    }

    #[derive(Debug, Default, Clone)]
    /// Tipo publico `InMemoryUserGateway` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryUserGateway {
        users: Vec<RegisteredUser>,
        next_id: usize,
    }

    impl UserGateway for InMemoryUserGateway {
        /// Operacion `save` definida por la abstraccion del ejemplo.
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

        /// Operacion `all` definida por la abstraccion del ejemplo.
        fn all(&self) -> Vec<RegisteredUser> {
            self.users.clone()
        }
    }
}

/// Modulo del ejemplo `use_cases` dentro del catalogo de patrones.
pub mod use_cases {
    use super::entities::{EntityError, NewUser, RegisteredUser};
    use super::gateways::UserGateway;

    #[derive(Debug, Clone)]
    /// Tipo publico `RegisterUser` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterUser<G> {
        gateway: G,
    }

    impl<G> RegisterUser<G>
    where
        G: UserGateway,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(gateway: G) -> Self {
            Self { gateway }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(
            &mut self,
            name: impl Into<String>,
            email: impl Into<String>,
        ) -> Result<RegisteredUser, EntityError> {
            let new_user = NewUser::new(name, email)?;
            Ok(self.gateway.save(new_user))
        }

        /// Modela la operacion `saved users` dentro del ejemplo del patron.
        pub fn saved_users(&self) -> Vec<RegisteredUser> {
            self.gateway.all()
        }
    }
}

/// Modulo del ejemplo `controllers` dentro del catalogo de patrones.
pub mod controllers {
    use super::entities::RegisteredUser;
    use super::gateways::UserGateway;
    use super::use_cases::RegisterUser;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `RegisterUserRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterUserRequest {
        name: String,
        email: String,
    }

    impl RegisterUserRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `RegisteredUserResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisteredUserResponse {
        id: String,
        status: String,
    }

    impl RegisteredUserResponse {
        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `status` dentro del ejemplo del patron.
        pub fn status(&self) -> &str {
            &self.status
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `RegisterUserResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterUserResponse {
        status_code: u16,
        body: Option<RegisteredUserResponse>,
        error: Option<String>,
    }

    impl RegisterUserResponse {
        /// Operacion `created` definida por la abstraccion del ejemplo.
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

        /// Operacion `validation error` definida por la abstraccion del ejemplo.
        fn validation_error(message: &'static str) -> Self {
            Self {
                status_code: 422,
                body: None,
                error: Some(message.to_string()),
            }
        }

        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &RegisteredUserResponse {
            self.body
                .as_ref()
                .expect("created response should include a body")
        }

        /// Modela la operacion `error` dentro del ejemplo del patron.
        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `RegisterUserController` usado por el ejemplo para expresar el dominio del patron.
    pub struct RegisterUserController<G> {
        use_case: RegisterUser<G>,
    }

    impl<G> RegisterUserController<G>
    where
        G: UserGateway,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(use_case: RegisterUser<G>) -> Self {
            Self { use_case }
        }

        /// Procesa la entrada publica del ejemplo.
        pub fn handle(&mut self, request: RegisterUserRequest) -> RegisterUserResponse {
            match self.use_case.execute(request.name, request.email) {
                Ok(user) => RegisterUserResponse::created(user),
                Err(error) => RegisterUserResponse::validation_error(error.message()),
            }
        }

        /// Modela la operacion `saved users` dentro del ejemplo del patron.
        pub fn saved_users(&self) -> Vec<RegisteredUser> {
            self.use_case.saved_users()
        }
    }
}
