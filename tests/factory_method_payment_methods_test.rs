use design_patterns_rust::patterns::gof::creational::factory_method::payment_methods::{
    PaymentChannel, PaymentError, charge,
};

#[test]
fn factory_method_creates_card_payment_from_checkout_channel() {
    let receipt = charge(PaymentChannel::Card, "order-42", 1_500).expect("card charge");

    assert_eq!(receipt, "card: authorized order-42 for 1500 cents");
}

#[test]
fn factory_method_creates_bank_transfer_from_checkout_channel() {
    let receipt = charge(PaymentChannel::BankTransfer, "order-42", 1_500).expect("bank transfer");

    assert_eq!(receipt, "bank-transfer: scheduled order-42 for 1500 cents");
}

#[test]
fn factory_method_creates_wallet_payment_from_checkout_channel() {
    let receipt = charge(PaymentChannel::Wallet, "order-42", 1_500).expect("wallet charge");

    assert_eq!(receipt, "wallet: captured order-42 for 1500 cents");
}

#[test]
fn factory_method_rejects_zero_amounts_before_creating_payment_method() {
    let error = charge(PaymentChannel::Card, "order-42", 0).expect_err("zero is invalid");

    assert_eq!(error, PaymentError::AmountMustBePositive);
}
