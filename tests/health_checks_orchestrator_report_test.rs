use design_patterns_rust::patterns::distributed_systems::health_checks_readiness::orchestrator_report::{
    ComponentStatus, OrchestratorReport, OverallStatus,
};

#[test]
fn aggregate_report_is_ready_when_all_required_components_are_ready() {
    let report = OrchestratorReport::new(vec![
        ComponentStatus::required("database", true),
        ComponentStatus::required("broker", true),
        ComponentStatus::optional("metrics", false),
    ]);

    assert_eq!(report.overall_status(), OverallStatus::Ready);
}

#[test]
fn aggregate_report_is_not_ready_when_required_component_fails() {
    let report = OrchestratorReport::new(vec![
        ComponentStatus::required("database", true),
        ComponentStatus::required("broker", false),
    ]);

    assert_eq!(report.overall_status(), OverallStatus::NotReady);
    assert_eq!(
        report.failed_required_components(),
        vec!["broker".to_string()]
    );
}

#[test]
fn report_contains_component_count_for_orchestrators() {
    let report = OrchestratorReport::new(vec![
        ComponentStatus::required("database", true),
        ComponentStatus::optional("metrics", false),
    ]);

    assert_eq!(report.component_count(), 2);
}
