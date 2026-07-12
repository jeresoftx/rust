use design_patterns_rust::patterns::architecture::service_layer::user_registration::{
    EmailGateway, InMemoryUserRepository, RegistrationError, RegistrationRequest,
    RegistrationService, User,
};

#[test]
fn service_layer_registers_user_and_sends_welcome_email() {
    let repository = InMemoryUserRepository::default();
    let email_gateway = EmailGateway::default();
    let mut service = RegistrationService::new(repository, email_gateway);

    let response = service
        .register(RegistrationRequest::new("USR-1", "Ana", "ana@example.com"))
        .expect("valid registration should succeed");

    assert_eq!(response.user_id(), "USR-1");
    assert_eq!(
        service.find_user("USR-1"),
        Some(User::new("USR-1", "Ana", "ana@example.com"))
    );
    assert_eq!(service.sent_emails(), vec!["welcome:ana@example.com"]);
}

#[test]
fn service_layer_rejects_duplicate_user_ids() {
    let repository = InMemoryUserRepository::default();
    let email_gateway = EmailGateway::default();
    let mut service = RegistrationService::new(repository, email_gateway);

    service
        .register(RegistrationRequest::new("USR-1", "Ana", "ana@example.com"))
        .expect("first registration should succeed");

    let result = service.register(RegistrationRequest::new(
        "USR-1",
        "Ana duplicada",
        "ana2@example.com",
    ));

    assert_eq!(result, Err(RegistrationError::UserAlreadyExists));
    assert_eq!(service.sent_emails(), vec!["welcome:ana@example.com"]);
}

#[test]
fn service_layer_validates_email_before_persisting() {
    let repository = InMemoryUserRepository::default();
    let email_gateway = EmailGateway::default();
    let mut service = RegistrationService::new(repository, email_gateway);

    let result = service.register(RegistrationRequest::new("USR-1", "Ana", "correo-invalido"));

    assert_eq!(result, Err(RegistrationError::InvalidEmail));
    assert_eq!(service.find_user("USR-1"), None);
    assert!(service.sent_emails().is_empty());
}
