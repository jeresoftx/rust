use design_patterns_rust::patterns::gof::structural::proxy::cached_api::{
    CachedApiProxy, ExternalProductApi,
};

#[test]
fn proxy_caches_external_api_responses_by_product_id() {
    let api = ExternalProductApi::new();
    let mut proxy = CachedApiProxy::new(api);

    let first = proxy.product_name("p-100");
    let second = proxy.product_name("p-100");

    assert_eq!(first, "Product p-100");
    assert_eq!(second, "Product p-100");
    assert_eq!(proxy.real_api_call_count(), 1);
}

#[test]
fn proxy_calls_external_api_for_each_new_product_id() {
    let api = ExternalProductApi::new();
    let mut proxy = CachedApiProxy::new(api);

    assert_eq!(proxy.product_name("p-100"), "Product p-100");
    assert_eq!(proxy.product_name("p-200"), "Product p-200");

    assert_eq!(proxy.real_api_call_count(), 2);
}
