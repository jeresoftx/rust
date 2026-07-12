use design_patterns_rust::patterns::rust_idiomatic::newtype::typed_ids::{
    OrderAssignment, OrderId, ProductId, ProductReservation, UserId,
};

#[test]
fn newtype_keeps_domain_ids_distinct() {
    let user_id = UserId::new("usr_123").unwrap();
    let order_id = OrderId::new("ord_456").unwrap();
    let assignment = OrderAssignment::new(user_id.clone(), order_id.clone());

    assert_eq!(user_id.as_str(), "usr_123");
    assert_eq!(order_id.as_str(), "ord_456");
    assert_eq!(assignment.audit_key(), "usr_123->ord_456");
}

#[test]
fn newtype_rejects_ids_with_wrong_prefix() {
    assert_eq!(
        UserId::new("ord_456").unwrap_err(),
        "UserId must start with usr_"
    );
    assert_eq!(
        ProductId::new("usr_123").unwrap_err(),
        "ProductId must start with prd_"
    );
}

#[test]
fn newtype_uses_typed_ids_in_order_reservations() {
    let order_id = OrderId::new("ord_456").unwrap();
    let product_id = ProductId::new("prd_789").unwrap();
    let reservation = ProductReservation::new(order_id, product_id, 3);

    assert_eq!(reservation.summary(), "ord_456 reserves 3 of prd_789");
}
