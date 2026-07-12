use std::sync::mpsc;
use std::thread;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawOrder {
    id: String,
    lines: Vec<OrderLine>,
}

impl RawOrder {
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
pub struct FulfillmentTask {
    order_id: String,
    total_units: u32,
    priority: String,
}

impl FulfillmentTask {
    pub fn new(order_id: impl Into<String>, total_units: u32, priority: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
            total_units,
            priority: priority.into(),
        }
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn total_units(&self) -> u32 {
        self.total_units
    }

    pub fn priority(&self) -> &str {
        &self.priority
    }
}

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
