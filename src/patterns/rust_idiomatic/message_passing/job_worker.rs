use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    id: String,
    units: u32,
}

impl Job {
    pub fn new(id: impl Into<String>, units: u32) -> Self {
        Self {
            id: id.into(),
            units,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobProducer {
    sender: Sender<Job>,
}

impl JobProducer {
    pub fn send(&self, job: Job) {
        self.sender.send(job).expect("job worker should be running");
    }
}

#[derive(Debug)]
pub struct JobWorker {
    sender: Sender<Job>,
    handle: JoinHandle<JobReport>,
}

impl JobWorker {
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

    pub fn enqueue(&self, job: Job) {
        self.sender.send(job).expect("job worker should be running");
    }

    pub fn producer(&self) -> JobProducer {
        JobProducer {
            sender: self.sender.clone(),
        }
    }

    pub fn finish(self) -> JobReport {
        drop(self.sender);
        self.handle.join().expect("job worker should finish")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JobReport {
    processed_ids: Vec<String>,
    total_units: u32,
}

impl JobReport {
    fn record(&mut self, job: Job) {
        self.total_units += job.units;
        self.processed_ids.push(job.id);
    }

    pub fn processed_ids(&self) -> Vec<&str> {
        self.processed_ids.iter().map(String::as_str).collect()
    }

    pub fn total_units(&self) -> u32 {
        self.total_units
    }
}
