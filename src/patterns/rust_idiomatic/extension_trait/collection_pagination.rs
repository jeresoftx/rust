#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    items: Vec<T>,
    page: usize,
    per_page: usize,
    total_items: usize,
}

pub trait CollectionPaginationExt<T> {
    fn paginate(&self, page: usize, per_page: usize) -> Page<T>;
}

impl<T> CollectionPaginationExt<T> for [T]
where
    T: Clone,
{
    fn paginate(&self, page: usize, per_page: usize) -> Page<T> {
        let total_items = self.len();

        if page == 0 || per_page == 0 {
            return Page {
                items: Vec::new(),
                page,
                per_page,
                total_items,
            };
        }

        let start = (page - 1).saturating_mul(per_page);
        let items = self
            .get(start..)
            .unwrap_or(&[])
            .iter()
            .take(per_page)
            .cloned()
            .collect();

        Page {
            items,
            page,
            per_page,
            total_items,
        }
    }
}

impl<T> Page<T> {
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn per_page(&self) -> usize {
        self.per_page
    }

    pub fn total_items(&self) -> usize {
        self.total_items
    }

    pub fn total_pages(&self) -> usize {
        if self.per_page == 0 {
            return 0;
        }

        self.total_items.div_ceil(self.per_page)
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }
}
