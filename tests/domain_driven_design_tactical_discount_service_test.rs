use design_patterns_rust::patterns::architecture::domain_driven_design_tactical::discount_service::{
    Cart, Coupon, CustomerSegment, DiscountBreakdown, DiscountPolicyService, Money,
};

#[test]
fn ddd_domain_service_applies_customer_segment_policy() {
    let service = DiscountPolicyService::default();
    let cart = Cart::new(Money::usd(20_000));

    let discount = service.calculate_discount(CustomerSegment::Premium, &cart, None);

    assert_eq!(
        discount,
        DiscountBreakdown::new(Money::usd(20_000), Money::usd(3_000), "premium segment")
    );
}

#[test]
fn ddd_domain_service_chooses_best_discount_without_stacking() {
    let service = DiscountPolicyService::default();
    let cart = Cart::new(Money::usd(20_000));
    let coupon = Coupon::percent("SUMMER20", 20).unwrap();

    let discount = service.calculate_discount(CustomerSegment::Premium, &cart, Some(&coupon));

    assert_eq!(discount.discount(), Money::usd(4_000));
    assert_eq!(discount.reason(), "coupon SUMMER20");
}

#[test]
fn ddd_domain_service_rejects_invalid_coupon_percentages() {
    assert!(Coupon::percent("ZERO", 0).is_err());
    assert!(Coupon::percent("TOO-MUCH", 100).is_err());
}

#[test]
fn ddd_domain_service_keeps_regular_customers_without_discount_when_no_coupon_exists() {
    let service = DiscountPolicyService::default();
    let cart = Cart::new(Money::usd(8_000));

    let discount = service.calculate_discount(CustomerSegment::Regular, &cart, None);

    assert_eq!(discount.discount(), Money::usd(0));
    assert_eq!(discount.total_after_discount(), Money::usd(8_000));
    assert_eq!(discount.reason(), "no discount");
}
