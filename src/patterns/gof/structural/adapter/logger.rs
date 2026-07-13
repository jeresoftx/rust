/// Contrato publico `AppLogger` que desacopla las piezas del ejemplo.
pub trait AppLogger {
    /// Operacion `info` definida por la abstraccion del ejemplo.
    fn info(&self, event: &str, details: &str) -> String;
    /// Operacion `warn` definida por la abstraccion del ejemplo.
    fn warn(&self, event: &str, details: &str) -> String;
}

/// Tipo publico `ThirdPartyLogger` usado por el ejemplo para expresar el dominio del patron.
pub struct ThirdPartyLogger {
    component: String,
}

impl ThirdPartyLogger {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
        }
    }

    /// Modela la operacion `write` dentro del ejemplo del patron.
    pub fn write(&self, level_code: &str, event_name: &str, payload: &str) -> String {
        format!(
            "[{}] {} {} {}",
            self.component, level_code, event_name, payload
        )
    }
}

/// Tipo publico `ThirdPartyLoggerAdapter` usado por el ejemplo para expresar el dominio del patron.
pub struct ThirdPartyLoggerAdapter {
    logger: ThirdPartyLogger,
}

impl ThirdPartyLoggerAdapter {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(logger: ThirdPartyLogger) -> Self {
        Self { logger }
    }
}

impl AppLogger for ThirdPartyLoggerAdapter {
    /// Operacion `info` definida por la abstraccion del ejemplo.
    fn info(&self, event: &str, details: &str) -> String {
        self.logger.write("INFO", event, details)
    }

    /// Operacion `warn` definida por la abstraccion del ejemplo.
    fn warn(&self, event: &str, details: &str) -> String {
        self.logger.write("WARN", event, details)
    }
}

/// Modela la operacion `log login event` dentro del ejemplo del patron.
pub fn log_login_event(logger: &dyn AppLogger, user_id: &str) -> String {
    logger.info("user-login", &format!("user_id={user_id}"))
}
