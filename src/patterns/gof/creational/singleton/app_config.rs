use std::sync::OnceLock;

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, PartialEq, Eq)]
pub struct AppConfig {
    environment: String,
    api_base_url: String,
    request_timeout_seconds: u64,
}

impl AppConfig {
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

    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn summary(&self) -> String {
        format!(
            "env={} api={} timeout={}s",
            self.environment, self.api_base_url, self.request_timeout_seconds
        )
    }
}

pub fn initialize_config(config: AppConfig) -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| config)
}

pub fn shared_config() -> &'static AppConfig {
    APP_CONFIG
        .get()
        .expect("app config must be initialized first")
}
