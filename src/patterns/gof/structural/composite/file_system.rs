pub enum FileSystemEntry {
    File(FileEntry),
    Folder(Folder),
}

impl FileSystemEntry {
    fn total_size(&self) -> u64 {
        match self {
            Self::File(file) => file.size,
            Self::Folder(folder) => folder.total_size(),
        }
    }

    fn collect_paths(&self, prefix: &str, paths: &mut Vec<String>) {
        match self {
            Self::File(file) => paths.push(format!("{prefix}/{}", file.name)),
            Self::Folder(folder) => folder.collect_paths(prefix, paths),
        }
    }
}

impl From<FileEntry> for FileSystemEntry {
    fn from(file: FileEntry) -> Self {
        Self::File(file)
    }
}

impl From<Folder> for FileSystemEntry {
    fn from(folder: Folder) -> Self {
        Self::Folder(folder)
    }
}

pub struct FileEntry {
    name: String,
    size: u64,
}

impl FileEntry {
    pub fn new(name: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

pub struct Folder {
    name: String,
    children: Vec<FileSystemEntry>,
}

impl Folder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn with(mut self, entry: impl Into<FileSystemEntry>) -> Self {
        self.children.push(entry.into());
        self
    }

    pub fn total_size(&self) -> u64 {
        self.children.iter().map(FileSystemEntry::total_size).sum()
    }

    pub fn file_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths("", &mut paths);
        paths
    }

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
