use design_patterns_rust::patterns::gof::behavioral::chain_of_responsibility::request_validation::{
    Request, RequestValidationChain,
};

#[test]
fn chain_accepts_request_when_all_validation_handlers_pass() {
    let chain = RequestValidationChain::default();
    let request = Request::new(Some("token-123"), "admin", "create-order");

    let result = chain.validate(&request);

    assert_eq!(result, Ok("request accepted".to_string()));
}

#[test]
fn chain_stops_at_first_validation_error() {
    let chain = RequestValidationChain::default();
    let request = Request::new(None, "guest", "");

    let result = chain.validate(&request);

    assert_eq!(result, Err("missing authentication token".to_string()));
}
