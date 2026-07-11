use design_patterns_rust::patterns::gof::behavioral::memento::config_snapshots::{
    AppConfig, ConfigEditor,
};

#[test]
fn memento_restores_configuration_after_failed_change() {
    let initial = AppConfig::new("production", 30, true);
    let mut editor = ConfigEditor::new(initial.clone());

    let snapshot = editor.save();
    editor.set_timeout_seconds(0);
    editor.set_feature_enabled(false);

    assert_eq!(editor.config().timeout_seconds, 0);

    editor.restore(snapshot);

    assert_eq!(editor.config(), &initial);
}

#[test]
fn memento_keeps_snapshots_for_stepwise_rollbacks() {
    let mut editor = ConfigEditor::new(AppConfig::new("staging", 20, false));

    let staging = editor.save();
    editor.set_environment("production");
    let production = editor.save();
    editor.set_timeout_seconds(60);

    editor.restore(production);
    assert_eq!(editor.config(), &AppConfig::new("production", 20, false));

    editor.restore(staging);
    assert_eq!(editor.config(), &AppConfig::new("staging", 20, false));
}
