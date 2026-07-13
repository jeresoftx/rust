use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Product` usado por el ejemplo para expresar el dominio del patron.
pub struct Product {
    sku: String,
    name: String,
}

impl Product {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug)]
/// Tipo publico `ProductRepository` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductRepository {
    products: HashMap<String, Product>,
}

impl ProductRepository {
    /// Modela la operacion `with products` dentro del ejemplo del patron.
    pub fn with_products<const N: usize>(products: [Product; N]) -> Self {
        Self {
            products: products
                .into_iter()
                .map(|product| (product.sku.clone(), product))
                .collect(),
        }
    }

    /// Operacion `find` definida por la abstraccion del ejemplo.
    fn find(&self, sku: &str) -> Option<Product> {
        self.products.get(sku).cloned()
    }
}

#[derive(Debug)]
/// Tipo publico `CacheAsideService` usado por el ejemplo para expresar el dominio del patron.
pub struct CacheAsideService {
    repository: ProductRepository,
    cache: HashMap<String, Product>,
    repository_reads: usize,
}

impl CacheAsideService {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(repository: ProductRepository) -> Self {
        Self {
            repository,
            cache: HashMap::new(),
            repository_reads: 0,
        }
    }

    /// Modela la operacion `get product` dentro del ejemplo del patron.
    pub fn get_product(&mut self, sku: &str) -> Option<Product> {
        if let Some(product) = self.cache.get(sku) {
            return Some(product.clone());
        }

        self.repository_reads += 1;
        let product = self.repository.find(sku)?;
        self.cache.insert(sku.to_string(), product.clone());
        Some(product)
    }

    /// Modela la operacion `repository reads` dentro del ejemplo del patron.
    pub fn repository_reads(&self) -> usize {
        self.repository_reads
    }

    /// Modela la operacion `cached keys` dentro del ejemplo del patron.
    pub fn cached_keys(&self) -> Vec<String> {
        let mut keys: Vec<_> = self.cache.keys().cloned().collect();
        keys.sort();
        keys
    }
}
