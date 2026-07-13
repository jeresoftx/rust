use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `MissingUrl` usado por el ejemplo para expresar el dominio del patron.
pub struct MissingUrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `ReadyToSend` usado por el ejemplo para expresar el dominio del patron.
pub struct ReadyToSend;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RequestBuilder` usado por el ejemplo para expresar el dominio del patron.
pub struct RequestBuilder<State> {
    method: String,
    headers: Vec<(String, String)>,
    url: Option<String>,
    body: Option<String>,
    state: PhantomData<State>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `FakeResponse` usado por el ejemplo para expresar el dominio del patron.
pub struct FakeResponse {
    status: u16,
    method: String,
    url: String,
    header_count: usize,
    has_body: bool,
}

impl RequestBuilder<MissingUrl> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            method: "GET".to_string(),
            headers: Vec::new(),
            url: None,
            body: None,
            state: PhantomData,
        }
    }

    /// Modela la operacion `url` dentro del ejemplo del patron.
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<ReadyToSend> {
        RequestBuilder {
            method: self.method,
            headers: self.headers,
            url: Some(url.into()),
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Default for RequestBuilder<MissingUrl> {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}

impl<State> RequestBuilder<State> {
    /// Modela la operacion `method` dentro del ejemplo del patron.
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    /// Modela la operacion `header` dentro del ejemplo del patron.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Modela la operacion `body` dentro del ejemplo del patron.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl RequestBuilder<ReadyToSend> {
    /// Simula el envio de la solicitud ya configurada.
    pub fn send(self) -> FakeResponse {
        FakeResponse {
            status: 202,
            method: self.method,
            url: self.url.expect("ReadyToSend siempre contiene URL"),
            header_count: self.headers.len(),
            has_body: self.body.is_some(),
        }
    }
}

impl FakeResponse {
    /// Modela la operacion `status` dentro del ejemplo del patron.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        let body = if self.has_body { "body" } else { "no body" };

        format!(
            "{} {} with {} headers and {}",
            self.method, self.url, self.header_count, body
        )
    }
}
