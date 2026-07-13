use std::sync::OnceLock;

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `AppConfig` usado por el ejemplo para expresar el dominio del patron.
pub struct AppConfig {
    environment: String,
    api_base_url: String,
    request_timeout_seconds: u64,
}

impl AppConfig {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        environment: impl Into<String>,
        api_base_url: impl Into<String>,
        request_timeout_seconds: u64,
    ) -> Self {
        Self {
            environment: environment.into(),
            api_base_url: api_base_url.into(),
            request_timeout_seconds,
        }
    }

    /// Modela la operacion `environment` dentro del ejemplo del patron.
    pub fn environment(&self) -> &str {
        &self.environment
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "env={} api={} timeout={}s",
            self.environment, self.api_base_url, self.request_timeout_seconds
        )
    }
}

/// Modela la operacion `initialize config` dentro del ejemplo del patron.
pub fn initialize_config(config: AppConfig) -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| config)
}

/// Modela la operacion `shared config` dentro del ejemplo del patron.
pub fn shared_config() -> &'static AppConfig {
    APP_CONFIG
        .get()
        .expect("app config must be initialized first")
}
