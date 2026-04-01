use std::path::Path;
use base64::Engine;
use crate::readers::FileReader;
use crate::minifiers::Minifier;

#[derive(Debug, PartialEq)]
pub struct Placeholder {
    pub file_path: String,
    pub modifier: Option<String>,
    pub full_match: String,
}

pub trait PlaceholderHandler: Send + Sync {
    fn handle(&self, placeholder: &Placeholder, base_path: &Path) -> String;
}

pub struct DataUrlHandler<R: FileReader> {
    reader: R,
}

impl<R: FileReader> DataUrlHandler<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn file_to_data_url(&self, path: &Path) -> String {
        let data = self.reader.read_bytes(path).unwrap_or_default();
        
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")
            .to_lowercase();

        let mime = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "svg" => "image/svg+xml",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        };

        let base64 = base64::engine::general_purpose::STANDARD.encode(&data);
        format!("data:{};base64,{}", mime, base64)
    }
}

impl<R: FileReader> PlaceholderHandler for DataUrlHandler<R> {
    fn handle(&self, placeholder: &Placeholder, base_path: &Path) -> String {
        let resolved_path = base_path.join(&placeholder.file_path);
        self.file_to_data_url(&resolved_path)
    }
}

/// Handler for minify placeholders
pub struct MinifyHandler<M: Minifier, R: FileReader> {
    minifier: M,
    reader: R,
}

impl<M: Minifier, R: FileReader> MinifyHandler<M, R> {
    pub fn new(minifier: M, reader: R) -> Self {
        Self { minifier, reader }
    }
}

impl<M: Minifier, R: FileReader> PlaceholderHandler for MinifyHandler<M, R> {
    fn handle(&self, placeholder: &Placeholder, base_path: &Path) -> String {
        let resolved_path = base_path.join(&placeholder.file_path);
        
        match self.reader.read_to_string(&resolved_path) {
            Ok(code) => self.minifier.minify_js(&code, &resolved_path),
            Err(_) => String::new(),
        }
    }
}

pub struct TextHandler<R: FileReader> {
    reader: R,
}

impl<R: FileReader> TextHandler<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: FileReader> PlaceholderHandler for TextHandler<R> {
    fn handle(&self, placeholder: &Placeholder, base_path: &Path) -> String {
        let resolved_path = base_path.join(&placeholder.file_path);
        self.reader.read_to_string(&resolved_path).unwrap_or_default()
    }
}