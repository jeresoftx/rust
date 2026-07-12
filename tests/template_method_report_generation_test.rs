use design_patterns_rust::patterns::gof::behavioral::template_method::report_generation::{
    IncidentReport, ReportTemplate, SalesReport,
};

#[test]
fn template_method_generates_sales_report_with_common_layout() {
    let report = SalesReport::new(120_000, 48).generate();

    assert_eq!(
        report.lines(),
        [
            "cover:Sales report",
            "summary:Monthly sales performance",
            "section:revenue_cents=120000",
            "section:orders=48",
            "footer:generated_by=reporting"
        ]
    );
}

#[test]
fn template_method_generates_incident_report_with_custom_sections() {
    let report = IncidentReport::new("checkout", "high", 32).generate();

    assert_eq!(
        report.lines(),
        [
            "cover:Incident report",
            "summary:Operational incident review",
            "section:service=checkout",
            "section:severity=high",
            "section:duration_minutes=32",
            "footer:generated_by=reporting"
        ]
    );
}

#[test]
fn template_method_allows_reports_to_override_footer_hook() {
    let report = IncidentReport::new("payments", "critical", 12)
        .with_owner("sre")
        .generate();

    assert_eq!(report.lines().last(), Some(&"footer:owner=sre"));
}
