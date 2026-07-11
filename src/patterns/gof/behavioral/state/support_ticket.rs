#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    Open,
    Assigned,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportTicket {
    id: String,
    title: String,
    state: TicketState,
    assignee: Option<String>,
    history: Vec<String>,
}

impl SupportTicket {
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

    pub fn state(&self) -> TicketState {
        self.state
    }

    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
