use design_patterns_rust::patterns::rust_idiomatic::message_passing::job_worker::{Job, JobWorker};

#[test]
fn message_passing_worker_processes_jobs_from_channel() {
    let worker = JobWorker::start();

    worker.enqueue(Job::new("import-orders", 12));
    worker.enqueue(Job::new("recalculate-inventory", 7));
    worker.enqueue(Job::new("send-receipts", 3));

    let report = worker.finish();

    assert_eq!(
        report.processed_ids(),
        vec!["import-orders", "recalculate-inventory", "send-receipts"]
    );
    assert_eq!(report.total_units(), 22);
}

#[test]
fn message_passing_worker_finishes_when_sender_is_closed() {
    let worker = JobWorker::start();

    let report = worker.finish();

    assert!(report.processed_ids().is_empty());
    assert_eq!(report.total_units(), 0);
}

#[test]
fn message_passing_worker_accepts_jobs_from_cloned_producers() {
    let worker = JobWorker::start();
    let producer = worker.producer();

    producer.send(Job::new("sync-catalog", 5));
    worker.enqueue(Job::new("sync-prices", 8));
    drop(producer);

    let report = worker.finish();

    assert_eq!(report.processed_ids(), vec!["sync-catalog", "sync-prices"]);
    assert_eq!(report.total_units(), 13);
}
