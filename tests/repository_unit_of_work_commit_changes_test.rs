use design_patterns_rust::patterns::architecture::repository_unit_of_work::commit_changes::{
    Customer, Order, UnitOfWork, UnitOfWorkStore,
};

#[test]
fn unit_of_work_keeps_changes_pending_until_commit() {
    let mut store = UnitOfWorkStore::default();
    let mut uow = UnitOfWork::new(&mut store);

    uow.customers()
        .save(Customer::new("CUS-1", "Ana", "ana@example.com"));
    uow.orders().save(Order::new("ORD-1", "CUS-1", 12_500));

    assert!(uow.customers().find("CUS-1").is_some());
    assert!(uow.orders().find("ORD-1").is_some());
    assert!(store.find_customer("CUS-1").is_none());
    assert!(store.find_order("ORD-1").is_none());
}

#[test]
fn unit_of_work_commits_multiple_repositories_together() {
    let mut store = UnitOfWorkStore::default();

    {
        let mut uow = UnitOfWork::new(&mut store);
        uow.customers()
            .save(Customer::new("CUS-2", "Luis", "luis@example.com"));
        uow.orders().save(Order::new("ORD-2", "CUS-2", 9_900));
        uow.commit();
    }

    assert_eq!(store.find_customer("CUS-2").unwrap().name(), "Luis");
    assert_eq!(store.find_order("ORD-2").unwrap().total_cents(), 9_900);
}

#[test]
fn unit_of_work_rollback_discards_pending_changes() {
    let mut store = UnitOfWorkStore::default();
    let mut uow = UnitOfWork::new(&mut store);

    uow.customers()
        .save(Customer::new("CUS-3", "Mia", "mia@example.com"));
    uow.orders().save(Order::new("ORD-3", "CUS-3", 2_500));
    uow.rollback();

    assert!(store.find_customer("CUS-3").is_none());
    assert!(store.find_order("ORD-3").is_none());
}
