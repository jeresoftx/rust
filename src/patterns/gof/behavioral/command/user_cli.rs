use std::collections::HashMap;

pub enum UserCommand {
    Create { id: u64, email: String },
    UpdateEmail { id: u64, email: String },
    Delete { id: u64 },
}

pub struct UserCli {
    users: HashMap<u64, String>,
    audit_log: Vec<String>,
}

impl UserCli {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

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

    pub fn email_for(&self, id: u64) -> Option<String> {
        self.users.get(&id).cloned()
    }

    pub fn audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

impl Default for UserCli {
    fn default() -> Self {
        Self::new()
    }
}
