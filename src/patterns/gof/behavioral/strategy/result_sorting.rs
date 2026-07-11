#[derive(Debug, Clone, PartialEq)]
pub struct ResultItem {
    title: String,
    relevance_score: f64,
    price: u32,
    created_at: u64,
}

impl ResultItem {
    pub fn new(
        title: impl Into<String>,
        relevance_score: f64,
        price: u32,
        created_at: u64,
    ) -> Self {
        Self {
            title: title.into(),
            relevance_score,
            price,
            created_at,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

pub trait SortStrategy {
    fn sort(&self, results: &mut [ResultItem]);
}

#[derive(Debug, Clone, Copy)]
pub struct RelevanceFirst;

impl SortStrategy for RelevanceFirst {
    fn sort(&self, results: &mut [ResultItem]) {
        results.sort_by(|left, right| {
            right
                .relevance_score
                .total_cmp(&left.relevance_score)
                .then_with(|| left.price.cmp(&right.price))
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PriceLowToHigh;

impl SortStrategy for PriceLowToHigh {
    fn sort(&self, results: &mut [ResultItem]) {
        results.sort_by(|left, right| {
            left.price
                .cmp(&right.price)
                .then_with(|| right.relevance_score.total_cmp(&left.relevance_score))
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NewestFirst;

impl SortStrategy for NewestFirst {
    fn sort(&self, results: &mut [ResultItem]) {
        results.sort_by(|left, right| {
            right
                .created_at
                .cmp(&left.created_at)
                .then_with(|| right.relevance_score.total_cmp(&left.relevance_score))
        });
    }
}

#[derive(Debug, Clone)]
pub struct ResultSorter<S>
where
    S: SortStrategy,
{
    strategy: S,
}

impl<S> ResultSorter<S>
where
    S: SortStrategy,
{
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    pub fn sort(&self, mut results: Vec<ResultItem>) -> Vec<ResultItem> {
        self.strategy.sort(&mut results);
        results
    }

    pub fn sort_titles(&self, results: Vec<ResultItem>) -> Vec<String> {
        self.sort(results)
            .into_iter()
            .map(|result| result.title)
            .collect()
    }
}
