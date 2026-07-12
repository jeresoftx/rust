use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    sku: String,
    stock: u32,
}

impl Product {
    pub fn new(sku: impl Into<String>, stock: u32) -> Self {
        Self {
            sku: sku.into(),
            stock,
        }
    }

    pub fn sku(&self) -> &str {
        &self.sku
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InventoryStore {
    products: HashMap<String, Product>,
}

impl InventoryStore {
    pub fn seed(&mut self, product: Product) {
        self.products.insert(product.sku().to_string(), product);
    }

    pub fn stock(&self, sku: &str) -> Option<u32> {
        self.products.get(sku).map(|product| product.stock)
    }
}

#[derive(Debug)]
pub struct TransactionalUnitOfWork<'a> {
    store: &'a mut InventoryStore,
}

impl<'a> TransactionalUnitOfWork<'a> {
    pub fn run<F>(store: &'a mut InventoryStore, operation: F) -> Result<(), TransactionError>
    where
        F: FnOnce(&mut Self) -> Result<(), TransactionError>,
    {
        let snapshot = store.clone();
        let mut unit_of_work = Self { store };

        match operation(&mut unit_of_work) {
            Ok(()) => Ok(()),
            Err(error) => {
                *unit_of_work.store = snapshot;
                Err(error)
            }
        }
    }

    pub fn create_product(
        &mut self,
        sku: impl Into<String>,
        stock: u32,
    ) -> Result<(), TransactionError> {
        let sku = sku.into();

        if self.store.products.contains_key(&sku) {
            return Err(TransactionError::ProductAlreadyExists);
        }

        self.store
            .products
            .insert(sku.clone(), Product::new(sku, stock));
        Ok(())
    }

    pub fn reserve_stock(&mut self, sku: &str, quantity: u32) -> Result<(), TransactionError> {
        let product = self
            .store
            .products
            .get_mut(sku)
            .ok_or(TransactionError::ProductNotFound)?;

        if product.stock < quantity {
            return Err(TransactionError::InsufficientStock);
        }

        product.stock -= quantity;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    ProductAlreadyExists,
    ProductNotFound,
    InsufficientStock,
}
