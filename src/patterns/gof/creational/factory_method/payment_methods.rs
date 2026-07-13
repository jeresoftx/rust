#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PaymentChannel` dentro del ejemplo.
pub enum PaymentChannel {
    /// Variante `Card` del estado o error del ejemplo.
    Card,
    /// Variante `BankTransfer` del estado o error del ejemplo.
    BankTransfer,
    /// Variante `Wallet` del estado o error del ejemplo.
    Wallet,
}

#[derive(Debug, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `PaymentError` dentro del ejemplo.
pub enum PaymentError {
    /// Variante `AmountMustBePositive` del estado o error del ejemplo.
    AmountMustBePositive,
}

trait PaymentMethod {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, order_id: &str, amount_cents: u32) -> String;
}

struct CardPayment;

impl PaymentMethod for CardPayment {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("card: authorized {order_id} for {amount_cents} cents")
    }
}

struct BankTransferPayment;

impl PaymentMethod for BankTransferPayment {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("bank-transfer: scheduled {order_id} for {amount_cents} cents")
    }
}

struct WalletPayment;

impl PaymentMethod for WalletPayment {
    /// Operacion `charge` definida por la abstraccion del ejemplo.
    fn charge(&self, order_id: &str, amount_cents: u32) -> String {
        format!("wallet: captured {order_id} for {amount_cents} cents")
    }
}

/// Operacion `payment method for` definida por la abstraccion del ejemplo.
fn payment_method_for(channel: PaymentChannel) -> Box<dyn PaymentMethod> {
    match channel {
        PaymentChannel::Card => Box::new(CardPayment),
        PaymentChannel::BankTransfer => Box::new(BankTransferPayment),
        PaymentChannel::Wallet => Box::new(WalletPayment),
    }
}

/// Modela la operacion `charge` dentro del ejemplo del patron.
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
