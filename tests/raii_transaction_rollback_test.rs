use design_patterns_rust::patterns::rust_idiomatic::raii::transaction_rollback::Ledger;

#[test]
fn raii_rolls_back_uncommitted_transaction_on_scope_exit() {
    let ledger = Ledger::new();

    {
        let mut transaction = ledger.begin_transaction("tx-100");
        transaction.credit("cash", 500);
        transaction.debit("fees", 25);
    }

    assert_eq!(ledger.balance("cash"), 0);
    assert_eq!(ledger.balance("fees"), 0);
    assert_eq!(ledger.events(), ["tx-100:rollback 2 staged entries"]);
}

#[test]
fn raii_committed_transaction_applies_changes_without_rollback() {
    let ledger = Ledger::new();

    let receipt = {
        let mut transaction = ledger.begin_transaction("tx-200");
        transaction.credit("cash", 100);
        transaction.debit("fees", 20);
        transaction.commit()
    };

    assert_eq!(receipt.summary(), "tx-200 committed 2 entries");
    assert_eq!(ledger.balance("cash"), 100);
    assert_eq!(ledger.balance("fees"), -20);
    assert_eq!(ledger.events(), ["tx-200:commit 2 entries"]);
}

#[test]
fn raii_rolls_back_transaction_on_early_return() {
    fn import_payment(ledger: &Ledger) {
        let mut transaction = ledger.begin_transaction("tx-300");
        transaction.credit("cash", 75);

        return;
    }

    let ledger = Ledger::new();

    import_payment(&ledger);

    assert_eq!(ledger.balance("cash"), 0);
    assert_eq!(ledger.events(), ["tx-300:rollback 1 staged entries"]);
}
