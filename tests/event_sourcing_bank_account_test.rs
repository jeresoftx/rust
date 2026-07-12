use design_patterns_rust::patterns::architecture::event_sourcing::bank_account::{
    Account, AccountEvent, AccountId, AccountState, Money,
};

#[test]
fn event_sourcing_bank_account_rebuilds_state_from_events() {
    let events = vec![
        AccountEvent::opened("ACC-1001"),
        AccountEvent::deposited(10_000),
        AccountEvent::withdrawn(3_500),
    ];

    let account = Account::rehydrate(AccountId::new("ACC-1001"), &events).unwrap();

    assert_eq!(account.id().as_str(), "ACC-1001");
    assert_eq!(account.state(), AccountState::Open);
    assert_eq!(account.balance(), Money::usd(6_500));
}

#[test]
fn event_sourcing_bank_account_records_new_events_after_commands() {
    let mut account = Account::rehydrate(
        AccountId::new("ACC-1002"),
        &[
            AccountEvent::opened("ACC-1002"),
            AccountEvent::deposited(5_000),
        ],
    )
    .unwrap();

    account.withdraw(Money::usd(1_250)).unwrap();

    assert_eq!(account.balance(), Money::usd(3_750));
    assert_eq!(
        account.pending_events(),
        vec![AccountEvent::withdrawn(1_250)]
    );
}

#[test]
fn event_sourcing_bank_account_rejects_overdraft_without_recording_event() {
    let mut account = Account::rehydrate(
        AccountId::new("ACC-1003"),
        &[
            AccountEvent::opened("ACC-1003"),
            AccountEvent::deposited(2_000),
        ],
    )
    .unwrap();

    let result = account.withdraw(Money::usd(3_000));

    assert!(result.is_err());
    assert_eq!(account.balance(), Money::usd(2_000));
    assert!(account.pending_events().is_empty());
}
