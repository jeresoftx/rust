use std::collections::HashMap;

/// Conjunto de estados o errores publicos de `UserCommand` dentro del ejemplo.
pub enum UserCommand {
    /// Variante `Create` del estado o error del ejemplo.
    Create {
        /// Valor publico `id` asociado a la variante `Create`.
        id: u64,
        /// Valor publico `email` asociado a la variante `Create`.
        email: String,
    },
    /// Variante `UpdateEmail` del estado o error del ejemplo.
    UpdateEmail {
        /// Valor publico `id` asociado a la variante `UpdateEmail`.
        id: u64,
        /// Valor publico `email` asociado a la variante `UpdateEmail`.
        email: String,
    },
    /// Variante `Delete` del estado o error del ejemplo.
    Delete {
        /// Valor publico `id` asociado a la variante `Delete`.
        id: u64,
    },
}

/// Tipo publico `UserCli` usado por el ejemplo para expresar el dominio del patron.
pub struct UserCli {
    users: HashMap<u64, String>,
    audit_log: Vec<String>,
}

impl UserCli {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Ejecuta el caso de uso o comando del ejemplo.
    pub fn execute(&mut self, command: UserCommand) {
        match command {
            UserCommand::Create { id, email } => {
                self.users.insert(id, email.clone());
                self.audit_log
                    .push(format!("created user {id} with {email}"));
            }
            UserCommand::UpdateEmail { id, email } => {
                self.users.insert(id, email.clone());
                self.audit_log.push(format!("updated user {id} to {email}"));
            }
            UserCommand::Delete { id } => {
                self.users.remove(&id);
                self.audit_log.push(format!("deleted user {id}"));
            }
        }
    }

    /// Modela la operacion `email for` dentro del ejemplo del patron.
    pub fn email_for(&self, id: u64) -> Option<String> {
        self.users.get(&id).cloned()
    }

    /// Modela la operacion `audit log` dentro del ejemplo del patron.
    pub fn audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

impl Default for UserCli {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}
