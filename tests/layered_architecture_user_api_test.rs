use design_patterns_rust::patterns::architecture::layered_architecture::user_api::{
    application::RegisterUser,
    infrastructure::InMemoryUserRepository,
    presentation::{CreateUserRequest, UserController},
};

#[test]
fn layered_architecture_user_api_creates_user_through_all_layers() {
    let repository = InMemoryUserRepository::default();
    let use_case = RegisterUser::new(repository);
    let mut controller = UserController::new(use_case);

    let response = controller.create_user(CreateUserRequest::new("Ana", "ana@example.com"));

    assert_eq!(response.status_code(), 201);
    assert_eq!(response.body().id(), "USR-1");
    assert_eq!(response.body().name(), "Ana");
    assert_eq!(response.body().email(), "ana@example.com");
    assert_eq!(controller.total_users(), 1);
}

#[test]
fn layered_architecture_user_api_maps_domain_validation_to_bad_request() {
    let repository = InMemoryUserRepository::default();
    let use_case = RegisterUser::new(repository);
    let mut controller = UserController::new(use_case);

    let response = controller.create_user(CreateUserRequest::new("Ana", "correo-invalido"));

    assert_eq!(response.status_code(), 400);
    assert_eq!(response.error(), Some("email must contain @"));
    assert_eq!(controller.total_users(), 0);
}

#[test]
fn layered_architecture_user_api_keeps_presentation_response_separate_from_domain() {
    let repository = InMemoryUserRepository::default();
    let use_case = RegisterUser::new(repository);
    let mut controller = UserController::new(use_case);

    let response = controller.create_user(CreateUserRequest::new("Luis", "luis@example.com"));

    assert_eq!(response.body().display_label(), "Luis <luis@example.com>");
}
