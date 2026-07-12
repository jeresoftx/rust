use design_patterns_rust::patterns::architecture::clean_architecture::presenters::{
    adapters::{CliPresenter, HttpPresenter},
    entities::Invoice,
    use_cases::{GenerateInvoice, InvoicePresenter, InvoiceView},
};

#[test]
fn clean_architecture_http_presenter_formats_invoice_response() {
    let presenter = HttpPresenter::default();
    let mut use_case = GenerateInvoice::new(presenter);

    let response = use_case.execute(Invoice::new("INV-1", 15_000));

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.body(),
        "{\"invoice\":\"INV-1\",\"total_cents\":15000}"
    );
}

#[test]
fn clean_architecture_cli_presenter_formats_invoice_response() {
    let presenter = CliPresenter::default();
    let mut use_case = GenerateInvoice::new(presenter);

    let output = use_case.execute(Invoice::new("INV-2", 2_500));

    assert_eq!(output, "INV-2 | total: $25.00");
}

#[test]
fn clean_architecture_presenter_keeps_use_case_output_boundary_generic() {
    #[derive(Default)]
    struct RecordingPresenter {
        last_view: Option<InvoiceView>,
    }

    impl InvoicePresenter for RecordingPresenter {
        type Response = String;

        fn present(&mut self, view: InvoiceView) -> Self::Response {
            self.last_view = Some(view.clone());
            format!("recorded:{}", view.id())
        }
    }

    let presenter = RecordingPresenter::default();
    let mut use_case = GenerateInvoice::new(presenter);

    let response = use_case.execute(Invoice::new("INV-3", 9_900));
    let presenter = use_case.into_presenter();

    assert_eq!(response, "recorded:INV-3");
    assert_eq!(
        presenter
            .last_view
            .expect("view should be recorded")
            .total_cents(),
        9_900
    );
}
