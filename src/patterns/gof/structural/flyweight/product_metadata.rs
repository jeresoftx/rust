use std::collections::HashMap;
use std::sync::Arc;

pub struct ProductMetadataCache {
    metadata_by_family: HashMap<String, Arc<ProductMetadata>>,
}

impl ProductMetadataCache {
    pub fn new() -> Self {
        Self {
            metadata_by_family: HashMap::new(),
        }
    }

    pub fn metadata(&mut self, family: &str, description: &str) -> Arc<ProductMetadata> {
        self.metadata_by_family
            .entry(family.to_string())
            .or_insert_with(|| Arc::new(ProductMetadata::new(family, description)))
            .clone()
    }
}

impl Default for ProductMetadataCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProductMetadata {
    family: String,
    description: String,
}

impl ProductMetadata {
    fn new(family: &str, description: &str) -> Self {
        Self {
            family: family.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct ProductVariant {
    sku: String,
    color: String,
    stock: u32,
    metadata: Arc<ProductMetadata>,
}

impl ProductVariant {
    pub fn new(
        sku: impl Into<String>,
        color: impl Into<String>,
        stock: u32,
        metadata: Arc<ProductMetadata>,
    ) -> Self {
        Self {
            sku: sku.into(),
            color: color.into(),
            stock,
            metadata,
        }
    }

    pub fn shares_metadata_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.metadata, &other.metadata)
    }

    pub fn summary(&self) -> String {
        format!(
            "sku={} color={} stock={} product={} description={}",
            self.sku, self.color, self.stock, self.metadata.family, self.metadata.description
        )
    }
}
