use design_patterns_rust::patterns::distributed_systems::health_checks_readiness::readiness_liveness::{
    ProbeStatus, ServiceProbe,
};

#[test]
fn process_can_be_alive_but_not_ready() {
    let probe = ServiceProbe::new(true, false, false);

    assert_eq!(probe.liveness(), ProbeStatus::Ok);
    assert_eq!(probe.readiness(), ProbeStatus::Unavailable);
}

#[test]
fn process_not_alive_is_not_ready() {
    let probe = ServiceProbe::new(false, true, true);

    assert_eq!(probe.liveness(), ProbeStatus::Unavailable);
    assert_eq!(probe.readiness(), ProbeStatus::Unavailable);
}

#[test]
fn ready_requires_started_process_and_loaded_config_and_database() {
    let probe = ServiceProbe::new(true, true, true);

    assert_eq!(probe.liveness(), ProbeStatus::Ok);
    assert_eq!(probe.readiness(), ProbeStatus::Ok);
}
