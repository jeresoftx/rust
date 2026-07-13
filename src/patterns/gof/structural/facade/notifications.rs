/// Tipo publico `EmailSender` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailSender;
/// Tipo publico `SmsSender` usado por el ejemplo para expresar el dominio del patron.
pub struct SmsSender;
/// Tipo publico `PushSender` usado por el ejemplo para expresar el dominio del patron.
pub struct PushSender;

impl EmailSender {
    /// Operacion `send` definida por la abstraccion del ejemplo.
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("email user={user_id} body={message}")
    }
}

impl SmsSender {
    /// Operacion `send` definida por la abstraccion del ejemplo.
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("sms user={user_id} body={message}")
    }
}

impl PushSender {
    /// Operacion `send` definida por la abstraccion del ejemplo.
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("push user={user_id} body={message}")
    }
}

/// Tipo publico `NotificationFacade` usado por el ejemplo para expresar el dominio del patron.
pub struct NotificationFacade {
    email: EmailSender,
    sms: SmsSender,
    push: PushSender,
}

impl NotificationFacade {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(email: EmailSender, sms: SmsSender, push: PushSender) -> Self {
        Self { email, sms, push }
    }

    /// Modela la operacion `notify user` dentro del ejemplo del patron.
    pub fn notify_user(&self, user_id: &str, message: &str) -> Vec<String> {
        vec![
            self.email.send(user_id, message),
            self.sms.send(user_id, message),
            self.push.send(user_id, message),
        ]
    }

    /// Modela la operacion `critical alert` dentro del ejemplo del patron.
    pub fn critical_alert(&self, user_id: &str, message: &str) -> String {
        let delivered = self
            .notify_user(user_id, message)
            .into_iter()
            .map(|entry| entry.split(' ').next().unwrap_or_default().to_string())
            .collect::<Vec<_>>()
            .join(",");

        format!("critical user={user_id} delivered={delivered} message={message}")
    }
}
