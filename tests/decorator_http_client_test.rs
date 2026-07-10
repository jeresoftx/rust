use design_patterns_rust::patterns::gof::structural::decorator::http_client::{
    BaseHttpClient, LoggingClient, RetryClient, TimeoutClient, request_summary,
};

#[test]
fn decorator_wraps_http_client_with_timeout_retry_and_logging() {
    let client = LoggingClient::new(RetryClient::new(
        TimeoutClient::new(BaseHttpClient::new("https://api.example.com"), 250),
        3,
    ));

    assert_eq!(
        request_summary(&client, "/users"),
        "log(request=GET /users response=retry(attempts=3 timeout=250ms response=GET https://api.example.com/users))"
    );
}

#[test]
fn decorator_keeps_base_client_usable_without_extra_layers() {
    let client = BaseHttpClient::new("https://api.example.com");

    assert_eq!(
        request_summary(&client, "/health"),
        "GET https://api.example.com/health"
    );
}
