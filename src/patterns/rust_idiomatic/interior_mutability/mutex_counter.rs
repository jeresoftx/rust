use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricSnapshot {
    name: String,
    value: u64,
}

impl MetricSnapshot {
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug, Default)]
pub struct MetricsCounter {
    metrics: Mutex<HashMap<String, u64>>,
}

impl MetricsCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&self, name: &str) {
        self.increment_by(name, 1);
    }

    pub fn increment_by(&self, name: &str, amount: u64) {
        let mut metrics = self
            .metrics
            .lock()
            .expect("metrics counter mutex should not be poisoned");
        *metrics.entry(name.to_string()).or_default() += amount;
    }

    pub fn value(&self, name: &str) -> u64 {
        let metrics = self
            .metrics
            .lock()
            .expect("metrics counter mutex should not be poisoned");
        metrics.get(name).copied().unwrap_or_default()
    }

    pub fn snapshot(&self) -> Vec<MetricSnapshot> {
        let metrics = self
            .metrics
            .lock()
            .expect("metrics counter mutex should not be poisoned");
        let mut snapshot = metrics
            .iter()
            .map(|(name, value)| MetricSnapshot::new(name.clone(), *value))
            .collect::<Vec<_>>();

        snapshot.sort_by(|left, right| left.name.cmp(&right.name));
        snapshot
    }
}
