pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Invoice {
        id: String,
        total_cents: u32,
    }

    impl Invoice {
        pub fn new(id: impl Into<String>, total_cents: u32) -> Self {
            Self {
                id: id.into(),
                total_cents,
            }
        }

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }
}

pub mod use_cases {
    use super::entities::Invoice;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InvoiceView {
        id: String,
        total_cents: u32,
    }

    impl InvoiceView {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }

    pub trait InvoicePresenter {
        type Response;

        fn present(&mut self, view: InvoiceView) -> Self::Response;
    }

    #[derive(Debug, Clone)]
    pub struct GenerateInvoice<P> {
        presenter: P,
    }

    impl<P> GenerateInvoice<P>
    where
        P: InvoicePresenter,
    {
        pub fn new(presenter: P) -> Self {
            Self { presenter }
        }

        pub fn execute(&mut self, invoice: Invoice) -> P::Response {
            let view = InvoiceView {
                id: invoice.id().to_string(),
                total_cents: invoice.total_cents(),
            };

            self.presenter.present(view)
        }

        pub fn into_presenter(self) -> P {
            self.presenter
        }
    }
}

pub mod adapters {
    use super::use_cases::{InvoicePresenter, InvoiceView};

    #[derive(Debug, Default, Clone)]
    pub struct HttpPresenter;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HttpInvoiceResponse {
        status_code: u16,
        body: String,
    }

    impl HttpInvoiceResponse {
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> &str {
            &self.body
        }
    }

    impl InvoicePresenter for HttpPresenter {
        type Response = HttpInvoiceResponse;

        fn present(&mut self, view: InvoiceView) -> Self::Response {
            HttpInvoiceResponse {
                status_code: 200,
                body: format!(
                    "{{\"invoice\":\"{}\",\"total_cents\":{}}}",
                    view.id(),
                    view.total_cents()
                ),
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct CliPresenter;

    impl InvoicePresenter for CliPresenter {
        type Response = String;

        fn present(&mut self, view: InvoiceView) -> Self::Response {
            format!(
                "{} | total: ${}.{:02}",
                view.id(),
                view.total_cents() / 100,
                view.total_cents() % 100
            )
        }
    }
}
