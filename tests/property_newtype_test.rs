use design_patterns_rust::patterns::rust_idiomatic::newtype::money_currency::{
    CurrencyCode, Money,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn money_addition_preserves_currency_and_adds_cents(
        left in -1_000_000_i64..1_000_000,
        right in -1_000_000_i64..1_000_000,
        code in "[A-Z]{3}",
    ) {
        let currency = CurrencyCode::new(code).expect("generated code is valid");
        let left_money = Money::new(left, currency.clone());
        let right_money = Money::new(right, currency.clone());

        let sum = left_money.add(&right_money).expect("same currency can be added");

        prop_assert_eq!(sum.cents(), left + right);
        prop_assert_eq!(sum.currency(), &currency);
    }

    #[test]
    fn currency_code_accepts_only_three_uppercase_ascii_letters(value in "[A-Za-z0-9]{0,6}") {
        let result = CurrencyCode::new(value.clone());
        let expected = value.len() == 3 && value.chars().all(|character| character.is_ascii_uppercase());

        prop_assert_eq!(result.is_ok(), expected);
    }
}
