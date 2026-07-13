#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PipelineEvent` dentro del ejemplo.
pub enum PipelineEvent {
    /// Variante `Started` del estado o error del ejemplo.
    Started {
        /// Valor publico `job` asociado a la variante `Started`.
        job: String,
    },
    /// Variante `Finished` del estado o error del ejemplo.
    Finished {
        /// Valor publico `job` asociado a la variante `Finished`.
        job: String,
        /// Valor publico `duration_ms` asociado a la variante `Finished`.
        duration_ms: u32,
    },
    /// Variante `Failed` del estado o error del ejemplo.
    Failed {
        /// Valor publico `job` asociado a la variante `Failed`.
        job: String,
        /// Valor publico `reason` asociado a la variante `Failed`.
        reason: String,
    },
}

impl PipelineEvent {
    /// Modela la operacion `started` dentro del ejemplo del patron.
    pub fn started(job: impl Into<String>) -> Self {
        Self::Started { job: job.into() }
    }

    /// Modela la operacion `finished` dentro del ejemplo del patron.
    pub fn finished(job: impl Into<String>, duration_ms: u32) -> Self {
        Self::Finished {
            job: job.into(),
            duration_ms,
        }
    }

    /// Modela la operacion `failed` dentro del ejemplo del patron.
    pub fn failed(job: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Failed {
            job: job.into(),
            reason: reason.into(),
        }
    }

    /// Operacion `log line` definida por la abstraccion del ejemplo.
    fn log_line(&self) -> String {
        match self {
            Self::Started { job } => format!("started:{job}"),
            Self::Finished { job, duration_ms } => format!("finished:{job}:{duration_ms}ms"),
            Self::Failed { job, reason } => format!("failed:{job}:{reason}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `PipelineMetrics` usado por el ejemplo para expresar el dominio del patron.
pub struct PipelineMetrics {
    /// Campo publico `started_jobs` expuesto por el tipo del ejemplo.
    pub started_jobs: u32,
    /// Campo publico `finished_jobs` expuesto por el tipo del ejemplo.
    pub finished_jobs: u32,
    /// Campo publico `failed_jobs` expuesto por el tipo del ejemplo.
    pub failed_jobs: u32,
}

#[derive(Debug, Default, Clone)]
/// Tipo publico `TelemetryHub` usado por el ejemplo para expresar el dominio del patron.
pub struct TelemetryHub {
    metrics_enabled: bool,
    logs_enabled: bool,
    metrics: PipelineMetrics,
    logs: Vec<String>,
}

impl TelemetryHub {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            metrics_enabled: false,
            logs_enabled: false,
            metrics: PipelineMetrics::default(),
            logs: Vec::new(),
        }
    }

    /// Modela la operacion `subscribe metrics` dentro del ejemplo del patron.
    pub fn subscribe_metrics(&mut self) {
        self.metrics_enabled = true;
    }

    /// Modela la operacion `subscribe logs` dentro del ejemplo del patron.
    pub fn subscribe_logs(&mut self) {
        self.logs_enabled = true;
    }

    /// Modela la operacion `publish` dentro del ejemplo del patron.
    pub fn publish(&mut self, event: PipelineEvent) {
        if self.metrics_enabled {
            match &event {
                PipelineEvent::Started { .. } => self.metrics.started_jobs += 1,
                PipelineEvent::Finished { .. } => self.metrics.finished_jobs += 1,
                PipelineEvent::Failed { .. } => self.metrics.failed_jobs += 1,
            }
        }

        if self.logs_enabled {
            self.logs.push(event.log_line());
        }
    }

    /// Modela la operacion `metrics` dentro del ejemplo del patron.
    pub fn metrics(&self) -> PipelineMetrics {
        self.metrics.clone()
    }

    /// Modela la operacion `logs` dentro del ejemplo del patron.
    pub fn logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
