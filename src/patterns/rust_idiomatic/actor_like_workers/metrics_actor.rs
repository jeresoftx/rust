use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `MetricEvent` usado por el ejemplo para expresar el dominio del patron.
pub struct MetricEvent {
    name: String,
    value: u64,
}

impl MetricEvent {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `MetricTotal` usado por el ejemplo para expresar el dominio del patron.
pub struct MetricTotal {
    name: String,
    value: u64,
}

impl MetricTotal {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `value` dentro del ejemplo del patron.
    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `MetricsClient` usado por el ejemplo para expresar el dominio del patron.
pub struct MetricsClient {
    sender: Sender<MetricsCommand>,
}

impl MetricsClient {
    /// Modela la operacion `record` dentro del ejemplo del patron.
    pub fn record(&self, event: MetricEvent) {
        self.sender
            .send(MetricsCommand::Record(event))
            .expect("metrics actor should be running");
    }
}

#[derive(Debug)]
/// Tipo publico `MetricsActor` usado por el ejemplo para expresar el dominio del patron.
pub struct MetricsActor {
    sender: Sender<MetricsCommand>,
    handle: JoinHandle<()>,
}

impl MetricsActor {
    /// Modela la operacion `start` dentro del ejemplo del patron.
    pub fn start() -> Self {
        let (sender, receiver) = mpsc::channel::<MetricsCommand>();
        let handle = thread::spawn(move || {
            let mut totals = HashMap::<String, u64>::new();

            for command in receiver {
                match command {
                    MetricsCommand::Record(event) => {
                        *totals.entry(event.name).or_default() += event.value;
                    }
                    MetricsCommand::Total { name, reply } => {
                        reply
                            .send(totals.get(&name).copied().unwrap_or_default())
                            .expect("metric total reply should be received");
                    }
                    MetricsCommand::Snapshot { reply } => {
                        let mut snapshot = totals
                            .iter()
                            .map(|(name, value)| MetricTotal::new(name.clone(), *value))
                            .collect::<Vec<_>>();
                        snapshot.sort_by(|left, right| left.name.cmp(&right.name));

                        reply
                            .send(snapshot)
                            .expect("metric snapshot reply should be received");
                    }
                    MetricsCommand::Shutdown => break,
                }
            }
        });

        Self { sender, handle }
    }

    /// Modela la operacion `client` dentro del ejemplo del patron.
    pub fn client(&self) -> MetricsClient {
        MetricsClient {
            sender: self.sender.clone(),
        }
    }

    /// Modela la operacion `record` dentro del ejemplo del patron.
    pub fn record(&self, event: MetricEvent) {
        self.client().record(event);
    }

    /// Modela la operacion `total` dentro del ejemplo del patron.
    pub fn total(&self, name: &str) -> u64 {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(MetricsCommand::Total {
                name: name.to_string(),
                reply,
            })
            .expect("metrics actor should be running");

        response
            .recv()
            .expect("metrics actor should reply to total command")
    }

    /// Modela la operacion `snapshot` dentro del ejemplo del patron.
    pub fn snapshot(&self) -> Vec<MetricTotal> {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(MetricsCommand::Snapshot { reply })
            .expect("metrics actor should be running");

        response
            .recv()
            .expect("metrics actor should reply to snapshot command")
    }

    /// Modela la operacion `shutdown` dentro del ejemplo del patron.
    pub fn shutdown(self) {
        self.sender
            .send(MetricsCommand::Shutdown)
            .expect("metrics actor should receive shutdown command");
        self.handle
            .join()
            .expect("metrics actor should stop cleanly");
    }
}

#[derive(Debug)]
enum MetricsCommand {
    Record(MetricEvent),
    Total { name: String, reply: Sender<u64> },
    Snapshot { reply: Sender<Vec<MetricTotal>> },
    Shutdown,
}
