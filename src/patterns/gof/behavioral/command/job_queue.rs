use std::collections::VecDeque;

/// Conjunto de estados o errores publicos de `JobCommand` dentro del ejemplo.
pub enum JobCommand {
    /// Variante `SendEmail` del estado o error del ejemplo.
    SendEmail {
        /// Valor publico `to` asociado a la variante `SendEmail`.
        to: String,
        /// Valor publico `template` asociado a la variante `SendEmail`.
        template: String,
    },
    /// Variante `GenerateInvoice` del estado o error del ejemplo.
    GenerateInvoice {
        /// Valor publico `order_id` asociado a la variante `GenerateInvoice`.
        order_id: u64,
    },
}

impl JobCommand {
    /// Operacion `serialize` definida por la abstraccion del ejemplo.
    fn serialize(&self) -> String {
        match self {
            Self::SendEmail { to, template } => format!("email|{to}|{template}"),
            Self::GenerateInvoice { order_id } => format!("invoice|{order_id}"),
        }
    }

    /// Operacion `execute` definida por la abstraccion del ejemplo.
    fn execute(&self) -> String {
        match self {
            Self::SendEmail { to, template } => format!("sent {template} email to {to}"),
            Self::GenerateInvoice { order_id } => {
                format!("generated invoice for order {order_id}")
            }
        }
    }
}

/// Tipo publico `JobQueue` usado por el ejemplo para expresar el dominio del patron.
pub struct JobQueue {
    pending: VecDeque<JobCommand>,
}

impl JobQueue {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }

    /// Modela la operacion `enqueue` dentro del ejemplo del patron.
    pub fn enqueue(&mut self, command: JobCommand) {
        self.pending.push_back(command);
    }

    /// Modela la operacion `serialize pending` dentro del ejemplo del patron.
    pub fn serialize_pending(&self) -> Vec<String> {
        self.pending.iter().map(JobCommand::serialize).collect()
    }

    /// Modela la operacion `run all` dentro del ejemplo del patron.
    pub fn run_all(&mut self) -> Vec<String> {
        let mut log = Vec::new();

        while let Some(command) = self.pending.pop_front() {
            log.push(command.execute());
        }

        log
    }
}

impl Default for JobQueue {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}
