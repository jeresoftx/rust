/// Tipo asociado `ModerationFilter` producido por la abstraccion del ejemplo.
type ModerationFilter = fn(&UserMessage) -> Result<(), String>;

/// Tipo publico `UserMessage` usado por el ejemplo para expresar el dominio del patron.
pub struct UserMessage {
    body: String,
}

impl UserMessage {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

/// Tipo publico `ModerationChain` usado por el ejemplo para expresar el dominio del patron.
pub struct ModerationChain {
    filters: Vec<ModerationFilter>,
}

impl ModerationChain {
    /// Modela la operacion `review` dentro del ejemplo del patron.
    pub fn review(&self, message: &UserMessage) -> Result<String, String> {
        for filter in &self.filters {
            filter(message)?;
        }

        Ok("message approved".to_string())
    }
}

impl Default for ModerationChain {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self {
            filters: vec![reject_spam, reject_banned_words, reject_too_long],
        }
    }
}

/// Operacion `reject spam` definida por la abstraccion del ejemplo.
fn reject_spam(message: &UserMessage) -> Result<(), String> {
    if message.body.to_lowercase().contains("spam") {
        Err("message rejected as spam".to_string())
    } else {
        Ok(())
    }
}

/// Operacion `reject banned words` definida por la abstraccion del ejemplo.
fn reject_banned_words(message: &UserMessage) -> Result<(), String> {
    if message.body.to_lowercase().contains("forbidden") {
        Err("message contains banned words".to_string())
    } else {
        Ok(())
    }
}

/// Operacion `reject too long` definida por la abstraccion del ejemplo.
fn reject_too_long(message: &UserMessage) -> Result<(), String> {
    if message.body.len() > 120 {
        Err("message is too long".to_string())
    } else {
        Ok(())
    }
}
