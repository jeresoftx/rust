use design_patterns_rust::patterns::architecture::clean_architecture::framework_independent_rules::{
    adapters::{HttpCartLineDto, HttpDiscountController, HttpDiscountRequest},
    entities::{Cart, CartLine, DiscountPolicy},
    use_cases::{CalculateDiscount, DiscountRequest},
};

#[test]
fn clean_architecture_business_rules_run_without_framework_types() {
    let cart = Cart::new(vec![CartLine::new("book", 3, 2_000)]);
    let use_case = CalculateDiscount::new(DiscountPolicy::new(10_000, 15));

    let result = use_case.execute(DiscountRequest::new(cart));

    assert_eq!(result.subtotal_cents(), 6_000);
    assert_eq!(result.discount_cents(), 0);
    assert_eq!(result.total_cents(), 6_000);
}

#[test]
fn clean_architecture_business_rules_apply_policy_when_threshold_is_reached() {
    let cart = Cart::new(vec![
        CartLine::new("monitor", 1, 8_000),
        CartLine::new("keyboard", 2, 2_000),
    ]);
    let use_case = CalculateDiscount::new(DiscountPolicy::new(10_000, 15));

    let result = use_case.execute(DiscountRequest::new(cart));

    assert_eq!(result.subtotal_cents(), 12_000);
    assert_eq!(result.discount_cents(), 1_800);
    assert_eq!(result.total_cents(), 10_200);
}

#[test]
fn clean_architecture_http_adapter_only_translates_request_and_response() {
    let use_case = CalculateDiscount::new(DiscountPolicy::new(10_000, 10));
    let controller = HttpDiscountController::new(use_case);
    let request = HttpDiscountRequest::new(vec![HttpCartLineDto::new("monitor", 1, 12_000)]);

    let response = controller.handle(request);

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.body(), "subtotal=12000 discount=1200 total=10800");
}
