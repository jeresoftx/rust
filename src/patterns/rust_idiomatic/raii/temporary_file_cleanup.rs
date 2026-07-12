use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    pub fn create(path: impl AsRef<Path>, contents: impl AsRef<str>) -> io::Result<Self> {
        fs::write(path.as_ref(), contents.as_ref())?;

        Ok(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }

    pub fn append_line(&self, line: impl AsRef<str>) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.path)?;

        writeln!(file, "{}", line.as_ref())
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
