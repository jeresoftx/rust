use std::sync::mpsc;
use std::thread;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RawOrder` usado por el ejemplo para expresar el dominio del patron.
pub struct RawOrder {
    id: String,
    lines: Vec<OrderLine>,
}

impl RawOrder {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, lines: Vec<OrderLine>) -> Self {
        Self {
            id: id.into(),
            lines,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidOrder {
    id: String,
    total_units: u32,
}

impl ValidOrder {
    /// Operacion `from raw` definida por la abstraccion del ejemplo.
    fn from_raw(order: RawOrder) -> Option<Self> {
        let total_units = order.lines.iter().map(|line| line.quantity).sum::<u32>();

        if total_units == 0 {
            None
        } else {
            Some(Self {
                id: order.id,
                total_units,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `FulfillmentTask` usado por el ejemplo para expresar el dominio del patron.
pub struct FulfillmentTask {
    order_id: String,
    total_units: u32,
    priority: String,
}

impl FulfillmentTask {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(order_id: impl Into<String>, total_units: u32, priority: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            total_units,
            priority: priority.into(),
        }
    }

    /// Modela la operacion `order id` dentro del ejemplo del patron.
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    /// Modela la operacion `total units` dentro del ejemplo del patron.
    pub fn total_units(&self) -> u32 {
        self.total_units
    }

    /// Modela la operacion `priority` dentro del ejemplo del patron.
    pub fn priority(&self) -> &str {
        &self.priority
    }
}

/// Modela la operacion `run fulfillment pipeline` dentro del ejemplo del patron.
pub fn run_fulfillment_pipeline(raw_orders: Vec<RawOrder>) -> Vec<FulfillmentTask> {
    let (raw_sender, raw_receiver) = mpsc::channel::<RawOrder>();
    let (valid_sender, valid_receiver) = mpsc::channel::<ValidOrder>();
    let (task_sender, task_receiver) = mpsc::channel::<FulfillmentTask>();

    let validator = thread::spawn(move || {
        for order in raw_receiver {
            if let Some(valid_order) = ValidOrder::from_raw(order) {
                valid_sender
                    .send(valid_order)
                    .expect("validation stage should send valid orders");
            }
        }
    });

    let planner = thread::spawn(move || {
        for order in valid_receiver {
            let priority = if order.total_units >= 10 {
                "bulk"
            } else {
                "standard"
            };

            task_sender
                .send(FulfillmentTask::new(order.id, order.total_units, priority))
                .expect("planner stage should send fulfillment tasks");
        }
    });

    for order in raw_orders {
        raw_sender
            .send(order)
            .expect("input stage should send raw orders");
    }

    drop(raw_sender);
    validator
        .join()
        .expect("validation stage should finish cleanly");
    planner.join().expect("planner stage should finish cleanly");

    task_receiver.into_iter().collect()
}
