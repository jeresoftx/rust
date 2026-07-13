use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EmailRequest` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailRequest {
    recipient: String,
    subject: String,
}

impl EmailRequest {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(recipient: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            recipient: recipient.into(),
            subject: subject.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `EmailStatus` dentro del ejemplo.
pub enum EmailStatus {
    /// Variante `Sent` del estado o error del ejemplo.
    Sent,
    /// Variante `Rejected` del estado o error del ejemplo.
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EmailDelivery` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailDelivery {
    recipient: String,
    subject: String,
    status: EmailStatus,
}

impl EmailDelivery {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        recipient: impl Into<String>,
        subject: impl Into<String>,
        status: EmailStatus,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            subject: subject.into(),
            status,
        }
    }
}

#[derive(Debug)]
/// Tipo publico `EmailActor` usado por el ejemplo para expresar el dominio del patron.
pub struct EmailActor {
    sender: Sender<EmailCommand>,
    handle: JoinHandle<()>,
}

impl EmailActor {
    /// Modela la operacion `start` dentro del ejemplo del patron.
    pub fn start() -> Self {
        let (sender, receiver) = mpsc::channel::<EmailCommand>();
        let handle = thread::spawn(move || {
            let mut history = Vec::new();

            for command in receiver {
                match command {
                    EmailCommand::Send { request, reply } => {
                        let status = if request.recipient.contains('@') {
                            EmailStatus::Sent
                        } else {
                            EmailStatus::Rejected
                        };
                        let delivery =
                            EmailDelivery::new(request.recipient, request.subject, status);

                        history.push(delivery.clone());
                        reply
                            .send(delivery)
                            .expect("email delivery reply should be received");
                    }
                    EmailCommand::History { reply } => {
                        reply
                            .send(history.clone())
                            .expect("email history reply should be received");
                    }
                    EmailCommand::Shutdown => break,
                }
            }
        });

        Self { sender, handle }
    }

    /// Simula el envio de la solicitud ya configurada.
    pub fn send(&self, request: EmailRequest) -> EmailDelivery {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(EmailCommand::Send { request, reply })
            .expect("email actor should be running");

        response
            .recv()
            .expect("email actor should reply to send command")
    }

    /// Modela la operacion `history` dentro del ejemplo del patron.
    pub fn history(&self) -> Vec<EmailDelivery> {
        let (reply, response) = mpsc::channel();
        self.sender
            .send(EmailCommand::History { reply })
            .expect("email actor should be running");

        response
            .recv()
            .expect("email actor should reply with history")
    }

    /// Modela la operacion `sent count` dentro del ejemplo del patron.
    pub fn sent_count(&self) -> usize {
        self.history()
            .iter()
            .filter(|delivery| delivery.status == EmailStatus::Sent)
            .count()
    }

    /// Modela la operacion `rejected count` dentro del ejemplo del patron.
    pub fn rejected_count(&self) -> usize {
        self.history()
            .iter()
            .filter(|delivery| delivery.status == EmailStatus::Rejected)
            .count()
    }

    /// Modela la operacion `shutdown` dentro del ejemplo del patron.
    pub fn shutdown(self) {
        self.sender
            .send(EmailCommand::Shutdown)
            .expect("email actor should receive shutdown command");
        self.handle.join().expect("email actor should stop cleanly");
    }
}

#[derive(Debug)]
enum EmailCommand {
    Send {
        request: EmailRequest,
        reply: Sender<EmailDelivery>,
    },
    History {
        reply: Sender<Vec<EmailDelivery>>,
    },
    Shutdown,
}
