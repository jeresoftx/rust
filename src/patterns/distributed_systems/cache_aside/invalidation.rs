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
pub struct CatalogService {
    repository: HashMap<String, Product>,
    cache: HashMap<String, Product>,
}

impl CatalogService {
    pub fn with_products<const N: usize>(products: [Product; N]) -> Self {
        Self {
            repository: products
                .into_iter()
                .map(|product| (product.sku.clone(), product))
                .collect(),
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, sku: &str) -> Option<Product> {
        if let Some(product) = self.cache.get(sku) {
            return Some(product.clone());
        }
        let product = self.repository.get(sku)?.clone();
        self.cache.insert(sku.to_string(), product.clone());
        Some(product)
    }

    pub fn update(&mut self, product: Product) {
        self.cache.remove(&product.sku);
        self.repository.insert(product.sku.clone(), product);
    }

    pub fn is_cached(&self, sku: &str) -> bool {
        self.cache.contains_key(sku)
    }
}
