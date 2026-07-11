use design_patterns_rust::patterns::gof::behavioral::observer::metrics_logs::{
    PipelineEvent, TelemetryHub,
};

#[test]
fn observer_updates_metrics_and_logs_from_pipeline_events() {
    let mut hub = TelemetryHub::new();
    hub.subscribe_metrics();
    hub.subscribe_logs();

    hub.publish(PipelineEvent::started("import-users"));
    hub.publish(PipelineEvent::finished("import-users", 120));
    hub.publish(PipelineEvent::failed("sync-products", "timeout"));

    assert_eq!(hub.metrics().started_jobs, 1);
    assert_eq!(hub.metrics().finished_jobs, 1);
    assert_eq!(hub.metrics().failed_jobs, 1);
    assert_eq!(
        hub.logs(),
        vec![
            "started:import-users".to_string(),
            "finished:import-users:120ms".to_string(),
            "failed:sync-products:timeout".to_string(),
        ]
    );
}

#[test]
fn observer_only_runs_registered_telemetry_subscribers() {
    let mut hub = TelemetryHub::new();
    hub.subscribe_metrics();

    hub.publish(PipelineEvent::finished("daily-report", 80));

    assert_eq!(hub.metrics().finished_jobs, 1);
    assert_eq!(hub.logs(), Vec::<String>::new());
}
