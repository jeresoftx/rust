#[derive(Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `TicketKind` dentro del ejemplo.
pub enum TicketKind {
    /// Variante `PasswordReset` del estado o error del ejemplo.
    PasswordReset,
    /// Variante `SoftwareBug` del estado o error del ejemplo.
    SoftwareBug,
    /// Variante `InfrastructureIncident` del estado o error del ejemplo.
    InfrastructureIncident,
}

/// Tipo publico `SupportTicket` usado por el ejemplo para expresar el dominio del patron.
pub struct SupportTicket {
    id: String,
    kind: TicketKind,
}

impl SupportTicket {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(id: impl Into<String>, kind: TicketKind) -> Self {
        Self {
            id: id.into(),
            kind,
        }
    }
}

/// Tipo publico `SupportChain` usado por el ejemplo para expresar el dominio del patron.
pub struct SupportChain {
    handlers: Vec<SupportHandler>,
}

impl SupportChain {
    /// Modela la operacion `resolve` dentro del ejemplo del patron.
    pub fn resolve(&self, ticket: &SupportTicket) -> String {
        self.handlers
            .iter()
            .find_map(|handler| handler.try_resolve(ticket))
            .unwrap_or_else(|| format!("{} could not be handled", ticket.id))
    }
}

impl Default for SupportChain {
    /// Operacion `default` definida por la abstraccion del ejemplo.
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
    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(level: u8, handles: TicketKind, label: &'static str) -> Self {
        Self {
            level,
            handles,
            label,
        }
    }

    /// Operacion `try resolve` definida por la abstraccion del ejemplo.
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
