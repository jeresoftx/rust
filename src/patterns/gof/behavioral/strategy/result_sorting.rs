#[derive(Debug, Clone, PartialEq)]
/// Tipo publico `ResultItem` usado por el ejemplo para expresar el dominio del patron.
pub struct ResultItem {
    title: String,
    relevance_score: f64,
    price: u32,
    created_at: u64,
}

impl ResultItem {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Modela la operacion `title` dentro del ejemplo del patron.
    pub fn title(&self) -> &str {
        &self.title
    }
}

/// Contrato publico `SortStrategy` que desacopla las piezas del ejemplo.
pub trait SortStrategy {
    /// Operacion `sort` definida por la abstraccion del ejemplo.
    fn sort(&self, results: &mut [ResultItem]);
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `RelevanceFirst` usado por el ejemplo para expresar el dominio del patron.
pub struct RelevanceFirst;

impl SortStrategy for RelevanceFirst {
    /// Operacion `sort` definida por la abstraccion del ejemplo.
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
/// Tipo publico `PriceLowToHigh` usado por el ejemplo para expresar el dominio del patron.
pub struct PriceLowToHigh;

impl SortStrategy for PriceLowToHigh {
    /// Operacion `sort` definida por la abstraccion del ejemplo.
    fn sort(&self, results: &mut [ResultItem]) {
        results.sort_by(|left, right| {
            left.price
                .cmp(&right.price)
                .then_with(|| right.relevance_score.total_cmp(&left.relevance_score))
        });
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `NewestFirst` usado por el ejemplo para expresar el dominio del patron.
pub struct NewestFirst;

impl SortStrategy for NewestFirst {
    /// Operacion `sort` definida por la abstraccion del ejemplo.
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
/// Tipo publico `ResultSorter` usado por el ejemplo para expresar el dominio del patron.
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
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    /// Modela la operacion `sort` dentro del ejemplo del patron.
    pub fn sort(&self, mut results: Vec<ResultItem>) -> Vec<ResultItem> {
        self.strategy.sort(&mut results);
        results
    }

    /// Modela la operacion `sort titles` dentro del ejemplo del patron.
    pub fn sort_titles(&self, results: Vec<ResultItem>) -> Vec<String> {
        self.sort(results)
            .into_iter()
            .map(|result| result.title)
            .collect()
    }
}
