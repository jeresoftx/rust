#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `TicketState` dentro del ejemplo.
pub enum TicketState {
    /// Variante `Open` del estado o error del ejemplo.
    Open,
    /// Variante `Assigned` del estado o error del ejemplo.
    Assigned,
    /// Variante `Resolved` del estado o error del ejemplo.
    Resolved,
    /// Variante `Closed` del estado o error del ejemplo.
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `SupportTicket` usado por el ejemplo para expresar el dominio del patron.
pub struct SupportTicket {
    id: String,
    title: String,
    state: TicketState,
    assignee: Option<String>,
    history: Vec<String>,
}

impl SupportTicket {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let id = id.into();
        let title = title.into();
        Self {
            history: vec![format!("opened:{id}:{title}")],
            id,
            title,
            state: TicketState::Open,
            assignee: None,
        }
    }

    /// Modela la operacion `assign` dentro del ejemplo del patron.
    pub fn assign(&mut self, assignee: impl Into<String>) -> Result<(), String> {
        match self.state {
            TicketState::Open => {
                let assignee = assignee.into();
                self.assignee = Some(assignee.clone());
                self.state = TicketState::Assigned;
                self.history
                    .push(format!("assigned:{}:{assignee}", self.id));
                Ok(())
            }
            TicketState::Closed => Err("closed tickets cannot be assigned".to_string()),
            _ => Err("only open tickets can be assigned".to_string()),
        }
    }

    /// Modela la operacion `resolve` dentro del ejemplo del patron.
    pub fn resolve(&mut self, resolution: impl Into<String>) -> Result<(), String> {
        match self.state {
            TicketState::Assigned => {
                let resolution = resolution.into();
                self.state = TicketState::Resolved;
                self.history
                    .push(format!("resolved:{}:{resolution}", self.id));
                Ok(())
            }
            _ => Err("only assigned tickets can be resolved".to_string()),
        }
    }

    /// Modela la operacion `reopen` dentro del ejemplo del patron.
    pub fn reopen(&mut self, reason: impl Into<String>) -> Result<(), String> {
        match self.state {
            TicketState::Resolved => {
                let reason = reason.into();
                self.state = TicketState::Open;
                self.history.push(format!("reopened:{}:{reason}", self.id));
                Ok(())
            }
            _ => Err("only resolved tickets can be reopened".to_string()),
        }
    }

    /// Modela la operacion `close` dentro del ejemplo del patron.
    pub fn close(&mut self) -> Result<(), String> {
        match self.state {
            TicketState::Resolved => {
                self.state = TicketState::Closed;
                self.history.push(format!("closed:{}", self.id));
                Ok(())
            }
            _ => Err("only resolved tickets can be closed".to_string()),
        }
    }

    /// Modela la operacion `state` dentro del ejemplo del patron.
    pub fn state(&self) -> TicketState {
        self.state
    }

    /// Modela la operacion `history` dentro del ejemplo del patron.
    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    /// Modela la operacion `title` dentro del ejemplo del patron.
    pub fn title(&self) -> &str {
        &self.title
    }
}
