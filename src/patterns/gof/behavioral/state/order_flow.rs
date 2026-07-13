#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderState` dentro del ejemplo.
pub enum OrderState {
    /// Variante `Pending` del estado o error del ejemplo.
    Pending,
    /// Variante `Paid` del estado o error del ejemplo.
    Paid,
    /// Variante `Shipped` del estado o error del ejemplo.
    Shipped,
    /// Variante `Cancelled` del estado o error del ejemplo.
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Order` usado por el ejemplo para expresar el dominio del patron.
pub struct Order {
    id: String,
    state: OrderState,
    history: Vec<String>,
}

impl Order {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            history: vec![format!("created:{id}")],
            id,
            state: OrderState::Pending,
        }
    }

    /// Modela la operacion `pay` dentro del ejemplo del patron.
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

    /// Modela la operacion `ship` dentro del ejemplo del patron.
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

    /// Modela la operacion `cancel` dentro del ejemplo del patron.
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

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> OrderState {
        self.state
    }

    /// Modela la operacion `history` dentro del ejemplo del patron.
    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }
}
