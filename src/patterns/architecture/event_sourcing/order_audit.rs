#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Paid,
    Shipped,
}

impl OrderStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Paid => "paid",
            Self::Shipped => "shipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderEvent {
    Created {
        order_id: String,
        actor: String,
        occurred_at: String,
    },
    StatusChanged {
        order_id: String,
        from: OrderStatus,
        to: OrderStatus,
        actor: String,
        occurred_at: String,
    },
    NoteAdded {
        order_id: String,
        note: String,
        actor: String,
        occurred_at: String,
    },
}

impl OrderEvent {
    pub fn created(
        order_id: impl Into<String>,
        actor: impl Into<String>,
        occurred_at: impl Into<String>,
    ) -> Self {
        Self::Created {
            order_id: order_id.into(),
            actor: actor.into(),
            occurred_at: occurred_at.into(),
        }
    }

    pub fn status_changed(
        order_id: impl Into<String>,
        from: OrderStatus,
        to: OrderStatus,
        actor: impl Into<String>,
        occurred_at: impl Into<String>,
    ) -> Self {
        Self::StatusChanged {
            order_id: order_id.into(),
            from,
            to,
            actor: actor.into(),
            occurred_at: occurred_at.into(),
        }
    }

    pub fn note_added(
        order_id: impl Into<String>,
        note: impl Into<String>,
        actor: impl Into<String>,
        occurred_at: impl Into<String>,
    ) -> Self {
        Self::NoteAdded {
            order_id: order_id.into(),
            note: note.into(),
            actor: actor.into(),
            occurred_at: occurred_at.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    occurred_at: String,
    actor: String,
    description: String,
}

impl AuditEntry {
    pub fn new(
        occurred_at: impl Into<String>,
        actor: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            occurred_at: occurred_at.into(),
            actor: actor.into(),
            description: description.into(),
        }
    }

    fn actor(&self) -> &str {
        &self.actor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderAuditTrail {
    order_id: Option<String>,
    current_status: Option<OrderStatus>,
    entries: Vec<AuditEntry>,
}

impl OrderAuditTrail {
    pub fn from_events(events: &[OrderEvent]) -> Self {
        let mut trail = Self {
            order_id: None,
            current_status: None,
            entries: Vec::new(),
        };

        for event in events {
            trail.apply(event);
        }

        trail
    }

    pub fn entries(&self) -> Vec<AuditEntry> {
        self.entries.clone()
    }

    pub fn entries_by_actor(&self, actor: &str) -> Vec<AuditEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.actor() == actor)
            .cloned()
            .collect()
    }

    pub fn order_id(&self) -> Option<&str> {
        self.order_id.as_deref()
    }

    pub fn current_status(&self) -> Option<OrderStatus> {
        self.current_status
    }

    fn apply(&mut self, event: &OrderEvent) {
        match event {
            OrderEvent::Created {
                order_id,
                actor,
                occurred_at,
            } => {
                self.order_id = Some(order_id.clone());
                self.current_status = Some(OrderStatus::Draft);
                self.entries.push(AuditEntry::new(
                    occurred_at,
                    actor,
                    format!("created order {order_id}"),
                ));
            }
            OrderEvent::StatusChanged {
                order_id,
                from,
                to,
                actor,
                occurred_at,
            } => {
                self.order_id = Some(order_id.clone());
                self.current_status = Some(*to);
                self.entries.push(AuditEntry::new(
                    occurred_at,
                    actor,
                    format!("changed status from {} to {}", from.as_str(), to.as_str()),
                ));
            }
            OrderEvent::NoteAdded {
                order_id,
                note,
                actor,
                occurred_at,
            } => {
                self.order_id = Some(order_id.clone());
                self.entries.push(AuditEntry::new(
                    occurred_at,
                    actor,
                    format!("added note: {note}"),
                ));
            }
        }
    }
}
