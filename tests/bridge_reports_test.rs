use design_patterns_rust::patterns::gof::structural::bridge::reports::{
    HtmlRenderer, PdfRenderer, SalesReport, TextRenderer,
};

#[test]
fn bridge_renders_the_same_report_with_different_renderers() {
    let pdf_report = SalesReport::new(PdfRenderer);
    let html_report = SalesReport::new(HtmlRenderer);
    let text_report = SalesReport::new(TextRenderer);

    assert_eq!(
        pdf_report.render("Q1", 120_000),
        "pdf title=Sales Q1 body=total:120000"
    );
    assert_eq!(
        html_report.render("Q1", 120_000),
        "<article><h1>Sales Q1</h1><p>total:120000</p></article>"
    );
    assert_eq!(
        text_report.render("Q1", 120_000),
        "Sales Q1\n----\ntotal:120000"
    );
}

#[test]
fn bridge_keeps_report_logic_independent_from_renderer() {
    let report = SalesReport::new(TextRenderer);

    assert_eq!(report.render("Q2", 98_500), "Sales Q2\n----\ntotal:98500");
}
