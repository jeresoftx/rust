use design_patterns_rust::patterns::gof::creational::prototype::deployment_configs::{
    DeploymentConfig, production_deployment, staging_deployment,
};

#[test]
fn prototype_clones_base_deployment_for_staging() {
    let base = DeploymentConfig::web_service_base();

    let staging = staging_deployment(&base);

    assert_eq!(
        staging.summary(),
        "service=web-api env=staging replicas=2 domain=staging.example.com features=logs,metrics"
    );
}

#[test]
fn prototype_clones_base_deployment_for_production() {
    let base = DeploymentConfig::web_service_base();

    let production = production_deployment(&base);

    assert_eq!(
        production.summary(),
        "service=web-api env=production replicas=6 domain=api.example.com features=logs,metrics,autoscaling"
    );
}

#[test]
fn prototype_keeps_base_deployment_unchanged() {
    let base = DeploymentConfig::web_service_base();

    let _production = production_deployment(&base);

    assert_eq!(
        base.summary(),
        "service=web-api env=base replicas=1 domain=localhost features=logs,metrics"
    );
}
