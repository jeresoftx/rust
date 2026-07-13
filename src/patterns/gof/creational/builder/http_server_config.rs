#[derive(Debug, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ConfigError` dentro del ejemplo.
pub enum ConfigError {
    /// Variante `WorkerThreadsMustBePositive` del estado o error del ejemplo.
    WorkerThreadsMustBePositive,
}

#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `HttpServerConfig` usado por el ejemplo para expresar el dominio del patron.
pub struct HttpServerConfig {
    /// Campo publico `host` expuesto por el tipo del ejemplo.
    pub host: String,
    /// Campo publico `port` expuesto por el tipo del ejemplo.
    pub port: u16,
    /// Campo publico `tls_enabled` expuesto por el tipo del ejemplo.
    pub tls_enabled: bool,
    /// Campo publico `worker_threads` expuesto por el tipo del ejemplo.
    pub worker_threads: usize,
}

impl HttpServerConfig {
    /// Modela la operacion `builder` dentro del ejemplo del patron.
    pub fn builder() -> HttpServerConfigBuilder {
        HttpServerConfigBuilder::default()
    }
}

/// Tipo publico `HttpServerConfigBuilder` usado por el ejemplo para expresar el dominio del patron.
pub struct HttpServerConfigBuilder {
    host: String,
    port: u16,
    tls_enabled: bool,
    worker_threads: usize,
}

impl Default for HttpServerConfigBuilder {
    /// Operacion `default` definida por la abstraccion del ejemplo.
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
    /// Modela la operacion `host` dentro del ejemplo del patron.
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Modela la operacion `port` dentro del ejemplo del patron.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Modela la operacion `enable tls` dentro del ejemplo del patron.
    pub fn enable_tls(mut self) -> Self {
        self.tls_enabled = true;
        self
    }

    /// Modela la operacion `worker threads` dentro del ejemplo del patron.
    pub fn worker_threads(mut self, worker_threads: usize) -> Self {
        self.worker_threads = worker_threads;
        self
    }

    /// Construye el valor final a partir de la configuracion acumulada.
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
