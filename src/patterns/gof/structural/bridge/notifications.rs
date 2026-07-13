/// Contrato publico `NotificationChannel` que desacopla las piezas del ejemplo.
pub trait NotificationChannel {
    /// Operacion `deliver` definida por la abstraccion del ejemplo.
    fn deliver(&self, title: &str, body: &str) -> String;
}

/// Tipo publico `AlertNotification` usado por el ejemplo para expresar el dominio del patron.
pub struct AlertNotification<C> {
    channel: C,
}

impl<C> AlertNotification<C>
where
    C: NotificationChannel,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(channel: C) -> Self {
        Self { channel }
    }

    /// Simula el envio de la solicitud ya configurada.
    pub fn send(&self, message: &str) -> String {
        self.channel.deliver("ALERT", message)
    }
}

/// Tipo publico `EmailChannel` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailChannel {
    recipient: String,
}

impl EmailChannel {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(recipient: impl Into<String>) -> Self {
        Self {
            recipient: recipient.into(),
        }
    }
}

impl NotificationChannel for EmailChannel {
    /// Operacion `deliver` definida por la abstraccion del ejemplo.
    fn deliver(&self, title: &str, body: &str) -> String {
        format!(
            "email to={} subject={} body={}",
            self.recipient, title, body
        )
    }
}

/// Tipo publico `SmsChannel` usado por el ejemplo para expresar el dominio del patron.
pub struct SmsChannel {
    phone_number: String,
}

impl SmsChannel {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
        }
    }
}

impl NotificationChannel for SmsChannel {
    /// Operacion `deliver` definida por la abstraccion del ejemplo.
    fn deliver(&self, title: &str, body: &str) -> String {
        format!("sms to={} message={}: {}", self.phone_number, title, body)
    }
}

/// Tipo publico `PushChannel` usado por el ejemplo para expresar el dominio del patron.
pub struct PushChannel {
    device_id: String,
}

impl PushChannel {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
        }
    }
}

impl NotificationChannel for PushChannel {
    /// Operacion `deliver` definida por la abstraccion del ejemplo.
    fn deliver(&self, title: &str, body: &str) -> String {
        format!(
            "push device={} title={} body={}",
            self.device_id, title, body
        )
    }
}
