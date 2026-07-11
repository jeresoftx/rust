use design_patterns_rust::patterns::gof::behavioral::mediator::form_workflow::{
    FormMediator, RegistrationForm,
};

#[test]
fn mediator_validates_and_saves_a_registration_form() {
    let form = RegistrationForm::new("Ada Lovelace", "ada@example.com");
    let mut mediator = FormMediator::new();

    let receipt = mediator.submit(form).expect("valid form should be saved");

    assert_eq!(receipt.record_id, 1);
    assert_eq!(receipt.message, "registration saved");
    assert_eq!(
        mediator.events(),
        vec![
            "validation:passed".to_string(),
            "repository:saved:1".to_string(),
            "notification:registration saved".to_string(),
        ]
    );
}

#[test]
fn mediator_stops_before_saving_when_validation_fails() {
    let form = RegistrationForm::new("", "invalid-email");
    let mut mediator = FormMediator::new();

    let error = mediator.submit(form).unwrap_err();

    assert_eq!(error, "name and valid email are required");
    assert_eq!(mediator.saved_records(), 0);
    assert_eq!(mediator.events(), vec!["validation:failed".to_string()]);
}
