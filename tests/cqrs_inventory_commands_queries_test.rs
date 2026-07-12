use design_patterns_rust::patterns::architecture::cqrs::inventory_commands_queries::{
    GetInventoryQuery, InventoryCommandHandler, InventoryQueryHandler, InventoryWriteModel,
    ReceiveStockCommand, ReserveStockCommand, StockLevel,
};

#[test]
fn cqrs_inventory_commands_change_write_model() {
    let mut write_model = InventoryWriteModel::default();
    let mut commands = InventoryCommandHandler::new(&mut write_model);

    commands
        .handle_receive(ReceiveStockCommand::new("SKU-BOOK", 10))
        .unwrap();
    commands
        .handle_reserve(ReserveStockCommand::new("SKU-BOOK", 3))
        .unwrap();

    assert_eq!(write_model.available("SKU-BOOK"), 7);
}

#[test]
fn cqrs_inventory_queries_read_without_changing_state() {
    let mut write_model = InventoryWriteModel::default();
    InventoryCommandHandler::new(&mut write_model)
        .handle_receive(ReceiveStockCommand::new("SKU-BOOK", 10))
        .unwrap();
    let queries = InventoryQueryHandler::new(&write_model);

    let first = queries.handle(GetInventoryQuery::new("SKU-BOOK"));
    let second = queries.handle(GetInventoryQuery::new("SKU-BOOK"));

    assert_eq!(first, StockLevel::new("SKU-BOOK", 10));
    assert_eq!(second, StockLevel::new("SKU-BOOK", 10));
    assert_eq!(write_model.available("SKU-BOOK"), 10);
}

#[test]
fn cqrs_inventory_commands_reject_invalid_reservations() {
    let mut write_model = InventoryWriteModel::default();
    let mut commands = InventoryCommandHandler::new(&mut write_model);
    commands
        .handle_receive(ReceiveStockCommand::new("SKU-BOOK", 2))
        .unwrap();

    let result = commands.handle_reserve(ReserveStockCommand::new("SKU-BOOK", 5));

    assert!(result.is_err());
    assert_eq!(write_model.available("SKU-BOOK"), 2);
}
