#[derive(Debug, Clone)]
/// Tipo publico `ApiPageSource` usado por el ejemplo para expresar el dominio del patron.
pub struct ApiPageSource {
    pages: Vec<Vec<String>>,
    fetched_pages: usize,
}

impl ApiPageSource {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(pages: Vec<Vec<String>>) -> Self {
        Self {
            pages,
            fetched_pages: 0,
        }
    }

    /// Operacion `fetch page` definida por la abstraccion del ejemplo.
    fn fetch_page(&mut self, index: usize) -> Option<Vec<String>> {
        let page = self.pages.get(index).cloned();
        if page.is_some() {
            self.fetched_pages += 1;
        }
        page
    }

    /// Operacion `fetched pages` definida por la abstraccion del ejemplo.
    fn fetched_pages(&self) -> usize {
        self.fetched_pages
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `PaginatedResults` usado por el ejemplo para expresar el dominio del patron.
pub struct PaginatedResults {
    source: ApiPageSource,
    page_index: usize,
    current_page: std::vec::IntoIter<String>,
}

impl PaginatedResults {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(source: ApiPageSource) -> Self {
        Self {
            source,
            page_index: 0,
            current_page: Vec::new().into_iter(),
        }
    }

    /// Modela la operacion `fetched pages` dentro del ejemplo del patron.
    pub fn fetched_pages(&self) -> usize {
        self.source.fetched_pages()
    }
}

impl Iterator for PaginatedResults {
    /// Tipo asociado `Item` producido por la abstraccion del ejemplo.
    type Item = String;

    /// Operacion `next` definida por la abstraccion del ejemplo.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.current_page.next() {
                return Some(item);
            }

            let page = self.source.fetch_page(self.page_index)?;
            self.page_index += 1;
            self.current_page = page.into_iter();
        }
    }
}
