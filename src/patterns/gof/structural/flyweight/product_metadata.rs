use std::collections::HashMap;
use std::sync::Arc;

/// Tipo publico `ProductMetadataCache` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductMetadataCache {
    metadata_by_family: HashMap<String, Arc<ProductMetadata>>,
}

impl ProductMetadataCache {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new() -> Self {
        Self {
            metadata_by_family: HashMap::new(),
        }
    }

    /// Modela la operacion `metadata` dentro del ejemplo del patron.
    pub fn metadata(&mut self, family: &str, description: &str) -> Arc<ProductMetadata> {
        self.metadata_by_family
            .entry(family.to_string())
            .or_insert_with(|| Arc::new(ProductMetadata::new(family, description)))
            .clone()
    }
}

impl Default for ProductMetadataCache {
    /// Operacion `default` definida por la abstraccion del ejemplo.
    fn default() -> Self {
        Self::new()
    }
}

/// Tipo publico `ProductMetadata` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductMetadata {
    family: String,
    description: String,
}

impl ProductMetadata {
    /// Operacion `new` definida por la abstraccion del ejemplo.
    fn new(family: &str, description: &str) -> Self {
        Self {
            family: family.to_string(),
            description: description.to_string(),
        }
    }
}

/// Tipo publico `ProductVariant` usado por el ejemplo para expresar el dominio del patron.
pub struct ProductVariant {
    sku: String,
    color: String,
    stock: u32,
    metadata: Arc<ProductMetadata>,
}

impl ProductVariant {
    /// Crea una instancia valida para el ejemplo del patron.
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

    /// Modela la operacion `shares metadata with` dentro del ejemplo del patron.
    pub fn shares_metadata_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.metadata, &other.metadata)
    }

    /// Devuelve un resumen legible del estado actual.
    pub fn summary(&self) -> String {
        format!(
            "sku={} color={} stock={} product={} description={}",
            self.sku, self.color, self.stock, self.metadata.family, self.metadata.description
        )
    }
}
