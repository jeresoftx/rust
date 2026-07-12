use design_patterns_rust::patterns::rust_idiomatic::typestate::request_builder::RequestBuilder;

#[test]
fn typestate_sends_request_after_required_url_is_set() {
    let response = RequestBuilder::new()
        .method("POST")
        .header("x-trace", "abc")
        .url("https://api.example.test/orders")
        .body("{\"id\":1}")
        .send();

    assert_eq!(response.status(), 202);
    assert_eq!(
        response.summary(),
        "POST https://api.example.test/orders with 1 headers and body"
    );
}

#[test]
fn typestate_uses_default_method_for_url_ready_request() {
    let response = RequestBuilder::new()
        .url("https://api.example.test/health")
        .send();

    assert_eq!(response.status(), 202);
    assert_eq!(
        response.summary(),
        "GET https://api.example.test/health with 0 headers and no body"
    );
}

#[test]
fn typestate_keeps_builder_data_when_transitioning_to_url_ready_state() {
    let response = RequestBuilder::new()
        .header("authorization", "Bearer token")
        .body("ping")
        .url("https://api.example.test/ping")
        .method("PUT")
        .send();

    assert_eq!(
        response.summary(),
        "PUT https://api.example.test/ping with 1 headers and body"
    );
}
