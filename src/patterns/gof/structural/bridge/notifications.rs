pub trait NotificationChannel {
    fn deliver(&self, title: &str, body: &str) -> String;
}

pub struct AlertNotification<C> {
    channel: C,
}

impl<C> AlertNotification<C>
where
    C: NotificationChannel,
{
    pub fn new(channel: C) -> Self {
        Self { channel }
    }

    pub fn send(&self, message: &str) -> String {
        self.channel.deliver("ALERT", message)
    }
}

pub struct EmailChannel {
    recipient: String,
}

impl EmailChannel {
    pub fn new(recipient: impl Into<String>) -> Self {
        Self {
            recipient: recipient.into(),
        }
    }
}

impl NotificationChannel for EmailChannel {
    fn deliver(&self, title: &str, body: &str) -> String {
        format!(
            "email to={} subject={} body={}",
            self.recipient, title, body
        )
    }
}

pub struct SmsChannel {
    phone_number: String,
}

impl SmsChannel {
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
        }
    }
}

impl NotificationChannel for SmsChannel {
    fn deliver(&self, title: &str, body: &str) -> String {
        format!("sms to={} message={}: {}", self.phone_number, title, body)
    }
}

pub struct PushChannel {
    device_id: String,
}

impl PushChannel {
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
        }
    }
}

impl NotificationChannel for PushChannel {
    fn deliver(&self, title: &str, body: &str) -> String {
        format!(
            "push device={} title={} body={}",
            self.device_id, title, body
        )
    }
}
