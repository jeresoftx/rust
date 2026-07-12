use design_patterns_rust::patterns::architecture::repository_unit_of_work::transaction_rollback::{
    InventoryStore, Product, TransactionError, TransactionalUnitOfWork,
};

#[test]
fn transaction_unit_of_work_commits_when_all_operations_succeed() {
    let mut store = InventoryStore::default();
    store.seed(Product::new("SKU-BOOK", 10));

    let result = TransactionalUnitOfWork::run(&mut store, |uow| {
        uow.reserve_stock("SKU-BOOK", 3)?;
        uow.create_product("SKU-PEN", 20)?;
        Ok(())
    });

    assert!(result.is_ok());
    assert_eq!(store.stock("SKU-BOOK"), Some(7));
    assert_eq!(store.stock("SKU-PEN"), Some(20));
}

#[test]
fn transaction_unit_of_work_rolls_back_when_operation_fails() {
    let mut store = InventoryStore::default();
    store.seed(Product::new("SKU-BOOK", 10));

    let result = TransactionalUnitOfWork::run(&mut store, |uow| {
        uow.reserve_stock("SKU-BOOK", 3)?;
        uow.reserve_stock("SKU-MISSING", 1)?;
        Ok(())
    });

    assert_eq!(result, Err(TransactionError::ProductNotFound));
    assert_eq!(store.stock("SKU-BOOK"), Some(10));
    assert_eq!(store.stock("SKU-MISSING"), None);
}

#[test]
fn transaction_unit_of_work_rolls_back_duplicate_creation() {
    let mut store = InventoryStore::default();
    store.seed(Product::new("SKU-BOOK", 10));

    let result = TransactionalUnitOfWork::run(&mut store, |uow| {
        uow.create_product("SKU-PEN", 20)?;
        uow.create_product("SKU-BOOK", 5)?;
        Ok(())
    });

    assert_eq!(result, Err(TransactionError::ProductAlreadyExists));
    assert_eq!(store.stock("SKU-PEN"), None);
    assert_eq!(store.stock("SKU-BOOK"), Some(10));
}
