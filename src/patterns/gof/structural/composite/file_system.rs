/// Conjunto de estados o errores publicos de `FileSystemEntry` dentro del ejemplo.
pub enum FileSystemEntry {
    /// Variante `File` del estado o error del ejemplo.
    File(FileEntry),
    /// Variante `Folder` del estado o error del ejemplo.
    Folder(Folder),
}

impl FileSystemEntry {
    /// Operacion `total size` definida por la abstraccion del ejemplo.
    fn total_size(&self) -> u64 {
        match self {
            Self::File(file) => file.size,
            Self::Folder(folder) => folder.total_size(),
        }
    }

    /// Operacion `collect paths` definida por la abstraccion del ejemplo.
    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        match self {
            Self::File(file) => paths.push(format!("{prefix}/{}", file.name)),
            Self::Folder(folder) => folder.collect_paths(prefix, paths),
        }
    }
}

impl From<FileEntry> for FileSystemEntry {
    /// Operacion `from` definida por la abstraccion del ejemplo.
    fn from(file: FileEntry) -> Self {
        Self::File(file)
    }
}

impl From<Folder> for FileSystemEntry {
    /// Operacion `from` definida por la abstraccion del ejemplo.
    fn from(folder: Folder) -> Self {
        Self::Folder(folder)
    }
}

/// Tipo publico `FileEntry` usado por el ejemplo para expresar el dominio del patron.
pub struct FileEntry {
    name: String,
    size: u64,
}

impl FileEntry {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

/// Tipo publico `Folder` usado por el ejemplo para expresar el dominio del patron.
pub struct Folder {
    name: String,
    children: Vec<FileSystemEntry>,
}

impl Folder {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    /// Modela la operacion `with` dentro del ejemplo del patron.
    pub fn with(mut self, entry: impl Into<FileSystemEntry>) -> Self {
        self.children.push(entry.into());
        self
    }

    /// Modela la operacion `total size` dentro del ejemplo del patron.
    pub fn total_size(&self) -> u64 {
        self.children.iter().map(FileSystemEntry::total_size).sum()
    }

    /// Modela la operacion `file paths` dentro del ejemplo del patron.
    pub fn file_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths("", &mut paths);
        paths
    }

    /// Operacion `collect paths` definida por la abstraccion del ejemplo.
    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        let folder_path = if prefix.is_empty() {
            self.name.clone()
        } else {
            format!("{prefix}/{}", self.name)
        };

        for child in &self.children {
            child.collect_paths(&folder_path, paths);
        }
    }
}
