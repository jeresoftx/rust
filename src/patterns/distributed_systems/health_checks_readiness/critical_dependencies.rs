#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Dependency` usado por el ejemplo para expresar el dominio del patron.
pub struct Dependency {
    name: String,
    critical: bool,
    up: bool,
}

impl Dependency {
    /// Modela la operacion `critical` dentro del ejemplo del patron.
    pub fn critical(name: impl Into<String>, up: bool) -> Self {
        Self {
            name: name.into(),
            critical: true,
            up,
        }
    }

    /// Modela la operacion `optional` dentro del ejemplo del patron.
    pub fn optional(name: impl Into<String>, up: bool) -> Self {
        Self {
            name: name.into(),
            critical: false,
            up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `HealthStatus` dentro del ejemplo.
pub enum HealthStatus {
    /// Variante `Healthy` del estado o error del ejemplo.
    Healthy,
    /// Variante `Unhealthy` del estado o error del ejemplo.
    Unhealthy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `DependencyHealth` usado por el ejemplo para expresar el dominio del patron.
pub struct DependencyHealth {
    name: String,
    up: bool,
}

impl DependencyHealth {
    /// Modela la operacion `down` dentro del ejemplo del patron.
    pub fn down(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            up: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `HealthReport` usado por el ejemplo para expresar el dominio del patron.
pub struct HealthReport {
    /// Campo publico `status` expuesto por el tipo del ejemplo.
    pub status: HealthStatus,
    /// Campo publico `details` expuesto por el tipo del ejemplo.
    pub details: Vec<DependencyHealth>,
}

#[derive(Debug)]
/// Tipo publico `HealthChecker` usado por el ejemplo para expresar el dominio del patron.
pub struct HealthChecker {
    dependencies: Vec<Dependency>,
}

impl HealthChecker {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }

    /// Modela la operacion `check` dentro del ejemplo del patron.
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
