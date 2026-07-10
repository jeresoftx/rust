pub trait ReportRenderer {
    fn render(&self, title: &str, body: &str) -> String;
}

pub struct SalesReport<R> {
    renderer: R,
}

impl<R> SalesReport<R>
where
    R: ReportRenderer,
{
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    pub fn render(&self, period: &str, total: u64) -> String {
        let title = format!("Sales {period}");
        let body = format!("total:{total}");

        self.renderer.render(&title, &body)
    }
}

pub struct PdfRenderer;

impl ReportRenderer for PdfRenderer {
    fn render(&self, title: &str, body: &str) -> String {
        format!("pdf title={title} body={body}")
    }
}

pub struct HtmlRenderer;

impl ReportRenderer for HtmlRenderer {
    fn render(&self, title: &str, body: &str) -> String {
        format!("<article><h1>{title}</h1><p>{body}</p></article>")
    }
}

pub struct TextRenderer;

impl ReportRenderer for TextRenderer {
    fn render(&self, title: &str, body: &str) -> String {
        format!("{title}\n----\n{body}")
    }
}
