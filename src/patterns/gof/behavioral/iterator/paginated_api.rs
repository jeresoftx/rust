#[derive(Debug, Clone)]
pub struct ApiPageSource {
    pages: Vec<Vec<String>>,
    fetched_pages: usize,
}

impl ApiPageSource {
    pub fn new(pages: Vec<Vec<String>>) -> Self {
        Self {
            pages,
            fetched_pages: 0,
        }
    }

    fn fetch_page(&mut self, index: usize) -> Option<Vec<String>> {
        let page = self.pages.get(index).cloned();
        if page.is_some() {
            self.fetched_pages += 1;
        }
        page
    }

    fn fetched_pages(&self) -> usize {
        self.fetched_pages
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResults {
    source: ApiPageSource,
    page_index: usize,
    current_page: std::vec::IntoIter<String>,
}

impl PaginatedResults {
    pub fn new(source: ApiPageSource) -> Self {
        Self {
            source,
            page_index: 0,
            current_page: Vec::new().into_iter(),
        }
    }

    pub fn fetched_pages(&self) -> usize {
        self.source.fetched_pages()
    }
}

impl Iterator for PaginatedResults {
    type Item = String;

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
