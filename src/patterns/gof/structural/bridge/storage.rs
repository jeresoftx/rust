pub trait StorageProvider {
    fn write(&self, name: &str, contents: &str) -> String;
}

pub struct DocumentStore<P> {
    provider: P,
}

impl<P> DocumentStore<P>
where
    P: StorageProvider,
{
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    pub fn save(&self, name: &str, contents: &str) -> String {
        self.provider.write(name, contents)
    }
}

pub struct LocalStorageProvider {
    root_path: String,
}

impl LocalStorageProvider {
    pub fn new(root_path: impl Into<String>) -> Self {
        Self {
            root_path: root_path.into(),
        }
    }
}

impl StorageProvider for LocalStorageProvider {
    fn write(&self, name: &str, contents: &str) -> String {
        format!(
            "local path={}/{} bytes={}",
            self.root_path,
            name,
            contents.len()
        )
    }
}

pub struct CloudStorageProvider {
    bucket: String,
}

impl CloudStorageProvider {
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}

impl StorageProvider for CloudStorageProvider {
    fn write(&self, name: &str, contents: &str) -> String {
        format!(
            "cloud bucket={} key={} bytes={}",
            self.bucket,
            name,
            contents.len()
        )
    }
}
