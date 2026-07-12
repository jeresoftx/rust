use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockItem {
    sku: String,
    quantity: u32,
}

impl StockItem {
    pub fn new(sku: impl Into<String>, quantity: u32) -> Self {
        Self {
            sku: sku.into(),
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservationResult {
    Reserved {
        sku: String,
        quantity: u32,
        remaining: u32,
    },
    Rejected {
        sku: String,
        requested: u32,
        available: u32,
    },
}

#[derive(Debug)]
pub struct InventoryActor {
    sender: Sender<InventoryCommand>,
    handle: JoinHandle<()>,
}

impl InventoryActor {
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

    pub fn release(&self, sku: &str, quantity: u32) {
        self.sender
            .send(InventoryCommand::Release {
                sku: sku.to_string(),
                quantity,
            })
            .expect("inventory actor should be running");
    }

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
