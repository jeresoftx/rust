use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
/// Tipo publico `TemporaryFile` usado por el ejemplo para expresar el dominio del patron.
pub struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    /// Modela la operacion `create` dentro del ejemplo del patron.
    pub fn create(path: impl AsRef<Path>, contents: impl AsRef<str>) -> io::Result<Self> {
        fs::write(path.as_ref(), contents.as_ref())?;

        Ok(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Modela la operacion `path` dentro del ejemplo del patron.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Modela la operacion `read to string` dentro del ejemplo del patron.
    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Modela la operacion `append line` dentro del ejemplo del patron.
    pub fn append_line(&self, line: impl AsRef<str>) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.path)?;

        writeln!(file, "{}", line.as_ref())
    }
}

impl Drop for TemporaryFile {
    /// Operacion `drop` definida por la abstraccion del ejemplo.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
