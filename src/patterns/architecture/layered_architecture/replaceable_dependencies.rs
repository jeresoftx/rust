pub mod domain {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SupportTicket {
        id: String,
        title: String,
        priority: TicketPriority,
        assigned_agent: Option<String>,
    }

    impl SupportTicket {
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

        pub fn assign_to(&mut self, agent: impl Into<String>) {
            self.assigned_agent = Some(agent.into());
        }

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn priority(&self) -> TicketPriority {
            self.priority
        }

        pub fn assigned_agent(&self) -> Option<&str> {
            self.assigned_agent.as_deref()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TicketPriority {
        Medium,
        High,
        Urgent,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TicketError {
        TitleRequired,
        UnknownPriority,
    }

    impl TicketError {
        pub fn message(&self) -> &'static str {
            match self {
                Self::TitleRequired => "ticket title is required",
                Self::UnknownPriority => "ticket priority is unknown",
            }
        }
    }
}

pub mod application {
    use super::domain::{SupportTicket, TicketError, TicketPriority};

    pub trait TicketRepository {
        fn save(&mut self, ticket: SupportTicket);
        fn saved_count(&self) -> usize;
    }

    pub trait TicketNotifier {
        fn notify_assigned(&mut self, ticket_id: &str, agent: &str);
        fn notifications(&self) -> Vec<String>;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AssignTicketCommand {
        id: String,
        title: String,
        priority: TicketPriority,
        agent: String,
    }

    impl AssignTicketCommand {
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
    pub struct AssignTicket<R, N> {
        repository: R,
        notifier: N,
    }

    impl<R, N> AssignTicket<R, N>
    where
        R: TicketRepository,
        N: TicketNotifier,
    {
        pub fn new(repository: R, notifier: N) -> Self {
            Self {
                repository,
                notifier,
            }
        }

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

        pub fn saved_count(&self) -> usize {
            self.repository.saved_count()
        }

        pub fn notifications(&self) -> Vec<String> {
            self.notifier.notifications()
        }
    }
}

pub mod infrastructure {
    use super::application::{TicketNotifier, TicketRepository};
    use super::domain::SupportTicket;

    #[derive(Debug, Clone, Default)]
    pub struct InMemoryTicketRepository {
        tickets: Vec<SupportTicket>,
    }

    impl TicketRepository for InMemoryTicketRepository {
        fn save(&mut self, ticket: SupportTicket) {
            self.tickets.push(ticket);
        }

        fn saved_count(&self) -> usize {
            self.tickets.len()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct RecordingTicketNotifier {
        notifications: Vec<String>,
    }

    impl TicketNotifier for RecordingTicketNotifier {
        fn notify_assigned(&mut self, ticket_id: &str, agent: &str) {
            self.notifications
                .push(format!("{ticket_id} assigned to {agent}"));
        }

        fn notifications(&self) -> Vec<String> {
            self.notifications.clone()
        }
    }
}

pub mod presentation {
    use super::application::{AssignTicket, AssignTicketCommand, TicketNotifier, TicketRepository};
    use super::domain::{SupportTicket, TicketError, TicketPriority};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AssignTicketRequest {
        id: String,
        title: String,
        priority: String,
        agent: String,
    }

    impl AssignTicketRequest {
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
    pub struct TicketController<R, N> {
        assign_ticket: AssignTicket<R, N>,
    }

    impl<R, N> TicketController<R, N>
    where
        R: TicketRepository,
        N: TicketNotifier,
    {
        pub fn new(assign_ticket: AssignTicket<R, N>) -> Self {
            Self { assign_ticket }
        }

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
    pub struct AssignTicketResponse {
        status_code: u16,
        body: Option<AssignedTicketDto>,
        error: Option<String>,
    }

    impl AssignTicketResponse {
        fn created(body: AssignedTicketDto) -> Self {
            Self {
                status_code: 201,
                body: Some(body),
                error: None,
            }
        }

        fn bad_request(message: &str) -> Self {
            Self {
                status_code: 400,
                body: None,
                error: Some(message.to_string()),
            }
        }

        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &AssignedTicketDto {
            self.body.as_ref().expect("response should contain a body")
        }

        pub fn error(&self) -> Option<&str> {
            self.error.as_deref()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AssignedTicketDto {
        ticket_id: String,
        agent: String,
    }

    impl AssignedTicketDto {
        pub fn ticket_id(&self) -> &str {
            &self.ticket_id
        }

        pub fn summary(&self) -> String {
            format!("{} assigned to {}", self.ticket_id, self.agent)
        }
    }

    impl From<SupportTicket> for AssignedTicketDto {
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
