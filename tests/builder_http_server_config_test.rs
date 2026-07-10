use design_patterns_rust::patterns::gof::creational::builder::http_server_config::{
    ConfigError, HttpServerConfig,
};

#[test]
fn builder_creates_http_server_config_with_readable_options() {
    let config = HttpServerConfig::builder()
        .host("0.0.0.0")
        .port(8080)
        .enable_tls()
        .worker_threads(8)
        .build()
        .expect("valid config");

    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 8080);
    assert!(config.tls_enabled);
    assert_eq!(config.worker_threads, 8);
}

#[test]
fn builder_rejects_zero_worker_threads() {
    let error = HttpServerConfig::builder()
        .worker_threads(0)
        .build()
        .expect_err("zero workers should be rejected");

    assert_eq!(error, ConfigError::WorkerThreadsMustBePositive);
}
