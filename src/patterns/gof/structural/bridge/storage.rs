/// Contrato publico `StorageProvider` que desacopla las piezas del ejemplo.
pub trait StorageProvider {
    /// Operacion `write` definida por la abstraccion del ejemplo.
    fn write(&self, name: &str, contents: &str) -> String;
}

/// Tipo publico `DocumentStore` usado por el ejemplo para expresar el dominio del patron.
pub struct DocumentStore<P> {
    provider: P,
}

impl<P> DocumentStore<P>
where
    P: StorageProvider,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Modela la operacion `save` dentro del ejemplo del patron.
    pub fn save(&self, name: &str, contents: &str) -> String {
        self.provider.write(name, contents)
    }
}

/// Tipo publico `LocalStorageProvider` usado por el ejemplo para expresar el dominio del patron.
pub struct LocalStorageProvider {
    root_path: String,
}

impl LocalStorageProvider {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(root_path: impl Into<String>) -> Self {
        Self {
            root_path: root_path.into(),
        }
    }
}

impl StorageProvider for LocalStorageProvider {
    /// Operacion `write` definida por la abstraccion del ejemplo.
    fn write(&self, name: &str, contents: &str) -> String {
        format!(
            "local path={}/{} bytes={}",
            self.root_path,
            name,
            contents.len()
        )
    }
}

/// Tipo publico `CloudStorageProvider` usado por el ejemplo para expresar el dominio del patron.
pub struct CloudStorageProvider {
    bucket: String,
}

impl CloudStorageProvider {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl StorageProvider for CloudStorageProvider {
    /// Operacion `write` definida por la abstraccion del ejemplo.
    fn write(&self, name: &str, contents: &str) -> String {
        format!(
            "cloud bucket={} key={} bytes={}",
            self.bucket,
            name,
            contents.len()
        )
    }
}
