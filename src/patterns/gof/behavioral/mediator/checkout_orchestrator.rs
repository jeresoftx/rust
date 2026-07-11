use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutRequest {
    order_id: String,
    sku: String,
    quantity: u32,
    amount_cents: u32,
}

impl CheckoutRequest {
    pub fn new(
        order_id: impl Into<String>,
        sku: impl Into<String>,
        quantity: u32,
        amount_cents: u32,
    ) -> Self {
        Self {
            order_id: order_id.into(),
            sku: sku.into(),
            quantity,
            amount_cents,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutResult {
    pub order_id: String,
    pub tracking_code: String,
}

#[derive(Debug, Default, Clone)]
pub struct CheckoutMediator {
    stock: BTreeMap<String, u32>,
    events: Vec<String>,
}

impl CheckoutMediator {
    pub fn new() -> Self {
        Self {
            stock: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    pub fn stock_item(&mut self, sku: impl Into<String>, quantity: u32) {
        self.stock.insert(sku.into(), quantity);
    }

    pub fn checkout(&mut self, request: CheckoutRequest) -> Result<CheckoutResult, String> {
        self.reserve_inventory(&request)?;

        if let Err(error) = self.charge_payment(request.amount_cents) {
            self.release_inventory(&request);
            return Err(error);
        }

        let tracking_code = self.create_shipping(&request.order_id);
        self.events
            .push(format!("notification:sent:{}", request.order_id));

        Ok(CheckoutResult {
            order_id: request.order_id,
            tracking_code,
        })
    }

    pub fn stock_for(&self, sku: &str) -> u32 {
        self.stock.get(sku).copied().unwrap_or_default()
    }

    pub fn events(&self) -> Vec<String> {
        self.events.clone()
    }

    fn reserve_inventory(&mut self, request: &CheckoutRequest) -> Result<(), String> {
        let available = self.stock_for(&request.sku);
        if available < request.quantity {
            self.events
                .push(format!("inventory:rejected:{}", request.sku));
            return Err("inventory is not available".to_string());
        }

        self.stock
            .insert(request.sku.clone(), available - request.quantity);
        self.events.push(format!(
            "inventory:reserved:{}:{}",
            request.sku, request.quantity
        ));
        Ok(())
    }

    fn release_inventory(&mut self, request: &CheckoutRequest) {
        let available = self.stock_for(&request.sku);
        self.stock
            .insert(request.sku.clone(), available + request.quantity);
        self.events.push(format!(
            "inventory:released:{}:{}",
            request.sku, request.quantity
        ));
    }

    fn charge_payment(&mut self, amount_cents: u32) -> Result<(), String> {
        if amount_cents == 0 {
            self.events.push("payment:rejected".to_string());
            return Err("payment amount must be greater than zero".to_string());
        }

        self.events.push(format!("payment:charged:{amount_cents}"));
        Ok(())
    }

    fn create_shipping(&mut self, order_id: &str) -> String {
        let tracking_code = format!("SHIP-{order_id}");
        self.events
            .push(format!("shipping:created:{tracking_code}"));
        tracking_code
    }
}
