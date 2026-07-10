#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    WorkerThreadsMustBePositive,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub worker_threads: usize,
}

impl HttpServerConfig {
    pub fn builder() -> HttpServerConfigBuilder {
        HttpServerConfigBuilder::default()
    }
}

pub struct HttpServerConfigBuilder {
    host: String,
    port: u16,
    tls_enabled: bool,
    worker_threads: usize,
}

impl Default for HttpServerConfigBuilder {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            tls_enabled: false,
            worker_threads: 4,
        }
    }
}

impl HttpServerConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn enable_tls(mut self) -> Self {
        self.tls_enabled = true;
        self
    }

    pub fn worker_threads(mut self, worker_threads: usize) -> Self {
        self.worker_threads = worker_threads;
        self
    }

    pub fn build(self) -> Result<HttpServerConfig, ConfigError> {
        if self.worker_threads == 0 {
            return Err(ConfigError::WorkerThreadsMustBePositive);
        }

        Ok(HttpServerConfig {
            host: self.host,
            port: self.port,
            tls_enabled: self.tls_enabled,
            worker_threads: self.worker_threads,
        })
    }
}
