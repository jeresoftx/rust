use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    sku: String,
    name: String,
}

impl Product {
    pub fn new(sku: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug)]
pub struct ProductRepository {
    products: HashMap<String, Product>,
}

impl ProductRepository {
    pub fn with_products<const N: usize>(products: [Product; N]) -> Self {
        Self {
            products: products
                .into_iter()
                .map(|product| (product.sku.clone(), product))
                .collect(),
        }
    }

    fn find(&self, sku: &str) -> Option<Product> {
        self.products.get(sku).cloned()
    }
}

#[derive(Debug)]
pub struct CacheAsideService {
    repository: ProductRepository,
    cache: HashMap<String, Product>,
    repository_reads: usize,
}

impl CacheAsideService {
    pub fn new(repository: ProductRepository) -> Self {
        Self {
            repository,
            cache: HashMap::new(),
            repository_reads: 0,
        }
    }

    pub fn get_product(&mut self, sku: &str) -> Option<Product> {
        if let Some(product) = self.cache.get(sku) {
            return Some(product.clone());
        }

        self.repository_reads += 1;
        let product = self.repository.find(sku)?;
        self.cache.insert(sku.to_string(), product.clone());
        Some(product)
    }

    pub fn repository_reads(&self) -> usize {
        self.repository_reads
    }

    pub fn cached_keys(&self) -> Vec<String> {
        let mut keys: Vec<_> = self.cache.keys().cloned().collect();
        keys.sort();
        keys
    }
}
