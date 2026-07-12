use design_patterns_rust::patterns::rust_idiomatic::error_handling_result::validation_modes::{
    Registration, RegistrationError, validate_registration_accumulated,
    validate_registration_fail_fast,
};

#[test]
fn result_error_handling_accumulates_validation_errors() {
    let registration = Registration::new("", "correo-sin-arroba", "123");

    let errors = validate_registration_accumulated(&registration).unwrap_err();

    assert_eq!(
        errors.messages(),
        vec![
            "name is required",
            "email must contain @",
            "password must have at least 8 characters"
        ]
    );
}

#[test]
fn result_error_handling_fail_fast_returns_first_error() {
    let registration = Registration::new("", "correo-sin-arroba", "123");

    let error = validate_registration_fail_fast(&registration).unwrap_err();

    assert_eq!(error, RegistrationError::NameRequired);
    assert_eq!(error.message(), "name is required");
}

#[test]
fn result_error_handling_accepts_valid_registration_in_both_modes() {
    let registration = Registration::new("Ana", "ana@example.com", "supersecret");

    assert!(validate_registration_fail_fast(&registration).is_ok());
    assert!(validate_registration_accumulated(&registration).is_ok());
}
