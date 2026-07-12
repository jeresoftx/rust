use design_patterns_rust::patterns::architecture::clean_architecture::registration_flow::{
    controllers::{RegisterUserController, RegisterUserRequest},
    entities::{AccountStatus, RegisteredUser},
    gateways::InMemoryUserGateway,
    use_cases::RegisterUser,
};

#[test]
fn clean_architecture_registration_flow_connects_controller_use_case_gateway_and_entity() {
    let gateway = InMemoryUserGateway::default();
    let use_case = RegisterUser::new(gateway);
    let mut controller = RegisterUserController::new(use_case);

    let response = controller.handle(RegisterUserRequest::new("Ana", "ana@example.com"));

    assert_eq!(response.status_code(), 201);
    assert_eq!(response.body().id(), "USR-1");
    assert_eq!(response.body().status(), "active");
    assert_eq!(
        controller.saved_users(),
        vec![RegisteredUser::new(
            "USR-1",
            "Ana",
            "ana@example.com",
            AccountStatus::Active,
        )]
    );
}

#[test]
fn clean_architecture_registration_flow_rejects_invalid_entity_input() {
    let gateway = InMemoryUserGateway::default();
    let use_case = RegisterUser::new(gateway);
    let mut controller = RegisterUserController::new(use_case);

    let response = controller.handle(RegisterUserRequest::new("", "correo-invalido"));

    assert_eq!(response.status_code(), 422);
    assert_eq!(response.error(), Some("name is required"));
    assert!(controller.saved_users().is_empty());
}

#[test]
fn clean_architecture_registration_flow_uses_gateway_to_assign_identity() {
    let gateway = InMemoryUserGateway::default();
    let use_case = RegisterUser::new(gateway);
    let mut controller = RegisterUserController::new(use_case);

    let first = controller.handle(RegisterUserRequest::new("Ana", "ana@example.com"));
    let second = controller.handle(RegisterUserRequest::new("Luis", "luis@example.com"));

    assert_eq!(first.body().id(), "USR-1");
    assert_eq!(second.body().id(), "USR-2");
    assert_eq!(controller.saved_users().len(), 2);
}
