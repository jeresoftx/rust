pub trait AppLogger {
    fn info(&self, event: &str, details: &str) -> String;
    fn warn(&self, event: &str, details: &str) -> String;
}

pub struct ThirdPartyLogger {
    component: String,
}

impl ThirdPartyLogger {
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
        }
    }

    pub fn write(&self, level_code: &str, event_name: &str, payload: &str) -> String {
        format!(
            "[{}] {} {} {}",
            self.component, level_code, event_name, payload
        )
    }
}

pub struct ThirdPartyLoggerAdapter {
    logger: ThirdPartyLogger,
}

impl ThirdPartyLoggerAdapter {
    pub fn new(logger: ThirdPartyLogger) -> Self {
        Self { logger }
    }
}

impl AppLogger for ThirdPartyLoggerAdapter {
    fn info(&self, event: &str, details: &str) -> String {
        self.logger.write("INFO", event, details)
    }

    fn warn(&self, event: &str, details: &str) -> String {
        self.logger.write("WARN", event, details)
    }
}

pub fn log_login_event(logger: &dyn AppLogger, user_id: &str) -> String {
    logger.info("user-login", &format!("user_id={user_id}"))
}
