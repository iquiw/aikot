use std::fs::remove_file;
use std::path::{Path, PathBuf};

pub struct TempPath(PathBuf);

impl TempPath {
    pub fn new(path: PathBuf) -> Self {
        TempPath(path)
    }
}

impl AsRef<Path> for TempPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        if let Err(e) = remove_file(self.as_ref()) {
            eprintln!("Unable to delete {}, {}", self.0.display(), e);
        }
    }
}
