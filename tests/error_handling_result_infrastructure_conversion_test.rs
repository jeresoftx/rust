use design_patterns_rust::patterns::rust_idiomatic::error_handling_result::infrastructure_conversion::{
    load_customer_profile, AppError, CustomerProfile, CustomerRepository,
};

#[test]
fn result_error_handling_converts_infrastructure_errors_to_application_errors() {
    let repository = CustomerRepository::unavailable();

    let error = load_customer_profile(&repository, "CUST-1").unwrap_err();

    assert_eq!(
        error,
        AppError::TemporarilyUnavailable {
            operation: "load customer profile".to_string()
        }
    );
    assert_eq!(
        error.message(),
        "load customer profile is temporarily unavailable"
    );
}

#[test]
fn result_error_handling_maps_missing_row_to_not_found() {
    let repository = CustomerRepository::with_profiles(vec![]);

    let error = load_customer_profile(&repository, "CUST-404").unwrap_err();

    assert_eq!(
        error,
        AppError::NotFound {
            entity: "customer".to_string(),
            id: "CUST-404".to_string()
        }
    );
    assert_eq!(error.message(), "customer CUST-404 was not found");
}

#[test]
fn result_error_handling_returns_profile_when_infrastructure_succeeds() {
    let repository = CustomerRepository::with_profiles(vec![CustomerProfile::new("CUST-1", "Ana")]);

    let profile = load_customer_profile(&repository, "CUST-1").expect("profile should exist");

    assert_eq!(profile.id(), "CUST-1");
    assert_eq!(profile.greeting(), "Hola, Ana");
}
