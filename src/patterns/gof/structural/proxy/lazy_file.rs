/// Tipo publico `LargeFile` usado por el ejemplo para expresar el dominio del patron.
pub struct LargeFile {
    name: String,
    content: String,
    load_count: usize,
}

impl LargeFile {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
            load_count: 0,
        }
    }

    /// Operacion `name` definida por la abstraccion del ejemplo.
    fn name(&self) -> &str {
        &self.name
    }

    /// Operacion `load content` definida por la abstraccion del ejemplo.
    fn load_content(&mut self) -> String {
        self.load_count += 1;
        self.content.clone()
    }

    /// Operacion `load count` definida por la abstraccion del ejemplo.
    fn load_count(&self) -> usize {
        self.load_count
    }
}

/// Tipo publico `LazyFileProxy` usado por el ejemplo para expresar el dominio del patron.
pub struct LazyFileProxy {
    file: LargeFile,
    loaded_content: Option<String>,
}

impl LazyFileProxy {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(file: LargeFile) -> Self {
        Self {
            file,
            loaded_content: None,
        }
    }

    /// Modela la operacion `file name` dentro del ejemplo del patron.
    pub fn file_name(&self) -> &str {
        self.file.name()
    }

    /// Modela la operacion `content` dentro del ejemplo del patron.
    pub fn content(&mut self) -> &str {
        if self.loaded_content.is_none() {
            self.loaded_content = Some(self.file.load_content());
        }

        self.loaded_content.as_deref().unwrap_or("")
    }

    /// Modela la operacion `load count` dentro del ejemplo del patron.
    pub fn load_count(&self) -> usize {
        self.file.load_count()
    }
}
