#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `AuthState` dentro del ejemplo.
pub enum AuthState {
    /// Variante `Anonymous` del estado o error del ejemplo.
    Anonymous,
    /// Variante `PasswordAccepted` del estado o error del ejemplo.
    PasswordAccepted,
    /// Variante `Authenticated` del estado o error del ejemplo.
    Authenticated,
    /// Variante `Locked` del estado o error del ejemplo.
    Locked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `AuthSession` usado por el ejemplo para expresar el dominio del patron.
pub struct AuthSession {
    user_id: String,
    password: String,
    second_factor: String,
    failed_attempts: u8,
    state: AuthState,
}

impl AuthSession {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        user_id: impl Into<String>,
        password: impl Into<String>,
        second_factor: impl Into<String>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            password: password.into(),
            second_factor: second_factor.into(),
            failed_attempts: 0,
            state: AuthState::Anonymous,
        }
    }

    /// Modela la operacion `submit password` dentro del ejemplo del patron.
    pub fn submit_password(&mut self, password: &str) -> Result<(), String> {
        match self.state {
            AuthState::Anonymous => {
                if password == self.password {
                    self.failed_attempts = 0;
                    self.state = AuthState::PasswordAccepted;
                    Ok(())
                } else {
                    self.failed_attempts += 1;
                    if self.failed_attempts >= 3 {
                        self.state = AuthState::Locked;
                        Err("session locked after too many attempts".to_string())
                    } else {
                        Err("invalid password".to_string())
                    }
                }
            }
            AuthState::Locked => Err("locked sessions cannot authenticate".to_string()),
            _ => Err("password was already accepted".to_string()),
        }
    }

    /// Modela la operacion `submit second factor` dentro del ejemplo del patron.
    pub fn submit_second_factor(&mut self, code: &str) -> Result<(), String> {
        match self.state {
            AuthState::PasswordAccepted => {
                if code == self.second_factor {
                    self.state = AuthState::Authenticated;
                    Ok(())
                } else {
                    Err("invalid second factor".to_string())
                }
            }
            AuthState::Locked => Err("locked sessions cannot authenticate".to_string()),
            _ => Err("password must be accepted before second factor".to_string()),
        }
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> AuthState {
        self.state
    }

    /// Modela la operacion `user id` dentro del ejemplo del patron.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}
