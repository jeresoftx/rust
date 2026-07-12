use design_patterns_rust::patterns::distributed_systems::health_checks_readiness::critical_dependencies::{
    Dependency, DependencyHealth, HealthChecker, HealthStatus,
};

#[test]
fn reports_healthy_when_all_critical_dependencies_are_up() {
    let checker = HealthChecker::new(vec![
        Dependency::critical("database", true),
        Dependency::critical("broker", true),
    ]);

    let report = checker.check();

    assert_eq!(report.status, HealthStatus::Healthy);
    assert_eq!(report.details.len(), 2);
}

#[test]
fn reports_unhealthy_when_a_critical_dependency_is_down() {
    let checker = HealthChecker::new(vec![
        Dependency::critical("database", true),
        Dependency::critical("broker", false),
    ]);

    let report = checker.check();

    assert_eq!(report.status, HealthStatus::Unhealthy);
    assert!(report.details.contains(&DependencyHealth::down("broker")));
}

#[test]
fn non_critical_dependency_does_not_make_report_unhealthy() {
    let checker = HealthChecker::new(vec![
        Dependency::critical("database", true),
        Dependency::optional("metrics", false),
    ]);

    assert_eq!(checker.check().status, HealthStatus::Healthy);
}
