#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportTemplate {
    title: String,
    customer: String,
    period: String,
    sections: Vec<String>,
}

impl ReportTemplate {
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
