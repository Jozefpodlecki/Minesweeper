use std::path::Path;
use std::io;
use anyhow::Result;

pub trait FileReader: Clone + Send + Sync {
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn read_bytes(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn file_exists(&self, path: &Path) -> bool;
}

#[derive(Default, Clone)]
pub struct RealFileReader;

impl FileReader for RealFileReader {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn read_bytes(&self, path: &Path) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

#[derive(Clone, Default)]
#[allow(unused)]
pub struct MockFileReader {
    files: std::collections::HashMap<String, Vec<u8>>,
}

#[allow(unused)]
impl MockFileReader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: &str, content: impl Into<Vec<u8>>) {
        self.files.insert(path.to_string(), content.into());
    }

    pub fn add_text_file(&mut self, path: &str, content: &str) {
        self.add_file(path, content.as_bytes().to_vec());
    }
}

impl FileReader for MockFileReader {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.read_bytes(path).and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
    }

    fn read_bytes(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.files
            .get(path.to_str().unwrap_or(""))
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn file_exists(&self, path: &Path) -> bool {
        self.files.contains_key(path.to_str().unwrap_or(""))
    }
}