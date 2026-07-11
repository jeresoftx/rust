use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct ChatRoom {
    inboxes: BTreeMap<String, Vec<String>>,
}

impl ChatRoom {
    pub fn new() -> Self {
        Self {
            inboxes: BTreeMap::new(),
        }
    }

    pub fn join(&mut self, user: impl Into<String>) {
        self.inboxes.entry(user.into()).or_default();
    }

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

    pub fn inbox(&self, user: &str) -> Vec<String> {
        self.inboxes.get(user).cloned().unwrap_or_default()
    }
}
