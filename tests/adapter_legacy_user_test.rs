use design_patterns_rust::patterns::gof::structural::adapter::legacy_user::{
    LegacyUserRecord, adapt_legacy_user,
};

#[test]
fn adapter_converts_legacy_user_record_to_modern_profile() {
    let legacy = LegacyUserRecord::new("usr-7", "Ada Lovelace", "ada@example.com", "active");

    let profile = adapt_legacy_user(legacy).expect("legacy user should be valid");

    assert_eq!(
        profile.summary(),
        "id=usr-7 name=Ada Lovelace email=ada@example.com active=true"
    );
}

#[test]
fn adapter_rejects_legacy_user_without_valid_email() {
    let legacy = LegacyUserRecord::new("usr-8", "Grace Hopper", "invalid-email", "active");

    let error = adapt_legacy_user(legacy).expect_err("email should be rejected");

    assert_eq!(error, "legacy user email must contain @");
}
