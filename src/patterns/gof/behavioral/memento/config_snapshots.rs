#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub environment: String,
    pub timeout_seconds: u32,
    pub feature_enabled: bool,
}

impl AppConfig {
    pub fn new(
        environment: impl Into<String>,
        timeout_seconds: u32,
        feature_enabled: bool,
    ) -> Self {
        Self {
            environment: environment.into(),
            timeout_seconds,
            feature_enabled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSnapshot {
    state: AppConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEditor {
    config: AppConfig,
}

impl ConfigEditor {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn save(&self) -> ConfigSnapshot {
        ConfigSnapshot {
            state: self.config.clone(),
        }
    }

    pub fn restore(&mut self, snapshot: ConfigSnapshot) {
        self.config = snapshot.state;
    }

    pub fn set_environment(&mut self, environment: impl Into<String>) {
        self.config.environment = environment.into();
    }

    pub fn set_timeout_seconds(&mut self, timeout_seconds: u32) {
        self.config.timeout_seconds = timeout_seconds;
    }

    pub fn set_feature_enabled(&mut self, feature_enabled: bool) {
        self.config.feature_enabled = feature_enabled;
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
