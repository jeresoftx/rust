use design_patterns_rust::patterns::gof::behavioral::state::auth_machine::{
    AuthSession, AuthState,
};

#[test]
fn state_authenticates_after_password_and_second_factor() {
    let mut session = AuthSession::new("user-1", "secret", "123456");

    assert_eq!(session.state(), AuthState::Anonymous);

    session
        .submit_password("secret")
        .expect("password is valid");
    assert_eq!(session.state(), AuthState::PasswordAccepted);

    session
        .submit_second_factor("123456")
        .expect("second factor is valid");
    assert_eq!(session.state(), AuthState::Authenticated);
}

#[test]
fn state_locks_session_after_three_bad_passwords() {
    let mut session = AuthSession::new("user-2", "secret", "123456");

    assert_eq!(
        session.submit_password("bad").unwrap_err(),
        "invalid password"
    );
    assert_eq!(
        session.submit_password("bad").unwrap_err(),
        "invalid password"
    );
    assert_eq!(
        session.submit_password("bad").unwrap_err(),
        "session locked after too many attempts"
    );

    assert_eq!(session.state(), AuthState::Locked);
    assert_eq!(
        session.submit_password("secret").unwrap_err(),
        "locked sessions cannot authenticate"
    );
}

#[test]
fn state_rejects_second_factor_before_password() {
    let mut session = AuthSession::new("user-3", "secret", "123456");

    assert_eq!(
        session.submit_second_factor("123456").unwrap_err(),
        "password must be accepted before second factor"
    );
    assert_eq!(session.state(), AuthState::Anonymous);
}
