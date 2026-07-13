//! Replaceable Dependencies.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::layered_architecture::replaceable_dependencies as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `domain` dentro del catalogo de patrones.
pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `SupportTicket` usado por el ejemplo para expresar el dominio del patron.
    pub struct SupportTicket {
        id: String,
        title: String,
        priority: TicketPriority,
        assigned_agent: Option<String>,
    }

    impl SupportTicket {
        /// Modela la operacion `open` dentro del ejemplo del patron.
        pub fn open(
            id: impl Into<String>,
            title: impl Into<String>,
            priority: TicketPriority,
        ) -> Result<Self, TicketError> {
            let title = title.into();

            if title.trim().is_empty() {
                return Err(TicketError::TitleRequired);
            }

            Ok(Self {
                id: id.into(),
                title,
                priority,
                assigned_agent: None,
            })
        }

        /// Modela la operacion `assign to` dentro del ejemplo del patron.
        pub fn assign_to(&mut self, agent: impl Into<String>) {
            self.assigned_agent = Some(agent.into());
        }

        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `priority` dentro del ejemplo del patron.
        pub fn priority(&self) -> TicketPriority {
            self.priority
        }

        /// Modela la operacion `assigned agent` dentro del ejemplo del patron.
        pub fn assigned_agent(&self) -> Option<&str> {
            self.assigned_agent.as_deref()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `TicketPriority` dentro del ejemplo.
    pub enum TicketPriority {
        /// Variante `Medium` del estado o error del ejemplo.
        Medium,
        /// Variante `High` del estado o error del ejemplo.
        High,
        /// Variante `Urgent` del estado o error del ejemplo.
        Urgent,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Conjunto de estados o errores publicos de `TicketError` dentro del ejemplo.
    pub enum TicketError {
        /// Variante `TitleRequired` del estado o error del ejemplo.
        TitleRequired,
        /// Variante `UnknownPriority` del estado o error del ejemplo.
        UnknownPriority,
    }

    impl TicketError {
        /// Modela la operacion `message` dentro del ejemplo del patron.
        pub fn message(&self) -> &'static str {
            match self {
                Self::TitleRequired => "ticket title is required",
                Self::UnknownPriority => "ticket priority is unknown",
            }
        }
    }
}

/// Modulo del ejemplo `application` dentro del catalogo de patrones.
pub mod application {
    use super::domain::{SupportTicket, TicketError, TicketPriority};

    /// Contrato publico `TicketRepository` que desacopla las piezas del ejemplo.
    pub trait TicketRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, ticket: SupportTicket);
        /// Operacion `saved count` definida por la abstraccion del ejemplo.
        fn saved_count(&self) -> usize;
    }

    /// Contrato publico `TicketNotifier` que desacopla las piezas del ejemplo.
    pub trait TicketNotifier {
        /// Operacion `notify assigned` definida por la abstraccion del ejemplo.
        fn notify_assigned(&mut self, ticket_id: &str, agent: &str);
        /// Operacion `notifications` definida por la abstraccion del ejemplo.
        fn notifications(&self) -> Vec<String>;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `AssignTicketCommand` usado por el ejemplo para expresar el dominio del patron.
    pub struct AssignTicketCommand {
        id: String,
        title: String,
        priority: TicketPriority,
        agent: String,
    }

    impl AssignTicketCommand {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(
            id: impl Into<String>,
            title: impl Into<String>,
            priority: TicketPriority,
            agent: impl Into<String>,
        ) -> Self {
            Self {
                id: id.into(),
                title: title.into(),
                priority,
                agent: agent.into(),
            }
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `AssignTicket` usado por el ejemplo para expresar el dominio del patron.
    pub struct AssignTicket<R, N> {
        repository: R,
        notifier: N,
    }

    impl<R, N> AssignTicket<R, N>
    where
        R: TicketRepository,
        N: TicketNotifier,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(repository: R, notifier: N) -> Self {
            Self {
                repository,
                notifier,
            }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(
            &mut self,
            command: AssignTicketCommand,
        ) -> Result<SupportTicket, TicketError> {
            let mut ticket = SupportTicket::open(command.id, command.title, command.priority)?;
            ticket.assign_to(command.agent);

            let ticket_id = ticket.id().to_string();
            let agent = ticket
                .assigned_agent()
                .expect("ticket should be assigned")
                .to_string();

            self.repository.save(ticket.clone());
            self.notifier.notify_assigned(&ticket_id, &agent);

            Ok(ticket)
        }

        /// Modela la operacion `saved count` dentro del ejemplo del patron.
        pub fn saved_count(&self) -> usize {
            self.repository.saved_count()
        }

        /// Modela la operacion `notifications` dentro del ejemplo del patron.
        pub fn notifications(&self) -> Vec<String> {
            self.notifier.notifications()
        }
    }
}

/// Modulo del ejemplo `infrastructure` dentro del catalogo de patrones.
pub mod infrastructure {
    use super::application::{TicketNotifier, TicketRepository};
    use super::domain::SupportTicket;

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `InMemoryTicketRepository` usado por el ejemplo para expresar el dominio del patron.
    pub struct InMemoryTicketRepository {
        tickets: Vec<SupportTicket>,
    }

    impl TicketRepository for InMemoryTicketRepository {
        /// Operacion `save` definida por la abstraccion del ejemplo.
        fn save(&mut self, ticket: SupportTicket) {
            self.tickets.push(ticket);
        }

        /// Operacion `saved count` definida por la abstraccion del ejemplo.
        fn saved_count(&self) -> usize {
            self.tickets.len()
        }
    }

    #[derive(Debug, Clone, Default)]
    /// Tipo publico `RecordingTicketNotifier` usado por el ejemplo para expresar el dominio del patron.
    pub struct RecordingTicketNotifier {
        notifications: Vec<String>,
    }

    impl TicketNotifier for RecordingTicketNotifier {
        /// Operacion `notify assigned` definida por la abstraccion del ejemplo.
        fn notify_assigned(&mut self, ticket_id: &str, agent: &str) {
            self.notifications
                .push(format!("{ticket_id} assigned to {agent}"));
        }

        /// Operacion `notifications` definida por la abstraccion del ejemplo.
        fn notifications(&self) -> Vec<String> {
            self.notifications.clone()
        }
    }
}

/// Modulo del ejemplo `presentation` dentro del catalogo de patrones.
pub mod presentation {
    use super::application::{AssignTicket, AssignTicketCommand, TicketNotifier, TicketRepository};
    use super::domain::{SupportTicket, TicketError, TicketPriority};

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `AssignTicketRequest` usado por el ejemplo para expresar el dominio del patron.
    pub struct AssignTicketRequest {
        id: String,
        title: String,
        priority: String,
        agent: String,
    }

    impl AssignTicketRequest {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(
            id: impl Into<String>,
            title: impl Into<String>,
            priority: impl Into<String>,
            agent: impl Into<String>,
        ) -> Self {
            Self {
                id: id.into(),
                title: title.into(),
                priority: priority.into(),
                agent: agent.into(),
            }
        }

        /// Operacion `into command` definida por la abstraccion del ejemplo.
        fn into_command(self) -> Result<AssignTicketCommand, TicketError> {
            let priority = match self.priority.as_str() {
                "medium" => TicketPriority::Medium,
                "high" => TicketPriority::High,
                "urgent" => TicketPriority::Urgent,
                _ => return Err(TicketError::UnknownPriority),
            };

            Ok(AssignTicketCommand::new(
                self.id, self.title, priority, self.agent,
            ))
        }
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `TicketController` usado por el ejemplo para expresar el dominio del patron.
    pub struct TicketController<R, N> {
        assign_ticket: AssignTicket<R, N>,
    }

    impl<R, N> TicketController<R, N>
    where
        R: TicketRepository,
        N: TicketNotifier,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(assign_ticket: AssignTicket<R, N>) -> Self {
            Self { assign_ticket }
        }

        /// Modela la operacion `assign ticket` dentro del ejemplo del patron.
        pub fn assign_ticket(&mut self, request: AssignTicketRequest) -> AssignTicketResponse {
            let command = match request.into_command() {
                Ok(command) => command,
                Err(error) => return AssignTicketResponse::bad_request(error.message()),
            };

            match self.assign_ticket.execute(command) {
                Ok(ticket) => AssignTicketResponse::created(AssignedTicketDto::from(ticket)),
                Err(error) => AssignTicketResponse::bad_request(error.message()),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `AssignTicketResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct AssignTicketResponse {
        status_code: u16,
        body: Option<AssignedTicketDto>,
        error: Option<String>,
    }

    impl AssignTicketResponse {
        /// Operacion `created` definida por la abstraccion del ejemplo.
        fn created(body: AssignedTicketDto) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        /// Operacion `bad request` definida por la abstraccion del ejemplo.
        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &AssignedTicketDto {
            self.body.as_ref().expect("response should contain a body")
        }

        /// Modela la operacion `error` dentro del ejemplo del patron.
        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `AssignedTicketDto` usado por el ejemplo para expresar el dominio del patron.
    pub struct AssignedTicketDto {
        ticket_id: String,
        agent: String,
    }

    impl AssignedTicketDto {
        /// Modela la operacion `ticket id` dentro del ejemplo del patron.
        pub fn ticket_id(&self) -> &str {
            &self.ticket_id
        }

        /// Devuelve un resumen legible del estado actual.
        pub fn summary(&self) -> String {
            format!("{} assigned to {}", self.ticket_id, self.agent)
        }
    }

    impl From<SupportTicket> for AssignedTicketDto {
        /// Operacion `from` definida por la abstraccion del ejemplo.
        fn from(ticket: SupportTicket) -> Self {
            Self {
                ticket_id: ticket.id().to_string(),
                agent: ticket
                    .assigned_agent()
                    .expect("ticket should be assigned")
                    .to_string(),
            }
        }
    }
}
