use design_patterns_rust::patterns::rust_idiomatic::extension_trait::result_mapping::ResultDomainExt;

#[test]
fn extension_trait_keeps_successful_result_values() {
    let order_id = "42"
        .parse::<u64>()
        .map_domain("parse order id")
        .expect("valid id should parse");

    assert_eq!(order_id, 42);
}

#[test]
fn extension_trait_maps_low_level_error_to_domain_error() {
    let error = "abc"
        .parse::<u64>()
        .map_domain("parse order id")
        .unwrap_err();

    assert_eq!(error.operation(), "parse order id");
    assert_eq!(
        error.summary(),
        "parse order id failed: invalid digit found in string"
    );
}

#[test]
fn extension_trait_maps_error_with_custom_domain_message() {
    let result: Result<&str, &str> = Err("provider timeout");

    let error = result
        .map_domain_with("load customer profile", |source| {
            format!("customer data is unavailable because {source}")
        })
        .unwrap_err();

    assert_eq!(
        error.summary(),
        "load customer profile failed: customer data is unavailable because provider timeout"
    );
}
