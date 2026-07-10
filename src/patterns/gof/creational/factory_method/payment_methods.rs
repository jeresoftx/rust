#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentChannel {
    Card,
    BankTransfer,
    Wallet,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PaymentError {
    AmountMustBePositive,
}

trait PaymentMethod {
    fn charge(&self, order_id: &str, amount_cents: u32) -> String;
}

struct CardPayment;

impl PaymentMethod for CardPayment {
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("card: authorized {order_id} for {amount_cents} cents")
    }
}

struct BankTransferPayment;

impl PaymentMethod for BankTransferPayment {
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("bank-transfer: scheduled {order_id} for {amount_cents} cents")
    }
}

struct WalletPayment;

impl PaymentMethod for WalletPayment {
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("wallet: captured {order_id} for {amount_cents} cents")
    }
}

fn payment_method_for(channel: PaymentChannel) -> Box<dyn PaymentMethod> {
    match channel {
        PaymentChannel::Card => Box::new(CardPayment),
        PaymentChannel::BankTransfer => Box::new(BankTransferPayment),
        PaymentChannel::Wallet => Box::new(WalletPayment),
    }
}

pub fn charge(
    channel: PaymentChannel,
    order_id: &str,
    amount_cents: u32,
) -> Result<String, PaymentError> {
    if amount_cents == 0 {
        return Err(PaymentError::AmountMustBePositive);
    }

    let payment_method = payment_method_for(channel);

    Ok(payment_method.charge(order_id, amount_cents))
}
