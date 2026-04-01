use std::path::Path;
use log::error;


pub trait Minifier: Clone + Send + Sync {
    fn minify_js(&self, code: &str, path: &Path) -> String;
}

#[derive(Default, Clone)]
pub struct RealMinifier;

impl Minifier for RealMinifier {
    fn minify_js(&self, code: &str, path: &Path) -> String {
        let session = minify_js::Session::new();
        let mut out = Vec::new();
        
        match minify_js::minify(&session, minify_js::TopLevelMode::Global, code.as_bytes(), &mut out) {
            Ok(_) => String::from_utf8_lossy(&out).into_owned(),
            Err(err) => {
                error!("Failed to minify {}: {}", path.display(), err);
                code.to_string()
            }
        }
    }
}

#[derive(Default, Clone)]
#[allow(unused)]
pub struct NoOpMinifier;

impl Minifier for NoOpMinifier {
    fn minify_js(&self, code: &str, _path: &Path) -> String {
        code.to_string()
    }
}