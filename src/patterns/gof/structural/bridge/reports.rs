/// Contrato publico `ReportRenderer` que desacopla las piezas del ejemplo.
pub trait ReportRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, title: &str, body: &str) -> String;
}

/// Tipo publico `SalesReport` usado por el ejemplo para expresar el dominio del patron.
pub struct SalesReport<R> {
    renderer: R,
}

impl<R> SalesReport<R>
where
    R: ReportRenderer,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    /// Modela la operacion `render` dentro del ejemplo del patron.
    pub fn render(&self, period: &str, total: u64) -> String {
        let title = format!("Sales {period}");
        let body = format!("total:{total}");

        self.renderer.render(&title, &body)
    }
}

/// Tipo publico `PdfRenderer` usado por el ejemplo para expresar el dominio del patron.
pub struct PdfRenderer;

impl ReportRenderer for PdfRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, title: &str, body: &str) -> String {
        format!("pdf title={title} body={body}")
    }
}

/// Tipo publico `HtmlRenderer` usado por el ejemplo para expresar el dominio del patron.
pub struct HtmlRenderer;

impl ReportRenderer for HtmlRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, title: &str, body: &str) -> String {
        format!("<article><h1>{title}</h1><p>{body}</p></article>")
    }
}

/// Tipo publico `TextRenderer` usado por el ejemplo para expresar el dominio del patron.
pub struct TextRenderer;

impl ReportRenderer for TextRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, title: &str, body: &str) -> String {
        format!("{title}\n----\n{body}")
    }
}
