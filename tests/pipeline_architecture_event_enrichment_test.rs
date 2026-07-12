use design_patterns_rust::patterns::architecture::pipeline_architecture::event_enrichment::{
    EnrichedEvent, EventPipeline, RawEvent,
};

#[test]
fn pipeline_validates_and_enriches_events() {
    let pipeline = EventPipeline::new(vec!["admin".to_string(), "system".to_string()]);
    let events = vec![
        RawEvent::new("login.failed", "admin", "user tried invalid password", 2),
        RawEvent::new("invoice.paid", "system", "invoice ORD-1 paid", 1),
    ];

    let report = pipeline.process(events);

    assert_eq!(
        report.accepted(),
        vec![
            EnrichedEvent::new(
                "LOGIN.FAILED",
                "admin",
                "user tried invalid password",
                2,
                "security"
            ),
            EnrichedEvent::new("INVOICE.PAID", "system", "invoice ORD-1 paid", 1, "billing")
        ]
    );
    assert!(report.rejected().is_empty());
}

#[test]
fn pipeline_rejects_events_without_allowed_actor() {
    let pipeline = EventPipeline::new(vec!["admin".to_string()]);
    let events = vec![
        RawEvent::new("login.failed", "unknown", "bad actor", 5),
        RawEvent::new("login.failed", "", "missing actor", 5),
    ];

    let report = pipeline.process(events);

    assert!(report.accepted().is_empty());
    assert_eq!(
        report.rejected(),
        vec![
            "actor no permitido: unknown".to_string(),
            "actor requerido".to_string()
        ]
    );
}

#[test]
fn pipeline_filters_low_priority_noise_after_enrichment() {
    let pipeline = EventPipeline::new(vec!["system".to_string()]);
    let events = vec![
        RawEvent::new("metrics.heartbeat", "system", "ok", 0),
        RawEvent::new("invoice.failed", "system", "payment declined", 1),
    ];

    let report = pipeline.process(events);

    assert_eq!(
        report.accepted(),
        vec![EnrichedEvent::new(
            "INVOICE.FAILED",
            "system",
            "payment declined",
            1,
            "billing"
        )]
    );
    assert_eq!(
        report.rejected(),
        vec!["evento de baja prioridad".to_string()]
    );
}
