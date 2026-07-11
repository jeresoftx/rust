use design_patterns_rust::patterns::gof::behavioral::command::user_cli::{UserCli, UserCommand};

#[test]
fn command_creates_and_updates_user_from_cli_actions() {
    let mut cli = UserCli::new();

    cli.execute(UserCommand::Create {
        id: 7,
        email: "ana@example.com".to_string(),
    });
    cli.execute(UserCommand::UpdateEmail {
        id: 7,
        email: "ana@company.test".to_string(),
    });

    assert_eq!(cli.email_for(7), Some("ana@company.test".to_string()));
    assert_eq!(
        cli.audit_log(),
        vec![
            "created user 7 with ana@example.com",
            "updated user 7 to ana@company.test"
        ]
    );
}

#[test]
fn command_deletes_user_from_cli_action() {
    let mut cli = UserCli::new();

    cli.execute(UserCommand::Create {
        id: 3,
        email: "leo@example.com".to_string(),
    });
    cli.execute(UserCommand::Delete { id: 3 });

    assert_eq!(cli.email_for(3), None);
    assert_eq!(cli.audit_log().last(), Some(&"deleted user 3".to_string()));
}
