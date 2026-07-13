/// Contrato publico `HttpClient` que desacopla las piezas del ejemplo.
pub trait HttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String;
}

/// Tipo publico `BaseHttpClient` usado por el ejemplo para expresar el dominio del patron.
pub struct BaseHttpClient {
    base_url: String,
}

impl BaseHttpClient {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}

impl HttpClient for BaseHttpClient {
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("GET {}{}", self.base_url, path)
    }
}

/// Tipo publico `TimeoutClient` usado por el ejemplo para expresar el dominio del patron.
pub struct TimeoutClient<C> {
    inner: C,
    timeout_ms: u64,
}

impl<C> TimeoutClient<C> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: C, timeout_ms: u64) -> Self {
        Self { inner, timeout_ms }
    }
}

impl<C> HttpClient for TimeoutClient<C>
where
    C: HttpClient,
{
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!(
            "timeout={}ms response={}",
            self.timeout_ms,
            self.inner.get(path)
        )
    }
}

/// Tipo publico `RetryClient` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryClient<C> {
    inner: C,
    attempts: u8,
}

impl<C> RetryClient<C> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: C, attempts: u8) -> Self {
        Self { inner, attempts }
    }
}

impl<C> HttpClient for RetryClient<C>
where
    C: HttpClient,
{
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("retry(attempts={} {})", self.attempts, self.inner.get(path))
    }
}

/// Tipo publico `LoggingClient` usado por el ejemplo para expresar el dominio del patron.
pub struct LoggingClient<C> {
    inner: C,
}

impl<C> LoggingClient<C> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C> HttpClient for LoggingClient<C>
where
    C: HttpClient,
{
    /// Operacion `get` definida por la abstraccion del ejemplo.
    fn get(&self, path: &str) -> String {
        format!("log(request=GET {path} response={})", self.inner.get(path))
    }
}

/// Modela la operacion `request summary` dentro del ejemplo del patron.
pub fn request_summary(client: &dyn HttpClient, path: &str) -> String {
    client.get(path)
}
