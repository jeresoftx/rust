#[derive(Debug, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `SearchQueryError` dentro del ejemplo.
pub enum SearchQueryError {
    /// Variante `TermRequired` del estado o error del ejemplo.
    TermRequired,
}

#[derive(Debug, PartialEq, Eq)]
/// Tipo publico `SearchQuery` usado por el ejemplo para expresar el dominio del patron.
pub struct SearchQuery {
    term: String,
    category: Option<String>,
    min_score: Option<u8>,
    tags: Vec<String>,
    sort_by: Option<String>,
}

impl SearchQuery {
    /// Modela la operacion `builder` dentro del ejemplo del patron.
    pub fn builder(term: impl Into<String>) -> SearchQueryBuilder {
        SearchQueryBuilder {
            term: term.into(),
            category: None,
            min_score: None,
            tags: Vec::new(),
            sort_by: None,
        }
    }

    /// Modela la operacion `to query string` dentro del ejemplo del patron.
    pub fn to_query_string(&self) -> String {
        let mut parts = vec![format!("q={}", self.term)];

        if let Some(category) = &self.category {
            parts.push(format!("category={category}"));
        }

        if let Some(min_score) = self.min_score {
            parts.push(format!("min_score={min_score}"));
        }

        if !self.tags.is_empty() {
            parts.push(format!("tags={}", self.tags.join(",")));
        }

        if let Some(sort_by) = &self.sort_by {
            parts.push(format!("sort={sort_by}"));
        }

        parts.join("&")
    }
}

/// Tipo publico `SearchQueryBuilder` usado por el ejemplo para expresar el dominio del patron.
pub struct SearchQueryBuilder {
    term: String,
    category: Option<String>,
    min_score: Option<u8>,
    tags: Vec<String>,
    sort_by: Option<String>,
}

impl SearchQueryBuilder {
    /// Modela la operacion `category` dentro del ejemplo del patron.
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Modela la operacion `min score` dentro del ejemplo del patron.
    pub fn min_score(mut self, min_score: u8) -> Self {
        self.min_score = Some(min_score);
        self
    }

    /// Modela la operacion `tag` dentro del ejemplo del patron.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Modela la operacion `sort by` dentro del ejemplo del patron.
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    /// Construye el valor final a partir de la configuracion acumulada.
    pub fn build(self) -> Result<SearchQuery, SearchQueryError> {
        let term = self.term.trim().to_string();

        if term.is_empty() {
            return Err(SearchQueryError::TermRequired);
        }

        Ok(SearchQuery {
            term,
            category: self.category,
            min_score: self.min_score,
            tags: self.tags,
            sort_by: self.sort_by,
        })
    }
}
