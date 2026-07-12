#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawEvent {
    event_type: String,
    actor: String,
    payload: String,
    priority: u8,
}

impl RawEvent {
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
pub struct EnrichedEvent {
    event_type: String,
    actor: String,
    payload: String,
    priority: u8,
    category: String,
}

impl EnrichedEvent {
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
pub struct EventReport {
    accepted: Vec<EnrichedEvent>,
    rejected: Vec<String>,
}

impl EventReport {
    pub fn accepted(&self) -> Vec<EnrichedEvent> {
        self.accepted.clone()
    }

    pub fn rejected(&self) -> Vec<String> {
        self.rejected.clone()
    }
}

#[derive(Debug)]
pub struct EventPipeline {
    allowed_actors: Vec<String>,
}

impl EventPipeline {
    pub fn new(allowed_actors: Vec<String>) -> Self {
        Self { allowed_actors }
    }

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

    fn process_one(&self, event: RawEvent) -> Result<EnrichedEvent, String> {
        let event = validate_actor(event, &self.allowed_actors)?;
        let event = filter_low_priority_noise(event)?;
        let event = normalize_event_type(event);
        Ok(enrich_category(event))
    }
}

fn validate_actor(event: RawEvent, allowed_actors: &[String]) -> Result<RawEvent, String> {
    if event.actor.is_empty() {
        return Err("actor requerido".to_string());
    }

    if !allowed_actors.iter().any(|actor| actor == &event.actor) {
        return Err(format!("actor no permitido: {}", event.actor));
    }

    Ok(event)
}

fn filter_low_priority_noise(event: RawEvent) -> Result<RawEvent, String> {
    if event.priority == 0 {
        return Err("evento de baja prioridad".to_string());
    }

    Ok(event)
}

fn normalize_event_type(mut event: RawEvent) -> RawEvent {
    event.event_type = event.event_type.to_uppercase();
    event
}

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
