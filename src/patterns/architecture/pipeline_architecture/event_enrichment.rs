#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `RawEvent` usado por el ejemplo para expresar el dominio del patron.
pub struct RawEvent {
    event_type: String,
    actor: String,
    payload: String,
    priority: u8,
}

impl RawEvent {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        event_type: impl Into<String>,
        actor: impl Into<String>,
        payload: impl Into<String>,
        priority: u8,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            actor: actor.into(),
            payload: payload.into(),
            priority,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EnrichedEvent` usado por el ejemplo para expresar el dominio del patron.
pub struct EnrichedEvent {
    event_type: String,
    actor: String,
    payload: String,
    priority: u8,
    category: String,
}

impl EnrichedEvent {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        event_type: impl Into<String>,
        actor: impl Into<String>,
        payload: impl Into<String>,
        priority: u8,
        category: impl Into<String>,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            actor: actor.into(),
            payload: payload.into(),
            priority,
            category: category.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `EventReport` usado por el ejemplo para expresar el dominio del patron.
pub struct EventReport {
    accepted: Vec<EnrichedEvent>,
    rejected: Vec<String>,
}

impl EventReport {
    /// Modela la operacion `accepted` dentro del ejemplo del patron.
    pub fn accepted(&self) -> Vec<EnrichedEvent> {
        self.accepted.clone()
    }

    /// Modela la operacion `rejected` dentro del ejemplo del patron.
    pub fn rejected(&self) -> Vec<String> {
        self.rejected.clone()
    }
}

#[derive(Debug)]
/// Tipo publico `EventPipeline` usado por el ejemplo para expresar el dominio del patron.
pub struct EventPipeline {
    allowed_actors: Vec<String>,
}

impl EventPipeline {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(allowed_actors: Vec<String>) -> Self {
        Self { allowed_actors }
    }

    /// Modela la operacion `process` dentro del ejemplo del patron.
    pub fn process(&self, events: Vec<RawEvent>) -> EventReport {
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();

        for event in events {
            match self.process_one(event) {
                Ok(event) => accepted.push(event),
                Err(reason) => rejected.push(reason),
            }
        }

        EventReport { accepted, rejected }
    }

    /// Operacion `process one` definida por la abstraccion del ejemplo.
    fn process_one(&self, event: RawEvent) -> Result<EnrichedEvent, String> {
        let event = validate_actor(event, &self.allowed_actors)?;
        let event = filter_low_priority_noise(event)?;
        let event = normalize_event_type(event);
        Ok(enrich_category(event))
    }
}

/// Operacion `validate actor` definida por la abstraccion del ejemplo.
fn validate_actor(event: RawEvent, allowed_actors: &[String]) -> Result<RawEvent, String> {
    if event.actor.is_empty() {
        return Err("actor requerido".to_string());
    }

    if !allowed_actors.iter().any(|actor| actor == &event.actor) {
        return Err(format!("actor no permitido: {}", event.actor));
    }

    Ok(event)
}

/// Operacion `filter low priority noise` definida por la abstraccion del ejemplo.
fn filter_low_priority_noise(event: RawEvent) -> Result<RawEvent, String> {
    if event.priority == 0 {
        return Err("evento de baja prioridad".to_string());
    }

    Ok(event)
}

/// Operacion `normalize event type` definida por la abstraccion del ejemplo.
fn normalize_event_type(mut event: RawEvent) -> RawEvent {
    event.event_type = event.event_type.to_uppercase();
    event
}

/// Operacion `enrich category` definida por la abstraccion del ejemplo.
fn enrich_category(event: RawEvent) -> EnrichedEvent {
    let category = if event.event_type.starts_with("LOGIN.") {
        "security"
    } else if event.event_type.starts_with("INVOICE.") {
        "billing"
    } else {
        "general"
    };

    EnrichedEvent::new(
        event.event_type,
        event.actor,
        event.payload,
        event.priority,
        category,
    )
}
