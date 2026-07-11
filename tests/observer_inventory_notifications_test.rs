use design_patterns_rust::patterns::gof::behavioral::observer::inventory_notifications::{
    InventorySubject, StockChange,
};

#[test]
fn observer_notifies_inventory_subscribers_about_stock_changes() {
    let mut inventory = InventorySubject::new();
    inventory.subscribe_restock_alerts(5);
    inventory.subscribe_sales_updates();

    inventory.change_stock(StockChange::new("keyboard", 12));
    inventory.change_stock(StockChange::new("keyboard", 3));

    assert_eq!(
        inventory.restock_alerts(),
        vec!["restock-needed:keyboard:3".to_string()]
    );
    assert_eq!(
        inventory.sales_updates(),
        vec![
            "stock-updated:keyboard:12".to_string(),
            "stock-updated:keyboard:3".to_string(),
        ]
    );
}

#[test]
fn observer_does_not_run_unregistered_inventory_notifications() {
    let mut inventory = InventorySubject::new();
    inventory.subscribe_sales_updates();

    inventory.change_stock(StockChange::new("mouse", 2));

    assert_eq!(inventory.restock_alerts(), Vec::<String>::new());
    assert_eq!(
        inventory.sales_updates(),
        vec!["stock-updated:mouse:2".to_string()]
    );
}
