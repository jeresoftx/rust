#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `ReportDocument` usado por el ejemplo para expresar el dominio del patron.
pub struct ReportDocument {
    lines: Vec<String>,
}

impl ReportDocument {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    /// Modela la operacion `lines` dentro del ejemplo del patron.
    pub fn lines(&self) -> Vec<&str> {
        self.lines.iter().map(String::as_str).collect()
    }
}

/// Contrato publico `ReportTemplate` que desacopla las piezas del ejemplo.
pub trait ReportTemplate {
    /// Operacion `title` definida por la abstraccion del ejemplo.
    fn title(&self) -> &'static str;
    /// Operacion `summary` definida por la abstraccion del ejemplo.
    fn summary(&self) -> &'static str;
    /// Operacion `sections` definida por la abstraccion del ejemplo.
    fn sections(&self) -> Vec<String>;

    /// Operacion `footer` definida por la abstraccion del ejemplo.
    fn footer(&self) -> String {
        "generated_by=reporting".to_string()
    }

    /// Operacion `generate` definida por la abstraccion del ejemplo.
    fn generate(&self) -> ReportDocument {
        let mut lines = vec![
            format!("cover:{}", self.title()),
            format!("summary:{}", self.summary()),
        ];

        lines.extend(
            self.sections()
                .into_iter()
                .map(|section| format!("section:{section}")),
        );
        lines.push(format!("footer:{}", self.footer()));

        ReportDocument::new(lines)
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `SalesReport` usado por el ejemplo para expresar el dominio del patron.
pub struct SalesReport {
    revenue_cents: u32,
    orders: u32,
}

impl SalesReport {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(revenue_cents: u32, orders: u32) -> Self {
        Self {
            revenue_cents,
            orders,
        }
    }
}

impl ReportTemplate for SalesReport {
    /// Operacion `title` definida por la abstraccion del ejemplo.
    fn title(&self) -> &'static str {
        "Sales report"
    }

    /// Operacion `summary` definida por la abstraccion del ejemplo.
    fn summary(&self) -> &'static str {
        "Monthly sales performance"
    }

    /// Operacion `sections` definida por la abstraccion del ejemplo.
    fn sections(&self) -> Vec<String> {
        vec![
            format!("revenue_cents={}", self.revenue_cents),
            format!("orders={}", self.orders),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `IncidentReport` usado por el ejemplo para expresar el dominio del patron.
pub struct IncidentReport {
    service: String,
    severity: String,
    duration_minutes: u32,
    owner: Option<String>,
}

impl IncidentReport {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(
        service: impl Into<String>,
        severity: impl Into<String>,
        duration_minutes: u32,
    ) -> Self {
        Self {
            service: service.into(),
            severity: severity.into(),
            duration_minutes,
            owner: None,
        }
    }

    /// Modela la operacion `with owner` dentro del ejemplo del patron.
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }
}

impl ReportTemplate for IncidentReport {
    /// Operacion `title` definida por la abstraccion del ejemplo.
    fn title(&self) -> &'static str {
        "Incident report"
    }

    /// Operacion `summary` definida por la abstraccion del ejemplo.
    fn summary(&self) -> &'static str {
        "Operational incident review"
    }

    /// Operacion `sections` definida por la abstraccion del ejemplo.
    fn sections(&self) -> Vec<String> {
        vec![
            format!("service={}", self.service),
            format!("severity={}", self.severity),
            format!("duration_minutes={}", self.duration_minutes),
        ]
    }

    /// Operacion `footer` definida por la abstraccion del ejemplo.
    fn footer(&self) -> String {
        self.owner
            .as_ref()
            .map(|owner| format!("owner={owner}"))
            .unwrap_or_else(|| "generated_by=reporting".to_string())
    }
}
