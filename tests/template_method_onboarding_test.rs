use design_patterns_rust::patterns::gof::behavioral::template_method::onboarding::{
    EnterpriseOnboarding, OnboardingTemplate, StarterOnboarding,
};

#[test]
fn template_method_runs_starter_onboarding_with_default_hooks() {
    let plan = StarterOnboarding::new("ana@example.com").onboard();

    assert_eq!(
        plan.steps(),
        [
            "validate:ana@example.com",
            "create_account:starter",
            "provision:workspace",
            "provision:billing_profile",
            "hook:send_welcome_email",
            "complete:ana@example.com"
        ]
    );
}

#[test]
fn template_method_runs_enterprise_onboarding_with_custom_hooks() {
    let plan = EnterpriseOnboarding::new("cto@example.com").onboard();

    assert_eq!(
        plan.steps(),
        [
            "validate:cto@example.com",
            "create_account:enterprise",
            "provision:workspace",
            "provision:billing_profile",
            "hook:assign_success_manager",
            "hook:enable_sso",
            "complete:cto@example.com"
        ]
    );
}

#[test]
fn template_method_stops_onboarding_when_validation_fails() {
    let plan = StarterOnboarding::new("invalid-email").onboard();

    assert_eq!(plan.steps(), ["validate:failed"]);
}
