use design_patterns_rust::patterns::gof::structural::facade::report_generator::{
    HtmlReportRenderer, ReportDataLoader, ReportFacade, TextReportRenderer,
};

#[test]
fn facade_generates_report_without_exposing_loading_calculation_or_rendering() {
    let facade = ReportFacade::new(
        ReportDataLoader::new(vec![("Q1", 1200), ("Q1", 800), ("Q2", 500)]),
        HtmlReportRenderer,
    );

    assert_eq!(
        facade.generate("Q1"),
        "<section><h1>Sales Q1</h1><p>total=2000 average=1000</p></section>"
    );
}

#[test]
fn facade_can_swap_renderer_without_changing_report_flow() {
    let facade = ReportFacade::new(
        ReportDataLoader::new(vec![("Q1", 1200), ("Q1", 800)]),
        TextReportRenderer,
    );

    assert_eq!(
        facade.generate("Q1"),
        "Sales Q1\nTotal: 2000\nAverage: 1000"
    );
}
