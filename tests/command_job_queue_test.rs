use design_patterns_rust::patterns::gof::behavioral::command::job_queue::{JobCommand, JobQueue};

#[test]
fn command_queue_serializes_pending_jobs() {
    let mut queue = JobQueue::new();

    queue.enqueue(JobCommand::SendEmail {
        to: "ana@example.com".to_string(),
        template: "welcome".to_string(),
    });
    queue.enqueue(JobCommand::GenerateInvoice { order_id: 42 });

    assert_eq!(
        queue.serialize_pending(),
        vec!["email|ana@example.com|welcome", "invoice|42"]
    );
}

#[test]
fn command_queue_runs_jobs_in_order() {
    let mut queue = JobQueue::new();

    queue.enqueue(JobCommand::GenerateInvoice { order_id: 7 });
    queue.enqueue(JobCommand::SendEmail {
        to: "ops@example.com".to_string(),
        template: "invoice-ready".to_string(),
    });

    let log = queue.run_all();

    assert_eq!(
        log,
        vec![
            "generated invoice for order 7",
            "sent invoice-ready email to ops@example.com"
        ]
    );
    assert!(queue.serialize_pending().is_empty());
}
