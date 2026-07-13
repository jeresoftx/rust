#[derive(Debug, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `EmailBuildError` dentro del ejemplo.
pub enum EmailBuildError {
    /// Variante `RecipientRequired` del estado o error del ejemplo.
    RecipientRequired,
    /// Variante `SubjectRequired` del estado o error del ejemplo.
    SubjectRequired,
    /// Variante `BodyRequired` del estado o error del ejemplo.
    BodyRequired,
}

#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `EmailPayload` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailPayload {
    to: String,
    cc: Vec<String>,
    subject: String,
    body: String,
    headers: Vec<(String, String)>,
}

impl EmailPayload {
    /// Modela la operacion `builder` dentro del ejemplo del patron.
    pub fn builder() -> EmailPayloadBuilder {
        EmailPayloadBuilder::default()
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        let cc = if self.cc.is_empty() {
            "none".to_string()
        } else {
            self.cc.join(",")
        };

        let headers = if self.headers.is_empty() {
            "none".to_string()
        } else {
            self.headers
                .iter()
                .map(|(key, value)| format!("{key}:{value}"))
                .collect::<Vec<_>>()
                .join(",")
        };

        format!(
            "to={} | cc={} | subject={} | headers={}",
            self.to, cc, self.subject, headers
        )
    }
}

#[derive(Default)]
/// Tipo publico `EmailPayloadBuilder` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailPayloadBuilder {
    to: Option<String>,
    cc: Vec<String>,
    subject: Option<String>,
    body: Option<String>,
    headers: Vec<(String, String)>,
}

impl EmailPayloadBuilder {
    /// Modela la operacion `to` dentro del ejemplo del patron.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Modela la operacion `cc` dentro del ejemplo del patron.
    pub fn cc(mut self, cc: impl Into<String>) -> Self {
        self.cc.push(cc.into());
        self
    }

    /// Modela la operacion `subject` dentro del ejemplo del patron.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Modela la operacion `body` dentro del ejemplo del patron.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Modela la operacion `header` dentro del ejemplo del patron.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Construye el valor final a partir de la configuracion acumulada.
    pub fn build(self) -> Result<EmailPayload, EmailBuildError> {
        let to = required(self.to, EmailBuildError::RecipientRequired)?;
        let subject = required(self.subject, EmailBuildError::SubjectRequired)?;
        let body = required(self.body, EmailBuildError::BodyRequired)?;

        Ok(EmailPayload {
            to,
            cc: self.cc,
            subject,
            body,
            headers: self.headers,
        })
    }
}

/// Operacion `required` definida por la abstraccion del ejemplo.
fn required(value: Option<String>, error: EmailBuildError) -> Result<String, EmailBuildError> {
    match value.map(|value| value.trim().to_string()) {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Err(error),
    }
}
