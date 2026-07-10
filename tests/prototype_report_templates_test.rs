use design_patterns_rust::patterns::gof::creational::prototype::report_templates::{
    ReportTemplate, monthly_report_for_customer,
};

#[test]
fn prototype_clones_report_template_for_a_customer_period() {
    let template = ReportTemplate::standard_financial();

    let report = monthly_report_for_customer(&template, "Acme Corp", "2026-07");

    assert_eq!(
        report.summary(),
        "Financial Summary for Acme Corp [2026-07] sections=Revenue,Costs,Margin"
    );
}

#[test]
fn prototype_keeps_original_report_template_unchanged() {
    let template = ReportTemplate::standard_financial();

    let _report = monthly_report_for_customer(&template, "Acme Corp", "2026-07");

    assert_eq!(
        template.summary(),
        "Financial Summary for <customer> [<period>] sections=Revenue,Costs,Margin"
    );
}
