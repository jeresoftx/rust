use std::collections::HashMap;

/// Tipo publico `ExternalProductApi` usado por el ejemplo para expresar el dominio del patron.
pub struct ExternalProductApi {
    call_count: usize,
}

impl ExternalProductApi {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self { call_count: 0 }
    }

    /// Operacion `fetch product name` definida por la abstraccion del ejemplo.
    fn fetch_product_name(&mut self, product_id: &str) -> String {
        self.call_count += 1;
        format!("Product {product_id}")
    }

    /// Operacion `call count` definida por la abstraccion del ejemplo.
    fn call_count(&self) -> usize {
        self.call_count
    }
}

impl Default for ExternalProductApi {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}

/// Tipo publico `CachedApiProxy` usado por el ejemplo para expresar el dominio del patron.
pub struct CachedApiProxy {
    api: ExternalProductApi,
    cache: HashMap<String, String>,
}

impl CachedApiProxy {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(api: ExternalProductApi) -> Self {
        Self {
            api,
            cache: HashMap::new(),
        }
    }

    /// Modela la operacion `product name` dentro del ejemplo del patron.
    pub fn product_name(&mut self, product_id: &str) -> String {
        if let Some(name) = self.cache.get(product_id) {
            return name.clone();
        }

        let name = self.api.fetch_product_name(product_id);
        self.cache.insert(product_id.to_string(), name.clone());
        name
    }

    /// Modela la operacion `real api call count` dentro del ejemplo del patron.
    pub fn real_api_call_count(&self) -> usize {
        self.api.call_count()
    }
}
