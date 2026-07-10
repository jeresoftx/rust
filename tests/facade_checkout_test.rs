use design_patterns_rust::patterns::gof::structural::facade::checkout::{
    Cart, CheckoutFacade, InventoryService, PaymentGateway,
};

#[test]
fn facade_completes_checkout_through_one_public_method() {
    let facade = CheckoutFacade::new(
        InventoryService::new(vec![("sku-1", 5), ("sku-2", 2)]),
        PaymentGateway::new("stripe-like"),
    );
    let cart = Cart::new("cart-77").with_item("sku-1", 2, 1500);

    assert_eq!(
        facade.checkout(cart),
        Ok("cart=cart-77 reserved=sku-1x2 charged=3000 provider=stripe-like".to_string())
    );
}

#[test]
fn facade_reports_inventory_failure_without_exposing_subsystems() {
    let facade = CheckoutFacade::new(
        InventoryService::new(vec![("sku-1", 1)]),
        PaymentGateway::new("stripe-like"),
    );
    let cart = Cart::new("cart-88").with_item("sku-1", 2, 1500);

    assert_eq!(
        facade.checkout(cart),
        Err("insufficient inventory for sku-1".to_string())
    );
}
