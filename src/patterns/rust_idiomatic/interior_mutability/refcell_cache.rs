use std::cell::{Cell, RefCell};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    sku: String,
    name: String,
    price_cents: u64,
}

impl Product {
    pub fn new(sku: impl Into<String>, name: impl Into<String>, price_cents: u64) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
            price_cents,
        }
    }

    pub fn sku(&self) -> &str {
        &self.sku
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price_cents(&self) -> u64 {
        self.price_cents
    }
}

#[derive(Debug)]
pub struct ProductCatalog {
    source: RefCell<HashMap<String, Product>>,
    cache: RefCell<HashMap<String, Product>>,
    source_reads: Cell<u32>,
}

impl ProductCatalog {
    pub fn new(products: Vec<Product>) -> Self {
        let source = products
            .into_iter()
            .map(|product| (product.sku.clone(), product))
            .collect();

        Self {
            source: RefCell::new(source),
            cache: RefCell::new(HashMap::new()),
            source_reads: Cell::new(0),
        }
    }

    pub fn find_product(&self, sku: &str) -> Option<Product> {
        if let Some(product) = self.cache.borrow().get(sku) {
            return Some(product.clone());
        }

        self.source_reads.set(self.source_reads.get() + 1);

        let product = self.source.borrow().get(sku).cloned()?;
        self.cache
            .borrow_mut()
            .insert(product.sku.clone(), product.clone());

        Some(product)
    }

    pub fn replace_product(&self, product: Product) {
        self.source
            .borrow_mut()
            .insert(product.sku.clone(), product.clone());
        self.cache.borrow_mut().remove(&product.sku);
    }

    pub fn source_reads(&self) -> u32 {
        self.source_reads.get()
    }

    pub fn cached_skus(&self) -> Vec<String> {
        let mut skus = self.cache.borrow().keys().cloned().collect::<Vec<_>>();
        skus.sort();
        skus
    }
}
