use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Message {
    id: String,
    sent: bool,
}

#[derive(Debug, Default)]
/// Tipo publico `RetryStore` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryStore {
    messages: Vec<Message>,
}

impl RetryStore {
    /// Modela la operacion `with pending` dentro del ejemplo del patron.
    pub fn with_pending<const N: usize>(ids: [&str; N]) -> Self {
        Self {
            messages: ids
                .into_iter()
                .map(|id| Message {
                    id: id.to_string(),
                    sent: false,
                })
                .collect(),
        }
    }

    /// Modela la operacion `pending ids` dentro del ejemplo del patron.
    pub fn pending_ids(&self) -> Vec<String> {
        self.messages
            .iter()
            .filter(|message| !message.sent)
            .map(|message| message.id.clone())
            .collect()
    }

    /// Operacion `mark sent` definida por la abstraccion del ejemplo.
    fn mark_sent(&mut self, id: &str) {
        if let Some(message) = self.messages.iter_mut().find(|message| message.id == id) {
            message.sent = true;
        }
    }
}

#[derive(Debug)]
/// Tipo publico `FlakyBroker` usado por el ejemplo para expresar el dominio del patron.
pub struct FlakyBroker {
    fail_first: bool,
    attempts: HashMap<String, usize>,
}

impl FlakyBroker {
    /// Modela la operacion `fail first then succeed` dentro del ejemplo del patron.
    pub fn fail_first_then_succeed() -> Self {
        Self {
            fail_first: true,
            attempts: HashMap::new(),
        }
    }

    /// Modela la operacion `always succeeds` dentro del ejemplo del patron.
    pub fn always_succeeds() -> Self {
        Self {
            fail_first: false,
            attempts: HashMap::new(),
        }
    }

    /// Operacion `publish` definida por la abstraccion del ejemplo.
    fn publish(&mut self, id: &str) -> Result<(), String> {
        let attempts = self.attempts.entry(id.to_string()).or_insert(0);
        *attempts += 1;
        if self.fail_first && *attempts == 1 {
            return Err("temporary broker failure".to_string());
        }
        Ok(())
    }

    /// Modela la operacion `attempts for` dentro del ejemplo del patron.
    pub fn attempts_for(&self, id: &str) -> usize {
        self.attempts.get(id).copied().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `PublishReport` usado por el ejemplo para expresar el dominio del patron.
pub struct PublishReport {
    /// Campo publico `sent` expuesto por el tipo del ejemplo.
    pub sent: usize,
    /// Campo publico `failed` expuesto por el tipo del ejemplo.
    pub failed: usize,
}

/// Tipo publico `RetryPublisher` usado por el ejemplo para expresar el dominio del patron.
pub struct RetryPublisher<'a> {
    broker: &'a mut FlakyBroker,
}

impl<'a> RetryPublisher<'a> {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(broker: &'a mut FlakyBroker) -> Self {
        Self { broker }
    }

    /// Modela la operacion `publish pending` dentro del ejemplo del patron.
    pub fn publish_pending(&mut self, store: &mut RetryStore) -> PublishReport {
        let pending = store.pending_ids();
        let mut report = PublishReport { sent: 0, failed: 0 };
        for id in pending {
            match self.broker.publish(&id) {
                Ok(()) => {
                    store.mark_sent(&id);
                    report.sent += 1;
                }
                Err(_) => {
                    report.failed += 1;
                }
            }
        }
        report
    }
}
