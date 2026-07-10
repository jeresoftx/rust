pub trait HttpClient {
    fn get(&self, path: &str) -> String;
}

pub struct BaseHttpClient {
    base_url: String,
}

impl BaseHttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}

impl HttpClient for BaseHttpClient {
    fn get(&self, path: &str) -> String {
        format!("GET {}{}", self.base_url, path)
    }
}

pub struct TimeoutClient<C> {
    inner: C,
    timeout_ms: u64,
}

impl<C> TimeoutClient<C> {
    pub fn new(inner: C, timeout_ms: u64) -> Self {
        Self { inner, timeout_ms }
    }
}

impl<C> HttpClient for TimeoutClient<C>
where
    C: HttpClient,
{
    fn get(&self, path: &str) -> String {
        format!(
            "timeout={}ms response={}",
            self.timeout_ms,
            self.inner.get(path)
        )
    }
}

pub struct RetryClient<C> {
    inner: C,
    attempts: u8,
}

impl<C> RetryClient<C> {
    pub fn new(inner: C, attempts: u8) -> Self {
        Self { inner, attempts }
    }
}

impl<C> HttpClient for RetryClient<C>
where
    C: HttpClient,
{
    fn get(&self, path: &str) -> String {
        format!("retry(attempts={} {})", self.attempts, self.inner.get(path))
    }
}

pub struct LoggingClient<C> {
    inner: C,
}

impl<C> LoggingClient<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C> HttpClient for LoggingClient<C>
where
    C: HttpClient,
{
    fn get(&self, path: &str) -> String {
        format!("log(request=GET {path} response={})", self.inner.get(path))
    }
}

pub fn request_summary(client: &dyn HttpClient, path: &str) -> String {
    client.get(path)
}
