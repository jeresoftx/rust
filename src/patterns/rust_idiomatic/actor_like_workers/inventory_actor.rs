use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `StockItem` usado por el ejemplo para expresar el dominio del patron.
pub struct StockItem {
    sku: String,
    quantity: u32,
}

impl StockItem {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ReservationResult` dentro del ejemplo.
pub enum ReservationResult {
    /// Variante `Reserved` del estado o error del ejemplo.
    Reserved {
        /// Variante `sku` del estado o error del ejemplo.
        sku: String,
        /// Variante `quantity` del estado o error del ejemplo.
        quantity: u32,
        /// Variante `remaining` del estado o error del ejemplo.
        remaining: u32,
    },
    /// Variante `Rejected` del estado o error del ejemplo.
    Rejected {
        /// Valor publico `sku` asociado a la variante del enum.
        sku: String,
        /// Valor publico `requested` asociado a la variante del enum.
        requested: u32,
        /// Valor publico `available` asociado a la variante del enum.
        available: u32,
    },
}

#[derive(Debug)]
/// Tipo publico `InventoryActor` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryActor {
    sender: Sender<InventoryCommand>,
    handle: JoinHandle<()>,
}

impl InventoryActor {
    /// Modela la operacion `start` dentro del ejemplo del patron.
    pub fn start(items: Vec<StockItem>) -> Self {
        let (sender, receiver) = mpsc::channel::<InventoryCommand>();
        let handle = thread::spawn(move || {
            let mut stock = items
                .into_iter()
                .map(|item| (item.sku, item.quantity))
                .collect::<HashMap<_, _>>();

            for command in receiver {
                match command {
                    InventoryCommand::Reserve {
                        sku,
                        quantity,
                        reply,
                    } => {
                        let available = stock.get(&sku).copied().unwrap_or_default();
                        let result = if available >= quantity {
                            let remaining = available - quantity;
                            stock.insert(sku.clone(), remaining);
                            ReservationResult::Reserved {
                                sku,
                                quantity,
                                remaining,
                            }
                        } else {
                            ReservationResult::Rejected {
                                sku,
                                requested: quantity,
                                available,
                            }
                        };

                        reply
                            .send(result)
                            .expect("reservation reply should be received");
                    }
                    InventoryCommand::Release { sku, quantity } => {
                        *stock.entry(sku).or_default() += quantity;
                    }
                    InventoryCommand::Stock { sku, reply } => {
                        reply
                            .send(stock.get(&sku).copied().unwrap_or_default())
                            .expect("stock reply should be received");
                    }
                    InventoryCommand::Shutdown => break,
                }
            }
        });

        Self { sender, handle }
    }

    /// Modela la operacion `reserve` dentro del ejemplo del patron.
    pub fn reserve(&self, sku: &str, quantity: u32) -> ReservationResult {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(InventoryCommand::Reserve {
                sku: sku.to_string(),
                quantity,
                reply,
            })
            .expect("inventory actor should be running");

        response
            .recv()
            .expect("inventory actor should reply to reserve command")
    }

    /// Modela la operacion `release` dentro del ejemplo del patron.
    pub fn release(&self, sku: &str, quantity: u32) {
        self.sender
            .send(InventoryCommand::Release {
                sku: sku.to_string(),
                quantity,
            })
            .expect("inventory actor should be running");
    }

    /// Modela la operacion `stock` dentro del ejemplo del patron.
    pub fn stock(&self, sku: &str) -> u32 {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(InventoryCommand::Stock {
                sku: sku.to_string(),
                reply,
            })
            .expect("inventory actor should be running");

        response
            .recv()
            .expect("inventory actor should reply to stock command")
    }

    /// Modela la operacion `shutdown` dentro del ejemplo del patron.
    pub fn shutdown(self) {
        self.sender
            .send(InventoryCommand::Shutdown)
            .expect("inventory actor should receive shutdown command");
        self.handle
            .join()
            .expect("inventory actor should stop cleanly");
    }
}

#[derive(Debug)]
enum InventoryCommand {
    Reserve {
        sku: String,
        quantity: u32,
        reply: Sender<ReservationResult>,
    },
    Release {
        sku: String,
        quantity: u32,
    },
    Stock {
        sku: String,
        reply: Sender<u32>,
    },
    Shutdown,
}
