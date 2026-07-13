#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Page` usado por el ejemplo para expresar el dominio del patron.
pub struct Page<T> {
    items: Vec<T>,
    page: usize,
    per_page: usize,
    total_items: usize,
}

/// Contrato publico `CollectionPaginationExt` que desacopla las piezas del ejemplo.
pub trait CollectionPaginationExt<T> {
    /// Operacion `paginate` definida por la abstraccion del ejemplo.
    fn paginate(&self, page: usize, per_page: usize) -> Page<T>;
}

impl<T> CollectionPaginationExt<T> for [T]
where
    T: Clone,
{
    /// Operacion `paginate` definida por la abstraccion del ejemplo.
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
    /// Modela la operacion `items` dentro del ejemplo del patron.
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Modela la operacion `page` dentro del ejemplo del patron.
    pub fn page(&self) -> usize {
        self.page
    }

    /// Modela la operacion `per page` dentro del ejemplo del patron.
    pub fn per_page(&self) -> usize {
        self.per_page
    }

    /// Modela la operacion `total items` dentro del ejemplo del patron.
    pub fn total_items(&self) -> usize {
        self.total_items
    }

    /// Modela la operacion `total pages` dentro del ejemplo del patron.
    pub fn total_pages(&self) -> usize {
        if self.per_page == 0 {
            return 0;
        }

        self.total_items.div_ceil(self.per_page)
    }

    /// Modela la operacion `has next` dentro del ejemplo del patron.
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }
}
