#[derive(Clone, Debug, PartialEq, Eq)]
/// Tipo publico `ReportTemplate` usado por el ejemplo para expresar el dominio del patron.
pub struct ReportTemplate {
    title: String,
    customer: String,
    period: String,
    sections: Vec<String>,
}

impl ReportTemplate {
    /// Modela la operacion `standard financial` dentro del ejemplo del patron.
    pub fn standard_financial() -> Self {
        Self {
            title: "Financial Summary".to_string(),
            customer: "<customer>".to_string(),
            period: "<period>".to_string(),
            sections: vec![
                "Revenue".to_string(),
                "Costs".to_string(),
                "Margin".to_string(),
            ],
        }
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "{} for {} [{}] sections={}",
            self.title,
            self.customer,
            self.period,
            self.sections.join(",")
        )
    }
}

/// Modela la operacion `monthly report for customer` dentro del ejemplo del patron.
pub fn monthly_report_for_customer(
    template: &ReportTemplate,
    customer: &str,
    period: &str,
) -> ReportTemplate {
    let mut report = template.clone();
    report.customer = customer.to_string();
    report.period = period.to_string();
    report
}
