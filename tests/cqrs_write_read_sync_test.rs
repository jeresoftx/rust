use design_patterns_rust::patterns::architecture::cqrs::write_read_sync::{
    InventoryProjection, InventoryReadModel, InventoryWriteModel, ReceiveStockCommand,
    ReserveStockCommand, StockChanged,
};

#[test]
fn cqrs_projection_syncs_write_events_into_read_model() {
    let mut write_model = InventoryWriteModel::default();
    let mut read_model = InventoryReadModel::default();
    let mut projection = InventoryProjection::new(&mut read_model);

    let event = write_model.receive(ReceiveStockCommand::new("SKU-BOOK", 10));
    projection.apply(event);

    assert_eq!(read_model.available("SKU-BOOK"), 10);
    assert_eq!(read_model.last_event_for("SKU-BOOK"), Some("received 10"));
}

#[test]
fn cqrs_projection_applies_multiple_write_events_in_order() {
    let mut write_model = InventoryWriteModel::default();
    let mut read_model = InventoryReadModel::default();

    let received = write_model.receive(ReceiveStockCommand::new("SKU-BOOK", 10));
    let reserved = write_model
        .reserve(ReserveStockCommand::new("SKU-BOOK", 4))
        .unwrap();

    InventoryProjection::new(&mut read_model).apply_many(vec![received, reserved]);

    assert_eq!(read_model.available("SKU-BOOK"), 6);
    assert_eq!(read_model.last_event_for("SKU-BOOK"), Some("reserved 4"));
}

#[test]
fn cqrs_projection_can_rebuild_read_model_from_events() {
    let events = vec![
        StockChanged::received("SKU-BOOK", 10, 10),
        StockChanged::reserved("SKU-BOOK", 4, 6),
        StockChanged::received("SKU-PEN", 5, 5),
    ];

    let read_model = InventoryProjection::rebuild(events);

    assert_eq!(read_model.available("SKU-BOOK"), 6);
    assert_eq!(read_model.available("SKU-PEN"), 5);
}
