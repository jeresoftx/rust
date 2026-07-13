#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ProbeStatus` dentro del ejemplo.
pub enum ProbeStatus {
    /// Variante `Ok` del estado o error del ejemplo.
    Ok,
    /// Variante `Unavailable` del estado o error del ejemplo.
    Unavailable,
}

#[derive(Debug)]
/// Tipo publico `ServiceProbe` usado por el ejemplo para expresar el dominio del patron.
pub struct ServiceProbe {
    process_alive: bool,
    config_loaded: bool,
    database_connected: bool,
}

impl ServiceProbe {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(process_alive: bool, config_loaded: bool, database_connected: bool) -> Self {
        Self {
            process_alive,
            config_loaded,
            database_connected,
        }
    }

    /// Modela la operacion `liveness` dentro del ejemplo del patron.
    pub fn liveness(&self) -> ProbeStatus {
        if self.process_alive {
            ProbeStatus::Ok
        } else {
            ProbeStatus::Unavailable
        }
    }

    /// Modela la operacion `readiness` dentro del ejemplo del patron.
    pub fn readiness(&self) -> ProbeStatus {
        if self.process_alive && self.config_loaded && self.database_connected {
            ProbeStatus::Ok
        } else {
            ProbeStatus::Unavailable
        }
    }
}
