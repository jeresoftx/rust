use design_patterns_rust::patterns::gof::creational::factory_method::http_clients::{
    Environment, request_summary,
};

#[test]
fn factory_method_creates_local_http_client() {
    let summary = request_summary(Environment::Local, "/health");

    assert_eq!(
        summary,
        "GET http://localhost:3000/health timeout=1000ms retries=0"
    );
}

#[test]
fn factory_method_creates_staging_http_client() {
    let summary = request_summary(Environment::Staging, "/health");

    assert_eq!(
        summary,
        "GET https://staging.api.example.com/health timeout=3000ms retries=1"
    );
}

#[test]
fn factory_method_creates_production_http_client() {
    let summary = request_summary(Environment::Production, "/health");

    assert_eq!(
        summary,
        "GET https://api.example.com/health timeout=5000ms retries=3"
    );
}
