use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricEvent {
    name: String,
    value: u64,
}

impl MetricEvent {
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricTotal {
    name: String,
    value: u64,
}

impl MetricTotal {
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

#[derive(Debug, Clone)]
pub struct MetricsClient {
    sender: Sender<MetricsCommand>,
}

impl MetricsClient {
    pub fn record(&self, event: MetricEvent) {
        self.sender
            .send(MetricsCommand::Record(event))
            .expect("metrics actor should be running");
    }
}

#[derive(Debug)]
pub struct MetricsActor {
    sender: Sender<MetricsCommand>,
    handle: JoinHandle<()>,
}

impl MetricsActor {
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

    pub fn client(&self) -> MetricsClient {
        MetricsClient {
            sender: self.sender.clone(),
        }
    }

    pub fn record(&self, event: MetricEvent) {
        self.client().record(event);
    }

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

    pub fn snapshot(&self) -> Vec<MetricTotal> {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(MetricsCommand::Snapshot { reply })
            .expect("metrics actor should be running");

        response
            .recv()
            .expect("metrics actor should reply to snapshot command")
    }

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
