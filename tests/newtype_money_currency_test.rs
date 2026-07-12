use design_patterns_rust::patterns::rust_idiomatic::newtype::money_currency::{
    CurrencyCode, Money,
};

#[test]
fn newtype_formats_money_with_currency() {
    let total = Money::new(12_345, CurrencyCode::new("USD").unwrap());

    assert_eq!(total.cents(), 12_345);
    assert_eq!(total.currency().as_str(), "USD");
    assert_eq!(total.format(), "USD 123.45");
}

#[test]
fn newtype_adds_money_only_when_currency_matches() {
    let price = Money::new(10_000, CurrencyCode::new("MXN").unwrap());
    let tax = Money::new(1_600, CurrencyCode::new("MXN").unwrap());

    let total = price.add(&tax).unwrap();

    assert_eq!(total.format(), "MXN 116.00");
}

#[test]
fn newtype_rejects_mixed_currency_addition() {
    let usd = Money::new(10_000, CurrencyCode::new("USD").unwrap());
    let mxn = Money::new(10_000, CurrencyCode::new("MXN").unwrap());

    assert_eq!(
        usd.add(&mxn).unwrap_err(),
        "cannot add USD money to MXN money"
    );
}

#[test]
fn newtype_rejects_invalid_currency_codes() {
    assert_eq!(
        CurrencyCode::new("usd").unwrap_err(),
        "currency code must be three uppercase ASCII letters"
    );
}
