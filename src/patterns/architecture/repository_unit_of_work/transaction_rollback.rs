use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tipo publico `Product` usado por el ejemplo para expresar el dominio del patron.
pub struct Product {
    sku: String,
    stock: u32,
}

impl Product {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(sku: impl Into<String>, stock: u32) -> Self {
        Self {
            sku: sku.into(),
            stock,
        }
    }

    /// Modela la operacion `sku` dentro del ejemplo del patron.
    pub fn sku(&self) -> &str {
        &self.sku
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Tipo publico `InventoryStore` usado por el ejemplo para expresar el dominio del patron.
pub struct InventoryStore {
    products: HashMap<String, Product>,
}

impl InventoryStore {
    /// Modela la operacion `seed` dentro del ejemplo del patron.
    pub fn seed(&mut self, product: Product) {
        self.products.insert(product.sku().to_string(), product);
    }

    /// Modela la operacion `stock` dentro del ejemplo del patron.
    pub fn stock(&self, sku: &str) -> Option<u32> {
        self.products.get(sku).map(|product| product.stock)
    }
}

#[derive(Debug)]
/// Tipo publico `TransactionalUnitOfWork` usado por el ejemplo para expresar el dominio del patron.
pub struct TransactionalUnitOfWork<'a> {
    store: &'a mut InventoryStore,
}

impl<'a> TransactionalUnitOfWork<'a> {
    /// Ejecuta el flujo principal del ejemplo.
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

    /// Modela la operacion `create product` dentro del ejemplo del patron.
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

    /// Modela la operacion `reserve stock` dentro del ejemplo del patron.
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
/// Conjunto de estados o errores publicos de `TransactionError` dentro del ejemplo.
pub enum TransactionError {
    /// Variante `ProductAlreadyExists` del estado o error del ejemplo.
    ProductAlreadyExists,
    /// Variante `ProductNotFound` del estado o error del ejemplo.
    ProductNotFound,
    /// Variante `InsufficientStock` del estado o error del ejemplo.
    InsufficientStock,
}
