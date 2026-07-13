#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderStatus` dentro del ejemplo.
pub enum OrderStatus {
    /// Variante `Draft` del estado o error del ejemplo.
    Draft,
    /// Variante `Paid` del estado o error del ejemplo.
    Paid,
    /// Variante `Shipped` del estado o error del ejemplo.
    Shipped,
}

impl OrderStatus {
    /// Operacion `as str` definida por la abstraccion del ejemplo.
    fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Paid => "paid",
            Self::Shipped => "shipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `OrderEvent` dentro del ejemplo.
pub enum OrderEvent {
    /// Variante `Created` del estado o error del ejemplo.
    Created {
        /// Variante `order_id` del estado o error del ejemplo.
        order_id: String,
        /// Variante `actor` del estado o error del ejemplo.
        actor: String,
        /// Variante `occurred_at` del estado o error del ejemplo.
        occurred_at: String,
    },
    /// Variante `StatusChanged` del estado o error del ejemplo.
    StatusChanged {
        /// Valor publico `order_id` asociado a la variante del enum.
        order_id: String,
        /// Valor publico `from` asociado a la variante del enum.
        from: OrderStatus,
        /// Valor publico `to` asociado a la variante del enum.
        to: OrderStatus,
        /// Valor publico `actor` asociado a la variante del enum.
        actor: String,
        /// Valor publico `occurred_at` asociado a la variante del enum.
        occurred_at: String,
    },
    /// Variante `NoteAdded` del estado o error del ejemplo.
    NoteAdded {
        /// Valor publico `order_id` asociado a la variante del enum.
        order_id: String,
        /// Valor publico `note` asociado a la variante del enum.
        note: String,
        /// Valor publico `actor` asociado a la variante del enum.
        actor: String,
        /// Valor publico `occurred_at` asociado a la variante del enum.
        occurred_at: String,
    },
}

impl OrderEvent {
    /// Modela la operacion `created` dentro del ejemplo del patron.
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

    /// Modela la operacion `status changed` dentro del ejemplo del patron.
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

    /// Modela la operacion `note added` dentro del ejemplo del patron.
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
/// Tipo publico `AuditEntry` usado por el ejemplo para expresar el dominio del patron.
pub struct AuditEntry {
    occurred_at: String,
    actor: String,
    description: String,
}

impl AuditEntry {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Operacion `actor` definida por la abstraccion del ejemplo.
    fn actor(&self) -> &str {
        &self.actor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `OrderAuditTrail` usado por el ejemplo para expresar el dominio del patron.
pub struct OrderAuditTrail {
    order_id: Option<String>,
    current_status: Option<OrderStatus>,
    entries: Vec<AuditEntry>,
}

impl OrderAuditTrail {
    /// Modela la operacion `from events` dentro del ejemplo del patron.
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

    /// Modela la operacion `entries` dentro del ejemplo del patron.
    pub fn entries(&self) -> Vec<AuditEntry> {
        self.entries.clone()
    }

    /// Modela la operacion `entries by actor` dentro del ejemplo del patron.
    pub fn entries_by_actor(&self, actor: &str) -> Vec<AuditEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.actor() == actor)
            .cloned()
            .collect()
    }

    /// Modela la operacion `order id` dentro del ejemplo del patron.
    pub fn order_id(&self) -> Option<&str> {
        self.order_id.as_deref()
    }

    /// Modela la operacion `current status` dentro del ejemplo del patron.
    pub fn current_status(&self) -> Option<OrderStatus> {
        self.current_status
    }

    /// Operacion `apply` definida por la abstraccion del ejemplo.
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
