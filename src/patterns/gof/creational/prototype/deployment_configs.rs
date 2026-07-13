#[derive(Clone, Debug, PartialEq, Eq)]
/// Tipo publico `DeploymentConfig` usado por el ejemplo para expresar el dominio del patron.
pub struct DeploymentConfig {
    service: String,
    environment: String,
    replicas: u8,
    domain: String,
    features: Vec<String>,
}

impl DeploymentConfig {
    /// Modela la operacion `web service base` dentro del ejemplo del patron.
    pub fn web_service_base() -> Self {
        Self {
            service: "web-api".to_string(),
            environment: "base".to_string(),
            replicas: 1,
            domain: "localhost".to_string(),
            features: vec!["logs".to_string(), "metrics".to_string()],
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "service={} env={} replicas={} domain={} features={}",
            self.service,
            self.environment,
            self.replicas,
            self.domain,
            self.features.join(",")
        )
    }
}

/// Modela la operacion `staging deployment` dentro del ejemplo del patron.
pub fn staging_deployment(base: &DeploymentConfig) -> DeploymentConfig {
    let mut deployment = base.clone();
    deployment.environment = "staging".to_string();
    deployment.replicas = 2;
    deployment.domain = "staging.example.com".to_string();
    deployment
}

/// Modela la operacion `production deployment` dentro del ejemplo del patron.
pub fn production_deployment(base: &DeploymentConfig) -> DeploymentConfig {
    let mut deployment = base.clone();
    deployment.environment = "production".to_string();
    deployment.replicas = 6;
    deployment.domain = "api.example.com".to_string();
    deployment.features.push("autoscaling".to_string());
    deployment
}
