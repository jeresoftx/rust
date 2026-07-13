use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static METRICS_REGISTRY: OnceLock<MetricsRegistry> = OnceLock::new();

/// Alias publico `MetricsRegistry` usado por el ejemplo.
pub type MetricsRegistry = Mutex<HashMap<String, u64>>;

/// Modela la operacion `metrics registry` dentro del ejemplo del patron.
pub fn metrics_registry() -> &'static MetricsRegistry {
    METRICS_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Modela la operacion `increment metric` dentro del ejemplo del patron.
pub fn increment_metric(name: &str) {
    let mut counters = metrics_registry()
        .lock()
        .expect("metrics registry lock must be available");

    *counters.entry(name.to_string()).or_insert(0) += 1;
}

/// Modela la operacion `read metric` dentro del ejemplo del patron.
pub fn read_metric(name: &str) -> u64 {
    let counters = metrics_registry()
        .lock()
        .expect("metrics registry lock must be available");

    counters.get(name).copied().unwrap_or_default()
}
