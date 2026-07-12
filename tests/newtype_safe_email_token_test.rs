use design_patterns_rust::patterns::rust_idiomatic::newtype::safe_email_token::{
    AuthToken, EmailAddress, LoginSession,
};

#[test]
fn newtype_normalizes_and_validates_email() {
    let email = EmailAddress::new("ANA@Example.COM").unwrap();

    assert_eq!(email.as_str(), "ana@example.com");
}

#[test]
fn newtype_rejects_invalid_email() {
    assert_eq!(
        EmailAddress::new("not-an-email").unwrap_err(),
        "email must contain a local part and domain"
    );
}

#[test]
fn newtype_builds_authorization_header_from_token() {
    let token = AuthToken::new("tok_live_abcdef123456").unwrap();

    assert_eq!(token.redacted(), "tok_live_...3456");
    assert_eq!(token.authorization_header(), "Bearer tok_live_abcdef123456");
}

#[test]
fn newtype_rejects_short_tokens() {
    assert_eq!(
        AuthToken::new("short").unwrap_err(),
        "token must start with tok_ and contain at least 16 characters"
    );
}

#[test]
fn newtype_composes_valid_email_and_token_into_session() {
    let email = EmailAddress::new("ana@example.com").unwrap();
    let token = AuthToken::new("tok_live_abcdef123456").unwrap();
    let session = LoginSession::new(email, token);

    assert_eq!(session.principal(), "ana@example.com");
    assert_eq!(
        session.audit_label(),
        "ana@example.com using tok_live_...3456"
    );
}
