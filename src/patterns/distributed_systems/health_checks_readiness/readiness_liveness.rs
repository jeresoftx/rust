#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeStatus {
    Ok,
    Unavailable,
}

#[derive(Debug)]
pub struct ServiceProbe {
    process_alive: bool,
    config_loaded: bool,
    database_connected: bool,
}

impl ServiceProbe {
    pub fn new(process_alive: bool, config_loaded: bool, database_connected: bool) -> Self {
        Self {
            process_alive,
            config_loaded,
            database_connected,
        }
    }

    pub fn liveness(&self) -> ProbeStatus {
        if self.process_alive {
            ProbeStatus::Ok
        } else {
            ProbeStatus::Unavailable
        }
    }

    pub fn readiness(&self) -> ProbeStatus {
        if self.process_alive && self.config_loaded && self.database_connected {
            ProbeStatus::Ok
        } else {
            ProbeStatus::Unavailable
        }
    }
}
