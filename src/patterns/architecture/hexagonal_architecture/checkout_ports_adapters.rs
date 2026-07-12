pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderLine {
        sku: String,
        quantity: u32,
    }

    impl OrderLine {
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }

        pub fn sku(&self) -> &str {
            &self.sku
        }

        pub fn quantity(&self) -> u32 {
            self.quantity
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CheckoutReceipt {
        order_id: String,
        total_cents: u64,
    }

    impl CheckoutReceipt {
        pub fn new(order_id: impl Into<String>, total_cents: u64) -> Self {
            Self {
                order_id: order_id.into(),
                total_cents,
            }
        }

        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        pub fn total_cents(&self) -> u64 {
            self.total_cents
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CheckoutError {
        EmptyCart,
        SkuNotFound {
            sku: String,
        },
        InsufficientInventory {
            sku: String,
            requested: u32,
            available: u32,
        },
        PaymentRejected,
    }
}

pub mod ports {
    use super::domain::CheckoutError;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InventoryItem {
        pub sku: String,
        pub available: u32,
        pub unit_price_cents: u64,
    }

    pub trait InventoryPort {
        fn find(&self, sku: &str) -> Option<InventoryItem>;
        fn reserve(&mut self, sku: &str, quantity: u32);
        fn remaining_stock(&self, sku: &str) -> u32;
    }

    pub trait PaymentPort {
        fn charge(&mut self, card_id: &str, amount_cents: u64) -> Result<(), CheckoutError>;
        fn charged_amounts(&self) -> Vec<u64>;
    }
}

pub mod application {
    use super::domain::{CheckoutError, CheckoutReceipt, OrderLine};
    use super::ports::{InventoryPort, PaymentPort};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CheckoutRequest {
        order_id: String,
        card_id: String,
        lines: Vec<OrderLine>,
    }

    impl CheckoutRequest {
        pub fn new(
            order_id: impl Into<String>,
            card_id: impl Into<String>,
            lines: Vec<OrderLine>,
        ) -> Self {
            Self {
                order_id: order_id.into(),
                card_id: card_id.into(),
                lines,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Checkout<I, P> {
        inventory: I,
        payments: P,
    }

    impl<I, P> Checkout<I, P>
    where
        I: InventoryPort,
        P: PaymentPort,
    {
        pub fn new(inventory: I, payments: P) -> Self {
            Self {
                inventory,
                payments,
            }
        }

        pub fn execute(
            &mut self,
            request: CheckoutRequest,
        ) -> Result<CheckoutReceipt, CheckoutError> {
            if request.lines.is_empty() {
                return Err(CheckoutError::EmptyCart);
            }

            let mut total_cents = 0;

            for line in &request.lines {
                let item =
                    self.inventory
                        .find(line.sku())
                        .ok_or_else(|| CheckoutError::SkuNotFound {
                            sku: line.sku().to_string(),
                        })?;

                if item.available < line.quantity() {
                    return Err(CheckoutError::InsufficientInventory {
                        sku: line.sku().to_string(),
                        requested: line.quantity(),
                        available: item.available,
                    });
                }

                total_cents += item.unit_price_cents * u64::from(line.quantity());
            }

            self.payments.charge(&request.card_id, total_cents)?;

            for line in &request.lines {
                self.inventory.reserve(line.sku(), line.quantity());
            }

            Ok(CheckoutReceipt::new(request.order_id, total_cents))
        }

        pub fn remaining_stock(&self, sku: &str) -> u32 {
            self.inventory.remaining_stock(sku)
        }

        pub fn charged_amounts(&self) -> Vec<u64> {
            self.payments.charged_amounts()
        }
    }
}

pub mod adapters {
    use std::collections::HashMap;

    use super::domain::CheckoutError;
    use super::ports::{InventoryItem, InventoryPort, PaymentPort};

    #[derive(Debug, Clone, Default)]
    pub struct InMemoryInventory {
        stock: HashMap<String, InventoryItem>,
    }

    impl InMemoryInventory {
        pub fn with_stock(items: Vec<(&str, u32, u64)>) -> Self {
            Self {
                stock: items
                    .into_iter()
                    .map(|(sku, available, unit_price_cents)| {
                        (
                            sku.to_string(),
                            InventoryItem {
                                sku: sku.to_string(),
                                available,
                                unit_price_cents,
                            },
                        )
                    })
                    .collect(),
            }
        }
    }

    impl InventoryPort for InMemoryInventory {
        fn find(&self, sku: &str) -> Option<InventoryItem> {
            self.stock.get(sku).cloned()
        }

        fn reserve(&mut self, sku: &str, quantity: u32) {
            if let Some(item) = self.stock.get_mut(sku) {
                item.available -= quantity;
            }
        }

        fn remaining_stock(&self, sku: &str) -> u32 {
            self.stock
                .get(sku)
                .map(|item| item.available)
                .unwrap_or_default()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct RecordingPaymentGateway {
        approved: bool,
        charged_amounts: Vec<u64>,
    }

    impl RecordingPaymentGateway {
        pub fn approved() -> Self {
            Self {
                approved: true,
                charged_amounts: Vec::new(),
            }
        }

        pub fn rejected() -> Self {
            Self {
                approved: false,
                charged_amounts: Vec::new(),
            }
        }
    }

    impl PaymentPort for RecordingPaymentGateway {
        fn charge(&mut self, _card_id: &str, amount_cents: u64) -> Result<(), CheckoutError> {
            self.charged_amounts.push(amount_cents);

            if self.approved {
                Ok(())
            } else {
                Err(CheckoutError::PaymentRejected)
            }
        }

        fn charged_amounts(&self) -> Vec<u64> {
            self.charged_amounts.clone()
        }
    }
}
