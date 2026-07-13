use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
/// Tipo publico `ChatRoom` usado por el ejemplo para expresar el dominio del patron.
pub struct ChatRoom {
    inboxes: BTreeMap<String, Vec<String>>,
}

impl ChatRoom {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            inboxes: BTreeMap::new(),
        }
    }

    /// Modela la operacion `join` dentro del ejemplo del patron.
    pub fn join(&mut self, user: impl Into<String>) {
        self.inboxes.entry(user.into()).or_default();
    }

    /// Modela la operacion `broadcast` dentro del ejemplo del patron.
    pub fn broadcast(&mut self, from: &str, message: &str) {
        if !self.inboxes.contains_key(from) {
            return;
        }

        let delivered_message = format!("{from}: {message}");
        for (user, inbox) in &mut self.inboxes {
            if user != from {
                inbox.push(delivered_message.clone());
            }
        }
    }

    /// Modela la operacion `send private` dentro del ejemplo del patron.
    pub fn send_private(&mut self, from: &str, to: &str, message: &str) -> bool {
        if !self.inboxes.contains_key(from) {
            return false;
        }

        if let Some(inbox) = self.inboxes.get_mut(to) {
            inbox.push(format!("{from} -> you: {message}"));
            true
        } else {
            if let Some(sender_inbox) = self.inboxes.get_mut(from) {
                sender_inbox.push(format!("system: {to} is not in the room"));
            }
            false
        }
    }

    /// Modela la operacion `inbox` dentro del ejemplo del patron.
    pub fn inbox(&self, user: &str) -> Vec<String> {
        self.inboxes.get(user).cloned().unwrap_or_default()
    }
}
