#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `AppConfig` usado por el ejemplo para expresar el dominio del patron.
pub struct AppConfig {
    /// Campo publico `environment` expuesto por el tipo del ejemplo.
    pub environment: String,
    /// Campo publico `timeout_seconds` expuesto por el tipo del ejemplo.
    pub timeout_seconds: u32,
    /// Campo publico `feature_enabled` expuesto por el tipo del ejemplo.
    pub feature_enabled: bool,
}

impl AppConfig {
    /// Crea una instancia valida para el ejemplo del patron.
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
/// Tipo publico `ConfigSnapshot` usado por el ejemplo para expresar el dominio del patron.
pub struct ConfigSnapshot {
    state: AppConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ConfigEditor` usado por el ejemplo para expresar el dominio del patron.
pub struct ConfigEditor {
    config: AppConfig,
}

impl ConfigEditor {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&self) -> ConfigSnapshot {
        ConfigSnapshot {
            state: self.config.clone(),
        }
    }

    /// Modela la operacion `restore` dentro del ejemplo del patron.
    pub fn restore(&mut self, snapshot: ConfigSnapshot) {
        self.config = snapshot.state;
    }

    /// Modela la operacion `set environment` dentro del ejemplo del patron.
    pub fn set_environment(&mut self, environment: impl Into<String>) {
        self.config.environment = environment.into();
    }

    /// Modela la operacion `set timeout seconds` dentro del ejemplo del patron.
    pub fn set_timeout_seconds(&mut self, timeout_seconds: u32) {
        self.config.timeout_seconds = timeout_seconds;
    }

    /// Modela la operacion `set feature enabled` dentro del ejemplo del patron.
    pub fn set_feature_enabled(&mut self, feature_enabled: bool) {
        self.config.feature_enabled = feature_enabled;
    }

    /// Modela la operacion `config` dentro del ejemplo del patron.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
