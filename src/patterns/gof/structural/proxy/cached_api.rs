use std::collections::HashMap;

pub struct ExternalProductApi {
    call_count: usize,
}

impl ExternalProductApi {
    pub fn new() -> Self {
        Self { call_count: 0 }
    }

    fn fetch_product_name(&mut self, product_id: &str) -> String {
        self.call_count += 1;
        format!("Product {product_id}")
    }

    fn call_count(&self) -> usize {
        self.call_count
    }
}

impl Default for ExternalProductApi {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CachedApiProxy {
    api: ExternalProductApi,
    cache: HashMap<String, String>,
}

impl CachedApiProxy {
    pub fn new(api: ExternalProductApi) -> Self {
        Self {
            api,
            cache: HashMap::new(),
        }
    }

    pub fn product_name(&mut self, product_id: &str) -> String {
        if let Some(name) = self.cache.get(product_id) {
            return name.clone();
        }

        let name = self.api.fetch_product_name(product_id);
        self.cache.insert(product_id.to_string(), name.clone());
        name
    }

    pub fn real_api_call_count(&self) -> usize {
        self.api.call_count()
    }
}
