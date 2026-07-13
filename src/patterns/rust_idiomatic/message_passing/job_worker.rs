use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Job` usado por el ejemplo para expresar el dominio del patron.
pub struct Job {
    id: String,
    units: u32,
}

impl Job {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, units: u32) -> Self {
        Self {
            id: id.into(),
            units,
        }
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `JobProducer` usado por el ejemplo para expresar el dominio del patron.
pub struct JobProducer {
    sender: Sender<Job>,
}

impl JobProducer {
    /// Simula el envio de la solicitud ya configurada.
    pub fn send(&self, job: Job) {
        self.sender.send(job).expect("job worker should be running");
    }
}

#[derive(Debug)]
/// Tipo publico `JobWorker` usado por el ejemplo para expresar el dominio del patron.
pub struct JobWorker {
    sender: Sender<Job>,
    handle: JoinHandle<JobReport>,
}

impl JobWorker {
    /// Modela la operacion `start` dentro del ejemplo del patron.
    pub fn start() -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let handle = thread::spawn(move || {
            let mut report = JobReport::default();

            for job in receiver {
                report.record(job);
            }

            report
        });

        Self { sender, handle }
    }

    /// Modela la operacion `enqueue` dentro del ejemplo del patron.
    pub fn enqueue(&self, job: Job) {
        self.sender.send(job).expect("job worker should be running");
    }

    /// Modela la operacion `producer` dentro del ejemplo del patron.
    pub fn producer(&self) -> JobProducer {
        JobProducer {
            sender: self.sender.clone(),
        }
    }

    /// Modela la operacion `finish` dentro del ejemplo del patron.
    pub fn finish(self) -> JobReport {
        drop(self.sender);
        self.handle.join().expect("job worker should finish")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Tipo publico `JobReport` usado por el ejemplo para expresar el dominio del patron.
pub struct JobReport {
    processed_ids: Vec<String>,
    total_units: u32,
}

impl JobReport {
    /// Operacion `record` definida por la abstraccion del ejemplo.
    fn record(&mut self, job: Job) {
        self.total_units += job.units;
        self.processed_ids.push(job.id);
    }

    /// Modela la operacion `processed ids` dentro del ejemplo del patron.
    pub fn processed_ids(&self) -> Vec<&str> {
        self.processed_ids.iter().map(String::as_str).collect()
    }

    /// Modela la operacion `total units` dentro del ejemplo del patron.
    pub fn total_units(&self) -> u32 {
        self.total_units
    }
}
