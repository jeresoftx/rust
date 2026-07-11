use design_patterns_rust::patterns::gof::behavioral::interpreter::discount_rules::{
    CartContext, DiscountRule,
};

#[test]
fn interpreter_applies_discount_when_rule_tree_matches_cart() {
    let rule = DiscountRule::and(
        DiscountRule::min_total(100),
        DiscountRule::customer_segment("vip"),
        20,
    );
    let cart = CartContext::new(150, "vip");

    assert_eq!(rule.discount_for(&cart), 20);
}

#[test]
fn interpreter_returns_zero_when_rule_tree_does_not_match_cart() {
    let rule = DiscountRule::or(
        DiscountRule::customer_segment("vip"),
        DiscountRule::min_total(500),
        15,
    );
    let cart = CartContext::new(120, "standard");

    assert_eq!(rule.discount_for(&cart), 0);
}
