use design_patterns_rust::patterns::rust_idiomatic::actor_like_workers::inventory_actor::{
    InventoryActor, ReservationResult, StockItem,
};

#[test]
fn actor_like_inventory_actor_serializes_reservations() {
    let actor = InventoryActor::start(vec![StockItem::new("keyboard", 5)]);

    let first = actor.reserve("keyboard", 3);
    let second = actor.reserve("keyboard", 3);

    assert_eq!(
        first,
        ReservationResult::Reserved {
            sku: "keyboard".to_string(),
            quantity: 3,
            remaining: 2
        }
    );
    assert_eq!(
        second,
        ReservationResult::Rejected {
            sku: "keyboard".to_string(),
            requested: 3,
            available: 2
        }
    );
    assert_eq!(actor.stock("keyboard"), 2);

    actor.shutdown();
}

#[test]
fn actor_like_inventory_actor_releases_stock_back_to_inventory() {
    let actor = InventoryActor::start(vec![StockItem::new("monitor", 4)]);

    actor.reserve("monitor", 4);
    actor.release("monitor", 2);

    assert_eq!(actor.stock("monitor"), 2);
    assert_eq!(
        actor.reserve("monitor", 2),
        ReservationResult::Reserved {
            sku: "monitor".to_string(),
            quantity: 2,
            remaining: 0
        }
    );

    actor.shutdown();
}

#[test]
fn actor_like_inventory_actor_handles_unknown_skus_as_zero_stock() {
    let actor = InventoryActor::start(vec![StockItem::new("mouse", 1)]);

    assert_eq!(actor.stock("missing"), 0);
    assert_eq!(
        actor.reserve("missing", 1),
        ReservationResult::Rejected {
            sku: "missing".to_string(),
            requested: 1,
            available: 0
        }
    );

    actor.shutdown();
}
