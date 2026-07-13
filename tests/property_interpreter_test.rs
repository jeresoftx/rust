use design_patterns_rust::patterns::gof::behavioral::interpreter::discount_rules::{
    CartContext, DiscountRule,
};
use design_patterns_rust::patterns::gof::behavioral::interpreter::search_filter::{
    SearchFilter, SearchRecord,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn discount_rule_matches_generated_cart_threshold(
        total in 0_u32..100_000,
        threshold in 0_u32..100_000,
        discount in 1_u8..100,
    ) {
        let rule = DiscountRule::and(
            DiscountRule::min_total(threshold),
            DiscountRule::customer_segment("vip"),
            discount,
        );
        let cart = CartContext::new(total, "vip");

        let expected = if total >= threshold { discount } else { 0 };

        prop_assert_eq!(rule.discount_for(&cart), expected);
    }

    #[test]
    fn parsed_total_filter_matches_strictly_greater_totals(total in 0_u32..100_000, cutoff in 0_u32..100_000) {
        let filter = SearchFilter::parse(&format!("total > {cutoff}")).expect("generated filter is valid");
        let record = SearchRecord::new("paid", total);

        prop_assert_eq!(filter.matches(&record), total > cutoff);
    }
}
