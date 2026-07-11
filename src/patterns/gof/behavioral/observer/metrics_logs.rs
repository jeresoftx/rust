#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineEvent {
    Started { job: String },
    Finished { job: String, duration_ms: u32 },
    Failed { job: String, reason: String },
}

impl PipelineEvent {
    pub fn started(job: impl Into<String>) -> Self {
        Self::Started { job: job.into() }
    }

    pub fn finished(job: impl Into<String>, duration_ms: u32) -> Self {
        Self::Finished {
            job: job.into(),
            duration_ms,
        }
    }

    pub fn failed(job: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Failed {
            job: job.into(),
            reason: reason.into(),
        }
    }

    fn log_line(&self) -> String {
        match self {
            Self::Started { job } => format!("started:{job}"),
            Self::Finished { job, duration_ms } => format!("finished:{job}:{duration_ms}ms"),
            Self::Failed { job, reason } => format!("failed:{job}:{reason}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PipelineMetrics {
    pub started_jobs: u32,
    pub finished_jobs: u32,
    pub failed_jobs: u32,
}

#[derive(Debug, Default, Clone)]
pub struct TelemetryHub {
    metrics_enabled: bool,
    logs_enabled: bool,
    metrics: PipelineMetrics,
    logs: Vec<String>,
}

impl TelemetryHub {
    pub fn new() -> Self {
        Self {
            metrics_enabled: false,
            logs_enabled: false,
            metrics: PipelineMetrics::default(),
            logs: Vec::new(),
        }
    }

    pub fn subscribe_metrics(&mut self) {
        self.metrics_enabled = true;
    }

    pub fn subscribe_logs(&mut self) {
        self.logs_enabled = true;
    }

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

    pub fn metrics(&self) -> PipelineMetrics {
        self.metrics.clone()
    }

    pub fn logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
