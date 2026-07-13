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

#[derive(Debug, Clone)]
struct CacheEntry {
    product: Product,
    expires_at: u64,
}

#[derive(Debug)]
/// Tipo publico `TtlCacheAsideService` usado por el ejemplo para expresar el dominio del patron.
pub struct TtlCacheAsideService {
    repository: HashMap<String, Product>,
    cache: HashMap<String, CacheEntry>,
    ttl_ticks: u64,
    repository_reads: usize,
}

impl TtlCacheAsideService {
    /// Modela la operacion `with products` dentro del ejemplo del patron.
    pub fn with_products<const N: usize>(products: [Product; N], ttl_ticks: u64) -> Self {
        Self {
            repository: products
                .into_iter()
                .map(|product| (product.sku.clone(), product))
                .collect(),
            cache: HashMap::new(),
            ttl_ticks,
            repository_reads: 0,
        }
    }

    /// Modela la operacion `get at` dentro del ejemplo del patron.
    pub fn get_at(&mut self, sku: &str, now_tick: u64) -> Option<Product> {
        if let Some(entry) = self.cache.get(sku)
            && now_tick < entry.expires_at
        {
            return Some(entry.product.clone());
        }

        self.repository_reads += 1;
        let product = self.repository.get(sku)?.clone();
        self.cache.insert(
            sku.to_string(),
            CacheEntry {
                product: product.clone(),
                expires_at: now_tick + self.ttl_ticks,
            },
        );
        Some(product)
    }

    /// Modela la operacion `repository reads` dentro del ejemplo del patron.
    pub fn repository_reads(&self) -> usize {
        self.repository_reads
    }

    /// Modela la operacion `expires at` dentro del ejemplo del patron.
    pub fn expires_at(&self, sku: &str) -> Option<u64> {
        self.cache.get(sku).map(|entry| entry.expires_at)
    }
}
