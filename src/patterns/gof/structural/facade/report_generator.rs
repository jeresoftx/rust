pub struct ReportDataLoader {
    rows: Vec<SalesRow>,
}

struct SalesRow {
    period: String,
    amount: u64,
}

impl ReportDataLoader {
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

pub struct ReportSummary {
    total: u64,
    average: u64,
}

pub trait ReportRenderer {
    fn render(&self, period: &str, summary: &ReportSummary) -> String;
}

pub struct HtmlReportRenderer;

impl ReportRenderer for HtmlReportRenderer {
    fn render(&self, period: &str, summary: &ReportSummary) -> String {
        format!(
            "<section><h1>Sales {period}</h1><p>total={} average={}</p></section>",
            summary.total, summary.average
        )
    }
}

pub struct TextReportRenderer;

impl ReportRenderer for TextReportRenderer {
    fn render(&self, period: &str, summary: &ReportSummary) -> String {
        format!(
            "Sales {period}\nTotal: {}\nAverage: {}",
            summary.total, summary.average
        )
    }
}

pub struct ReportFacade<R> {
    loader: ReportDataLoader,
    renderer: R,
}

impl<R> ReportFacade<R>
where
    R: ReportRenderer,
{
    pub fn new(loader: ReportDataLoader, renderer: R) -> Self {
        Self { loader, renderer }
    }

    pub fn generate(&self, period: &str) -> String {
        let rows = self.loader.load_period(period);
        let summary = ReportCalculator::summarize(&rows);

        self.renderer.render(period, &summary)
    }
}
