use design_patterns_rust::patterns::gof::creational::singleton::app_config::{
    AppConfig, initialize_config, shared_config,
};

#[test]
fn singleton_initializes_app_config_once() {
    let config = initialize_config(AppConfig::new("production", "https://api.example.com", 30));

    assert_eq!(
        config.summary(),
        "env=production api=https://api.example.com timeout=30s"
    );
}

#[test]
fn singleton_returns_the_same_app_config_instance() {
    let first = initialize_config(AppConfig::new("production", "https://api.example.com", 30));
    let second = shared_config();

    assert!(std::ptr::eq(first, second));
    assert_eq!(second.environment(), "production");
}
