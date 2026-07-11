#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TicketKind {
    PasswordReset,
    SoftwareBug,
    InfrastructureIncident,
}

pub struct SupportTicket {
    id: String,
    kind: TicketKind,
}

impl SupportTicket {
    pub fn new(id: impl Into<String>, kind: TicketKind) -> Self {
        Self {
            id: id.into(),
            kind,
        }
    }
}

pub struct SupportChain {
    handlers: Vec<SupportHandler>,
}

impl SupportChain {
    pub fn resolve(&self, ticket: &SupportTicket) -> String {
        self.handlers
            .iter()
            .find_map(|handler| handler.try_resolve(ticket))
            .unwrap_or_else(|| format!("{} could not be handled", ticket.id))
    }
}

impl Default for SupportChain {
    fn default() -> Self {
        Self {
            handlers: vec![
                SupportHandler::new(1, TicketKind::PasswordReset, "password reset"),
                SupportHandler::new(2, TicketKind::SoftwareBug, "software bug"),
                SupportHandler::new(
                    3,
                    TicketKind::InfrastructureIncident,
                    "infrastructure incident",
                ),
            ],
        }
    }
}

struct SupportHandler {
    level: u8,
    handles: TicketKind,
    label: &'static str,
}

impl SupportHandler {
    fn new(level: u8, handles: TicketKind, label: &'static str) -> Self {
        Self {
            level,
            handles,
            label,
        }
    }

    fn try_resolve(&self, ticket: &SupportTicket) -> Option<String> {
        if ticket.kind == self.handles {
            Some(format!(
                "{} handled by level {}: {}",
                ticket.id, self.level, self.label
            ))
        } else {
            None
        }
    }
}
