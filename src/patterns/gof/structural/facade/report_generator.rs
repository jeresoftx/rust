/// Tipo publico `ReportDataLoader` usado por el ejemplo para expresar el dominio del patron.
pub struct ReportDataLoader {
    rows: Vec<SalesRow>,
}

struct SalesRow {
    period: String,
    amount: u64,
}

impl ReportDataLoader {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(rows: Vec<(&str, u64)>) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(|(period, amount)| SalesRow {
                    period: period.to_string(),
                    amount,
                })
                .collect(),
        }
    }

    /// Operacion `load period` definida por la abstraccion del ejemplo.
    fn load_period(&self, period: &str) -> Vec<u64> {
        self.rows
            .iter()
            .filter(|row| row.period == period)
            .map(|row| row.amount)
            .collect()
    }
}

struct ReportCalculator;

impl ReportCalculator {
    /// Operacion `summarize` definida por la abstraccion del ejemplo.
    fn summarize(amounts: &[u64]) -> ReportSummary {
        let total = amounts.iter().sum();
        let average = if amounts.is_empty() {
            0
        } else {
            total / amounts.len() as u64
        };

        ReportSummary { total, average }
    }
}

/// Tipo publico `ReportSummary` usado por el ejemplo para expresar el dominio del patron.
pub struct ReportSummary {
    total: u64,
    average: u64,
}

/// Contrato publico `ReportRenderer` que desacopla las piezas del ejemplo.
pub trait ReportRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, period: &str, summary: &ReportSummary) -> String;
}

/// Tipo publico `HtmlReportRenderer` usado por el ejemplo para expresar el dominio del patron.
pub struct HtmlReportRenderer;

impl ReportRenderer for HtmlReportRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, period: &str, summary: &ReportSummary) -> String {
        format!(
            "<section><h1>Sales {period}</h1><p>total={} average={}</p></section>",
            summary.total, summary.average
        )
    }
}

/// Tipo publico `TextReportRenderer` usado por el ejemplo para expresar el dominio del patron.
pub struct TextReportRenderer;

impl ReportRenderer for TextReportRenderer {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, period: &str, summary: &ReportSummary) -> String {
        format!(
            "Sales {period}\nTotal: {}\nAverage: {}",
            summary.total, summary.average
        )
    }
}

/// Tipo publico `ReportFacade` usado por el ejemplo para expresar el dominio del patron.
pub struct ReportFacade<R> {
    loader: ReportDataLoader,
    renderer: R,
}

impl<R> ReportFacade<R>
where
    R: ReportRenderer,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(loader: ReportDataLoader, renderer: R) -> Self {
        Self { loader, renderer }
    }

    /// Modela la operacion `generate` dentro del ejemplo del patron.
    pub fn generate(&self, period: &str) -> String {
        let rows = self.loader.load_period(period);
        let summary = ReportCalculator::summarize(&rows);

        self.renderer.render(period, &summary)
    }
}
