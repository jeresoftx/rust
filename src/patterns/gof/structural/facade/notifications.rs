pub struct EmailSender;
pub struct SmsSender;
pub struct PushSender;

impl EmailSender {
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("email user={user_id} body={message}")
    }
}

impl SmsSender {
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("sms user={user_id} body={message}")
    }
}

impl PushSender {
    fn send(&self, user_id: &str, message: &str) -> String {
        format!("push user={user_id} body={message}")
    }
}

pub struct NotificationFacade {
    email: EmailSender,
    sms: SmsSender,
    push: PushSender,
}

impl NotificationFacade {
    pub fn new(email: EmailSender, sms: SmsSender, push: PushSender) -> Self {
        Self { email, sms, push }
    }

    pub fn notify_user(&self, user_id: &str, message: &str) -> Vec<String> {
        vec![
            self.email.send(user_id, message),
            self.sms.send(user_id, message),
            self.push.send(user_id, message),
        ]
    }

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
