#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    name: String,
    critical: bool,
    up: bool,
}

impl Dependency {
    pub fn critical(name: impl Into<String>, up: bool) -> Self {
        Self {
            name: name.into(),
            critical: true,
            up,
        }
    }

    pub fn optional(name: impl Into<String>, up: bool) -> Self {
        Self {
            name: name.into(),
            critical: false,
            up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyHealth {
    name: String,
    up: bool,
}

impl DependencyHealth {
    pub fn down(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            up: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub details: Vec<DependencyHealth>,
}

#[derive(Debug)]
pub struct HealthChecker {
    dependencies: Vec<Dependency>,
}

impl HealthChecker {
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }

    pub fn check(&self) -> HealthReport {
        let unhealthy_critical = self
            .dependencies
            .iter()
            .any(|dependency| dependency.critical && !dependency.up);

        HealthReport {
            status: if unhealthy_critical {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Healthy
            },
            details: self
                .dependencies
                .iter()
                .map(|dependency| DependencyHealth {
                    name: dependency.name.clone(),
                    up: dependency.up,
                })
                .collect(),
        }
    }
}
