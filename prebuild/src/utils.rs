use std::fs;
use std::path::Path;
use base64::Engine;
use base64::{engine::general_purpose, Engine as _};

pub fn read_file(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn file_to_data_url(path: &Path) -> String {
    let data = fs::read(path).unwrap_or_default();

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

    let base64 = general_purpose::STANDARD.encode(&data);

    format!("data:{};base64,{}", mime, base64)
}