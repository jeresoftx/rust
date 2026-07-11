use std::sync::Arc;

use design_patterns_rust::patterns::gof::structural::flyweight::currency_catalog::{
    CurrencyCatalog, MoneyAmount,
};

#[test]
fn flyweight_reuses_currency_instances_from_catalog() {
    let catalog = CurrencyCatalog::default();

    let usd_for_invoice = catalog.currency("USD").expect("USD should exist");
    let usd_for_refund = catalog.currency("USD").expect("USD should exist");

    assert!(Arc::ptr_eq(&usd_for_invoice, &usd_for_refund));
    assert_eq!(usd_for_invoice.summary(), "USD symbol=$ decimals=2");
}

#[test]
fn flyweight_keeps_amount_state_outside_shared_currency() {
    let catalog = CurrencyCatalog::default();
    let amount = MoneyAmount::new(1599, catalog.currency("USD").expect("USD should exist"));

    assert_eq!(amount.format(), "$15.99 USD");
}
