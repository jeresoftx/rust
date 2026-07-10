use std::cell::{Cell, RefCell};
use std::collections::HashMap;

pub trait ProductRepository {
    fn find_name(&self, sku: &str) -> Option<String>;
    fn lookup_count(&self) -> usize {
        0
    }
}

pub struct InMemoryProductRepository {
    products: HashMap<String, String>,
    lookups: Cell<usize>,
}

impl InMemoryProductRepository {
    pub fn new(products: Vec<(&str, &str)>) -> Self {
        Self {
            products: products
                .into_iter()
                .map(|(sku, name)| (sku.to_string(), name.to_string()))
                .collect(),
            lookups: Cell::new(0),
        }
    }
}

impl ProductRepository for InMemoryProductRepository {
    fn find_name(&self, sku: &str) -> Option<String> {
        self.lookups.set(self.lookups.get() + 1);
        self.products.get(sku).cloned()
    }

    fn lookup_count(&self) -> usize {
        self.lookups.get()
    }
}

pub struct CachedProductRepository<R> {
    inner: R,
    cache: RefCell<HashMap<String, String>>,
}

impl<R> CachedProductRepository<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            cache: RefCell::new(HashMap::new()),
        }
    }
}

impl<R> ProductRepository for CachedProductRepository<R>
where
    R: ProductRepository,
{
    fn find_name(&self, sku: &str) -> Option<String> {
        if let Some(name) = self.cache.borrow().get(sku) {
            return Some(name.clone());
        }

        let name = self.inner.find_name(sku)?;
        self.cache
            .borrow_mut()
            .insert(sku.to_string(), name.clone());
        Some(name)
    }

    fn lookup_count(&self) -> usize {
        self.inner.lookup_count()
    }
}

pub fn read_product_twice(repository: &dyn ProductRepository, sku: &str) -> String {
    let first = repository
        .find_name(sku)
        .unwrap_or_else(|| "missing".to_string());
    let second = repository
        .find_name(sku)
        .unwrap_or_else(|| "missing".to_string());

    format!(
        "first={} second={} lookups={}",
        first,
        second,
        repository.lookup_count()
    )
}
