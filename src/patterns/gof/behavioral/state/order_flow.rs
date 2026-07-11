#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: String,
    state: OrderState,
    history: Vec<String>,
}

impl Order {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            history: vec![format!("created:{id}")],
            id,
            state: OrderState::Pending,
        }
    }

    pub fn pay(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Pending => {
                self.state = OrderState::Paid;
                self.history.push(format!("paid:{}", self.id));
                Ok(())
            }
            OrderState::Cancelled => Err("cancelled orders cannot be paid".to_string()),
            _ => Err("only pending orders can be paid".to_string()),
        }
    }

    pub fn ship(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Paid => {
                self.state = OrderState::Shipped;
                self.history.push(format!("shipped:{}", self.id));
                Ok(())
            }
            _ => Err("only paid orders can be shipped".to_string()),
        }
    }

    pub fn cancel(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Pending | OrderState::Paid => {
                self.state = OrderState::Cancelled;
                self.history.push(format!("cancelled:{}", self.id));
                Ok(())
            }
            _ => Err("shipped orders cannot be cancelled".to_string()),
        }
    }

    pub fn state(&self) -> OrderState {
        self.state
    }

    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }
}
