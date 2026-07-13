use std::cell::{Cell, RefCell};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Product` usado por el ejemplo para expresar el dominio del patron.
pub struct Product {
    sku: String,
    name: String,
    price_cents: u64,
}

impl Product {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, name: impl Into<String>, price_cents: u64) -> Self {
        Self {
            sku: sku.into(),
            name: name.into(),
            price_cents,
        }
    }

    /// Modela la operacion `sku` dentro del ejemplo del patron.
    pub fn sku(&self) -> &str {
        &self.sku
    }

    /// Modela la operacion `name` dentro del ejemplo del patron.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Modela la operacion `price cents` dentro del ejemplo del patron.
    pub fn price_cents(&self) -> u64 {
        self.price_cents
    }
}

#[derive(Debug)]
/// Tipo publico `ProductCatalog` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductCatalog {
    source: RefCell<HashMap<String, Product>>,
    cache: RefCell<HashMap<String, Product>>,
    source_reads: Cell<u32>,
}

impl ProductCatalog {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Modela la operacion `find product` dentro del ejemplo del patron.
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

    /// Modela la operacion `replace product` dentro del ejemplo del patron.
    pub fn replace_product(&self, product: Product) {
        self.source
            .borrow_mut()
            .insert(product.sku.clone(), product.clone());
        self.cache.borrow_mut().remove(&product.sku);
    }

    /// Modela la operacion `source reads` dentro del ejemplo del patron.
    pub fn source_reads(&self) -> u32 {
        self.source_reads.get()
    }

    /// Modela la operacion `cached skus` dentro del ejemplo del patron.
    pub fn cached_skus(&self) -> Vec<String> {
        let mut skus = self.cache.borrow().keys().cloned().collect::<Vec<_>>();
        skus.sort();
        skus
    }
}
