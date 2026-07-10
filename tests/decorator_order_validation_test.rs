use design_patterns_rust::patterns::gof::structural::decorator::order_validation::{
    InventoryValidator, MinimumAmountValidator, NonEmptyOrderValidator, Order, ValidOrder,
    ValidationStep, validate_order,
};

#[test]
fn decorator_runs_order_validation_pipeline() {
    let validator = InventoryValidator::new(
        MinimumAmountValidator::new(NonEmptyOrderValidator::new(ValidOrder), 1000),
        vec!["sku-1", "sku-2"],
    );
    let order = Order::new("order-1", vec![("sku-1", 2)], 1500);

    assert_eq!(
        validate_order(&validator, &order),
        "order=order-1 status=valid checks=non-empty,min-amount,inventory"
    );
}

#[test]
fn decorator_stops_pipeline_when_a_validation_fails() {
    let validator = InventoryValidator::new(
        MinimumAmountValidator::new(NonEmptyOrderValidator::new(ValidOrder), 1000),
        vec!["sku-1"],
    );
    let order = Order::new("order-2", vec![("sku-2", 1)], 1500);

    assert_eq!(
        validator.validate(&order),
        Err("missing inventory for sku-2".to_string())
    );
}
