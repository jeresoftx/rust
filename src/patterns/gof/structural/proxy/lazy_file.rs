pub struct LargeFile {
    name: String,
    content: String,
    load_count: usize,
}

impl LargeFile {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
            load_count: 0,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn load_content(&mut self) -> String {
        self.load_count += 1;
        self.content.clone()
    }

    fn load_count(&self) -> usize {
        self.load_count
    }
}

pub struct LazyFileProxy {
    file: LargeFile,
    loaded_content: Option<String>,
}

impl LazyFileProxy {
    pub fn new(file: LargeFile) -> Self {
        Self {
            file,
            loaded_content: None,
        }
    }

    pub fn file_name(&self) -> &str {
        self.file.name()
    }

    pub fn content(&mut self) -> &str {
        if self.loaded_content.is_none() {
            self.loaded_content = Some(self.file.load_content());
        }

        self.loaded_content.as_deref().unwrap_or("")
    }

    pub fn load_count(&self) -> usize {
        self.file.load_count()
    }
}
