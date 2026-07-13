//! Checkout Ports Adapters.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::hexagonal_architecture::checkout_ports_adapters as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `OrderLine` usado por el ejemplo para expresar el dominio del patron.
    pub struct OrderLine {
        sku: String,
        quantity: u32,
    }

    impl OrderLine {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
            Self {
                sku: sku.into(),
                quantity,
            }
        }

        /// Modela la operacion `sku` dentro del ejemplo del patron.
        pub fn sku(&self) -> &str {
            &self.sku
        }

        /// Modela la operacion `quantity` dentro del ejemplo del patron.
        pub fn quantity(&self) -> u32 {
            self.quantity
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CheckoutReceipt` usado por el ejemplo para expresar el dominio del patron.
    pub struct CheckoutReceipt {
        order_id: String,
        total_cents: u64,
    }

    impl CheckoutReceipt {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(order_id: impl Into<String>, total_cents: u64) -> Self {
            Self {
                order_id: order_id.into(),
                total_cents,
            }
        }

        /// Modela la operacion `order id` dentro del ejemplo del patron.
        pub fn order_id(&self) -> &str {
            &self.order_id
        }

        /// Modela la operacion `total cents` dentro del ejemplo del patron.
        pub fn total_cents(&self) -> u64 {
            self.total_cents
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `CheckoutError` dentro del ejemplo.
    pub enum CheckoutError {
        /// Variante `EmptyCart` del estado o error del ejemplo.
        EmptyCart,
        /// Variante `SkuNotFound` del estado o error del ejemplo.
        SkuNotFound {
            /// Valor publico `sku` asociado a la variante `SkuNotFound`.
            sku: String,
        },
        /// Variante `InsufficientInventory` del estado o error del ejemplo.
        InsufficientInventory {
            /// Valor publico `sku` asociado a la variante del enum.
            sku: String,
            /// Valor publico `requested` asociado a la variante del enum.
            requested: u32,
            /// Valor publico `available` asociado a la variante del enum.
            available: u32,
        },
        /// Variante `PaymentRejected` del estado o error del ejemplo.
        PaymentRejected,
    }
}

/// Modulo del ejemplo `ports` dentro del catalogo de patrones.
pub mod ports {
    use super::domain::CheckoutError;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `InventoryItem` usado por el ejemplo para expresar el dominio del patron.
    pub struct InventoryItem {
        /// Campo publico `sku` expuesto por el tipo del ejemplo.
        pub sku: String,
        /// Campo publico `available` expuesto por el tipo del ejemplo.
        pub available: u32,
        /// Campo publico `unit_price_cents` expuesto por el tipo del ejemplo.
        pub unit_price_cents: u64,
    }

    /// Contrato publico `InventoryPort` que desacopla las piezas del ejemplo.
    pub trait InventoryPort {
        /// Operacion `find` definida por la abstraccion del ejemplo.
        fn find(&self, sku: &str) -> Option<InventoryItem>;
        /// Operacion `reserve` definida por la abstraccion del ejemplo.
        fn reserve(&mut self, sku: &str, quantity: u32);
        /// Operacion `remaining stock` definida por la abstraccion del ejemplo.
        fn remaining_stock(&self, sku: &str) -> u32;
    }

    /// Contrato publico `PaymentPort` que desacopla las piezas del ejemplo.
    pub trait PaymentPort {
        /// Operacion `charge` definida por la abstraccion del ejemplo.
        fn charge(&mut self, card_id: &str, amount_cents: u64) -> Result<(), CheckoutError>;
        /// Operacion `charged amounts` definida por la abstraccion del ejemplo.
        fn charged_amounts(&self) -> Vec<u64>;
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::{CheckoutError, CheckoutReceipt, OrderLine};
    use super::ports::{InventoryPort, PaymentPort};

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `CheckoutRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct CheckoutRequest {
        order_id: String,
        card_id: String,
        lines: Vec<OrderLine>,
    }

    impl CheckoutRequest {
        /// Crea una instancia valida para el ejemplo del patron.
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
    /// Tipo publico `Checkout` usado por el ejemplo para expresar el dominio del patron.
    pub struct Checkout<I, P> {
        inventory: I,
        payments: P,
    }

    impl<I, P> Checkout<I, P>
    where
        I: InventoryPort,
        P: PaymentPort,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(inventory: I, payments: P) -> Self {
            Self {
                inventory,
                payments,
            }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
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

        /// Modela la operacion `remaining stock` dentro del ejemplo del patron.
        pub fn remaining_stock(&self, sku: &str) -> u32 {
            self.inventory.remaining_stock(sku)
        }

        /// Modela la operacion `charged amounts` dentro del ejemplo del patron.
        pub fn charged_amounts(&self) -> Vec<u64> {
            self.payments.charged_amounts()
        }
    }
}

/// Modulo del ejemplo `adapters` dentro del catalogo de patrones.
pub mod adapters {
    use std::collections::HashMap;

    use super::domain::CheckoutError;
    use super::ports::{InventoryItem, InventoryPort, PaymentPort};

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `InMemoryInventory` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryInventory {
        stock: HashMap<String, InventoryItem>,
    }

    impl InMemoryInventory {
        /// Modela la operacion `with stock` dentro del ejemplo del patron.
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
        /// Operacion `find` definida por la abstraccion del ejemplo.
        fn find(&self, sku: &str) -> Option<InventoryItem> {
            self.stock.get(sku).cloned()
        }

        /// Operacion `reserve` definida por la abstraccion del ejemplo.
        fn reserve(&mut self, sku: &str, quantity: u32) {
            if let Some(item) = self.stock.get_mut(sku) {
                item.available -= quantity;
            }
        }

        /// Operacion `remaining stock` definida por la abstraccion del ejemplo.
        fn remaining_stock(&self, sku: &str) -> u32 {
            self.stock
                .get(sku)
                .map(|item| item.available)
                .unwrap_or_default()
        }
    }

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `RecordingPaymentGateway` usado por el ejemplo para expresar el dominio del patron.
    pub struct RecordingPaymentGateway {
        approved: bool,
        charged_amounts: Vec<u64>,
    }

    impl RecordingPaymentGateway {
        /// Modela la operacion `approved` dentro del ejemplo del patron.
        pub fn approved() -> Self {
            Self {
                approved: true,
                charged_amounts: Vec::new(),
            }
        }

        /// Modela la operacion `rejected` dentro del ejemplo del patron.
        pub fn rejected() -> Self {
            Self {
                approved: false,
                charged_amounts: Vec::new(),
            }
        }
    }

    impl PaymentPort for RecordingPaymentGateway {
        /// Operacion `charge` definida por la abstraccion del ejemplo.
        fn charge(&mut self, _card_id: &str, amount_cents: u64) -> Result<(), CheckoutError> {
            self.charged_amounts.push(amount_cents);

            if self.approved {
                Ok(())
            } else {
                Err(CheckoutError::PaymentRejected)
            }
        }

        /// Operacion `charged amounts` definida por la abstraccion del ejemplo.
        fn charged_amounts(&self) -> Vec<u64> {
            self.charged_amounts.clone()
        }
    }
}
