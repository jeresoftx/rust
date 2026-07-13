use design_patterns_rust::patterns::rust_idiomatic::raii::lock_guard::CriticalSection;

#[test]
fn raii_releases_lock_when_guard_leaves_scope() {
    let section = CriticalSection::new("billing");

    {
        let mut guard = section.enter().expect("lock should be available");
        assert!(section.is_locked());
        assert!(section.enter().is_none());

        guard.record("charge customer");
    }

    assert!(!section.is_locked());
    assert_eq!(
        section.audit_log(),
        ["billing:charge customer", "billing:released"]
    );
}

#[test]
#[allow(clippy::needless_return)] // El ejemplo verifica que Drop libera el recurso durante una salida temprana.
fn raii_releases_lock_on_early_return() {
    fn process(section: &CriticalSection) {
        let mut guard = section.enter().expect("lock should be available");
        guard.record("validate invoice");

        if section.is_locked() {
            return;
        }
    }

    let section = CriticalSection::new("invoices");

    process(&section);

    assert!(!section.is_locked());
    assert_eq!(
        section.audit_log(),
        ["invoices:validate invoice", "invoices:released"]
    );
}

#[test]
fn raii_allows_reentering_after_previous_guard_drops() {
    let section = CriticalSection::new("inventory");

    {
        let mut guard = section.enter().expect("first lock should be available");
        guard.record("reserve stock");
    }

    {
        let mut guard = section.enter().expect("second lock should be available");
        guard.record("release stock");
    }

    assert_eq!(
        section.audit_log(),
        [
            "inventory:reserve stock",
            "inventory:released",
            "inventory:release stock",
            "inventory:released"
        ]
    );
}
