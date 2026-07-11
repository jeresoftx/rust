use std::collections::VecDeque;

pub enum JobCommand {
    SendEmail { to: String, template: String },
    GenerateInvoice { order_id: u64 },
}

impl JobCommand {
    fn serialize(&self) -> String {
        match self {
            Self::SendEmail { to, template } => format!("email|{to}|{template}"),
            Self::GenerateInvoice { order_id } => format!("invoice|{order_id}"),
        }
    }

    fn execute(&self) -> String {
        match self {
            Self::SendEmail { to, template } => format!("sent {template} email to {to}"),
            Self::GenerateInvoice { order_id } => {
                format!("generated invoice for order {order_id}")
            }
        }
    }
}

pub struct JobQueue {
    pending: VecDeque<JobCommand>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, command: JobCommand) {
        self.pending.push_back(command);
    }

    pub fn serialize_pending(&self) -> Vec<String> {
        self.pending.iter().map(JobCommand::serialize).collect()
    }

    pub fn run_all(&mut self) -> Vec<String> {
        let mut log = Vec::new();

        while let Some(command) = self.pending.pop_front() {
            log.push(command.execute());
        }

        log
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}
