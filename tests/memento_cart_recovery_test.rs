use design_patterns_rust::patterns::gof::behavioral::memento::cart_recovery::{
    CartBackupStore, CartLine, ShoppingCart,
};

#[test]
fn memento_restores_cart_after_interrupted_checkout() {
    let mut cart = ShoppingCart::new("session-1");
    cart.add_item("book", 2, 1500);
    cart.apply_coupon("SAVE10");

    let checkout_checkpoint = cart.save();
    cart.add_item("pen", 3, 200);
    cart.clear_coupon();

    cart.restore(checkout_checkpoint);

    assert_eq!(cart.items(), vec![CartLine::new("book", 2, 1500)]);
    assert_eq!(cart.coupon(), Some("SAVE10"));
    assert_eq!(cart.total_cents(), 2700);
}

#[test]
fn memento_store_recovers_cart_by_session() {
    let mut store = CartBackupStore::new();
    let mut cart = ShoppingCart::new("session-2");
    cart.add_item("keyboard", 1, 5000);

    store.capture(&cart);
    cart.clear();

    assert!(store.restore("session-2", &mut cart));
    assert_eq!(cart.items(), vec![CartLine::new("keyboard", 1, 5000)]);
    assert_eq!(cart.total_cents(), 5000);

    assert!(!store.restore("missing", &mut cart));
}
