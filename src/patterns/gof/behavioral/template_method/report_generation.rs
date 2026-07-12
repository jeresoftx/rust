#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportDocument {
    lines: Vec<String>,
}

impl ReportDocument {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    pub fn lines(&self) -> Vec<&str> {
        self.lines.iter().map(String::as_str).collect()
    }
}

pub trait ReportTemplate {
    fn title(&self) -> &'static str;
    fn summary(&self) -> &'static str;
    fn sections(&self) -> Vec<String>;

    fn footer(&self) -> String {
        "generated_by=reporting".to_string()
    }

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
pub struct SalesReport {
    revenue_cents: u32,
    orders: u32,
}

impl SalesReport {
    pub fn new(revenue_cents: u32, orders: u32) -> Self {
        Self {
            revenue_cents,
            orders,
        }
    }
}

impl ReportTemplate for SalesReport {
    fn title(&self) -> &'static str {
        "Sales report"
    }

    fn summary(&self) -> &'static str {
        "Monthly sales performance"
    }

    fn sections(&self) -> Vec<String> {
        vec![
            format!("revenue_cents={}", self.revenue_cents),
            format!("orders={}", self.orders),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidentReport {
    service: String,
    severity: String,
    duration_minutes: u32,
    owner: Option<String>,
}

impl IncidentReport {
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

    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }
}

impl ReportTemplate for IncidentReport {
    fn title(&self) -> &'static str {
        "Incident report"
    }

    fn summary(&self) -> &'static str {
        "Operational incident review"
    }

    fn sections(&self) -> Vec<String> {
        vec![
            format!("service={}", self.service),
            format!("severity={}", self.severity),
            format!("duration_minutes={}", self.duration_minutes),
        ]
    }

    fn footer(&self) -> String {
        self.owner
            .as_ref()
            .map(|owner| format!("owner={owner}"))
            .unwrap_or_else(|| "generated_by=reporting".to_string())
    }
}
