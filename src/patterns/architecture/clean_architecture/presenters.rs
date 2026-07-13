//! Presenters.
//!
//! # Ejemplo ejecutable
//!
//! ```
//! use design_patterns_rust::patterns::architecture::clean_architecture::presenters as _module;
//! ```
//!
//! Complejidad: el modulo solo reexporta ejemplos; consulta cada tipo
//! publico para los costes de sus operaciones.
/// Modulo del ejemplo `entities` dentro del catalogo de patrones.
pub mod entities {
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `Invoice` usado por el ejemplo para expresar el dominio del patron.
    pub struct Invoice {
        id: String,
        total_cents: u32,
    }

    impl Invoice {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(id: impl Into<String>, total_cents: u32) -> Self {
            Self {
                id: id.into(),
                total_cents,
            }
        }

        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `total cents` dentro del ejemplo del patron.
        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }
}

/// Modulo del ejemplo `use_cases` dentro del catalogo de patrones.
pub mod use_cases {
    use super::entities::Invoice;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `InvoiceView` usado por el ejemplo para expresar el dominio del patron.
    pub struct InvoiceView {
        id: String,
        total_cents: u32,
    }

    impl InvoiceView {
        /// Modela la operacion `id` dentro del ejemplo del patron.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Modela la operacion `total cents` dentro del ejemplo del patron.
        pub fn total_cents(&self) -> u32 {
            self.total_cents
        }
    }

    /// Contrato publico `InvoicePresenter` que desacopla las piezas del ejemplo.
    pub trait InvoicePresenter {
        /// Tipo asociado `Response` producido por la abstraccion del ejemplo.
        type Response;

        /// Operacion `present` definida por la abstraccion del ejemplo.
        fn present(&mut self, view: InvoiceView) -> Self::Response;
    }

    #[derive(Debug, Clone)]
    /// Tipo publico `GenerateInvoice` usado por el ejemplo para expresar el dominio del patron.
    pub struct GenerateInvoice<P> {
        presenter: P,
    }

    impl<P> GenerateInvoice<P>
    where
        P: InvoicePresenter,
    {
        /// Crea una instancia valida para el ejemplo del patron.
        pub fn new(presenter: P) -> Self {
            Self { presenter }
        }

        /// Ejecuta el caso de uso o comando del ejemplo.
        pub fn execute(&mut self, invoice: Invoice) -> P::Response {
            let view = InvoiceView {
                id: invoice.id().to_string(),
                total_cents: invoice.total_cents(),
            };

            self.presenter.present(view)
        }

        /// Modela la operacion `into presenter` dentro del ejemplo del patron.
        pub fn into_presenter(self) -> P {
            self.presenter
        }
    }
}

/// Modulo del ejemplo `adapters` dentro del catalogo de patrones.
pub mod adapters {
    use super::use_cases::{InvoicePresenter, InvoiceView};

    #[derive(Debug, Default, Clone)]
    /// Tipo publico `HttpPresenter` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpPresenter;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Tipo publico `HttpInvoiceResponse` usado por el ejemplo para expresar el dominio del patron.
    pub struct HttpInvoiceResponse {
        status_code: u16,
        body: String,
    }

    impl HttpInvoiceResponse {
        /// Modela la operacion `status code` dentro del ejemplo del patron.
        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        /// Modela la operacion `body` dentro del ejemplo del patron.
        pub fn body(&self) -> &str {
            &self.body
        }
    }

    impl InvoicePresenter for HttpPresenter {
        /// Tipo asociado `Response` producido por la abstraccion del ejemplo.
        type Response = HttpInvoiceResponse;

        /// Operacion `present` definida por la abstraccion del ejemplo.
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
    /// Tipo publico `CliPresenter` usado por el ejemplo para expresar el dominio del patron.
    pub struct CliPresenter;

    impl InvoicePresenter for CliPresenter {
        /// Tipo asociado `Response` producido por la abstraccion del ejemplo.
        type Response = String;

        /// Operacion `present` definida por la abstraccion del ejemplo.
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
