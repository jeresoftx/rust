use design_patterns_rust::patterns::architecture::cqrs::dashboard_read_model::{
    DashboardQuery, InventoryDashboardReadModel, InventorySummaryCard, ProductStockSnapshot,
    StockStatus,
};

#[test]
fn cqrs_dashboard_read_model_returns_cards_ready_for_the_ui() {
    let read_model = InventoryDashboardReadModel::from_snapshots(vec![
        ProductStockSnapshot::new("SKU-BOOK", "Rust Book", 12, 5),
        ProductStockSnapshot::new("SKU-PEN", "Marker Pen", 2, 5),
        ProductStockSnapshot::new("SKU-MUG", "Team Mug", 0, 5),
    ]);

    let cards = read_model.handle(DashboardQuery::all_products());

    assert_eq!(
        cards,
        vec![
            InventorySummaryCard::new("SKU-MUG", "Team Mug", 0, StockStatus::OutOfStock),
            InventorySummaryCard::new("SKU-PEN", "Marker Pen", 2, StockStatus::Low),
            InventorySummaryCard::new("SKU-BOOK", "Rust Book", 12, StockStatus::Healthy),
        ]
    );
}

#[test]
fn cqrs_dashboard_read_model_filters_only_products_that_need_attention() {
    let read_model = InventoryDashboardReadModel::from_snapshots(vec![
        ProductStockSnapshot::new("SKU-BOOK", "Rust Book", 12, 5),
        ProductStockSnapshot::new("SKU-PEN", "Marker Pen", 2, 5),
        ProductStockSnapshot::new("SKU-MUG", "Team Mug", 0, 5),
    ]);

    let cards = read_model.handle(DashboardQuery::needs_attention());

    assert_eq!(
        cards,
        vec![
            InventorySummaryCard::new("SKU-MUG", "Team Mug", 0, StockStatus::OutOfStock),
            InventorySummaryCard::new("SKU-PEN", "Marker Pen", 2, StockStatus::Low),
        ]
    );
}

#[test]
fn cqrs_dashboard_read_model_sorts_cards_by_status_and_name() {
    let read_model = InventoryDashboardReadModel::from_snapshots(vec![
        ProductStockSnapshot::new("SKU-PEN", "Marker Pen", 2, 5),
        ProductStockSnapshot::new("SKU-MUG", "Team Mug", 0, 5),
        ProductStockSnapshot::new("SKU-CAP", "Baseball Cap", 0, 5),
    ]);

    let cards = read_model.handle(DashboardQuery::all_products());

    assert_eq!(cards[0].name(), "Baseball Cap");
    assert_eq!(cards[1].name(), "Team Mug");
    assert_eq!(cards[2].name(), "Marker Pen");
}
