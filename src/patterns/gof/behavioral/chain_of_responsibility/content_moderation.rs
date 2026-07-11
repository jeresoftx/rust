pub struct UserMessage {
    body: String,
}

impl UserMessage {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

pub struct ModerationChain {
    filters: Vec<fn(&UserMessage) -> Result<(), String>>,
}

impl ModerationChain {
    pub fn review(&self, message: &UserMessage) -> Result<String, String> {
        for filter in &self.filters {
            filter(message)?;
        }

        Ok("message approved".to_string())
    }
}

impl Default for ModerationChain {
    fn default() -> Self {
        Self {
            filters: vec![reject_spam, reject_banned_words, reject_too_long],
        }
    }
}

fn reject_spam(message: &UserMessage) -> Result<(), String> {
    if message.body.to_lowercase().contains("spam") {
        Err("message rejected as spam".to_string())
    } else {
        Ok(())
    }
}

fn reject_banned_words(message: &UserMessage) -> Result<(), String> {
    if message.body.to_lowercase().contains("forbidden") {
        Err("message contains banned words".to_string())
    } else {
        Ok(())
    }
}

fn reject_too_long(message: &UserMessage) -> Result<(), String> {
    if message.body.len() > 120 {
        Err("message is too long".to_string())
    } else {
        Ok(())
    }
}
