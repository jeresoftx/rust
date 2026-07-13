use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// Contrato publico `ProductRepository` que desacopla las piezas del ejemplo.
pub trait ProductRepository {
    /// Operacion `find name` definida por la abstraccion del ejemplo.
    fn find_name(&self, sku: &str) -> Option<String>;
    /// Operacion `lookup count` definida por la abstraccion del ejemplo.
    fn lookup_count(&self) -> usize {
        0
    }
}

/// Tipo publico `InMemoryProductRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct InMemoryProductRepository {
    products: HashMap<String, String>,
    lookups: Cell<usize>,
}

impl InMemoryProductRepository {
    /// Crea una instancia valida para el ejemplo del patron.
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
    /// Operacion `find name` definida por la abstraccion del ejemplo.
    fn find_name(&self, sku: &str) -> Option<String> {
        self.lookups.set(self.lookups.get() + 1);
        self.products.get(sku).cloned()
    }

    /// Operacion `lookup count` definida por la abstraccion del ejemplo.
    fn lookup_count(&self) -> usize {
        self.lookups.get()
    }
}

/// Tipo publico `CachedProductRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct CachedProductRepository<R> {
    inner: R,
    cache: RefCell<HashMap<String, String>>,
}

impl<R> CachedProductRepository<R> {
    /// Crea una instancia valida para el ejemplo del patron.
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
    /// Operacion `find name` definida por la abstraccion del ejemplo.
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

    /// Operacion `lookup count` definida por la abstraccion del ejemplo.
    fn lookup_count(&self) -> usize {
        self.inner.lookup_count()
    }
}

/// Modela la operacion `read product twice` dentro del ejemplo del patron.
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
