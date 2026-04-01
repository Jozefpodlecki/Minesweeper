use std::path::{Path, PathBuf};
use anyhow::Result;
use regex::Regex;
use log::info;

use crate::readers::FileReader;
use crate::minifiers::Minifier;
use crate::placeholders::{Placeholder, PlaceholderHandler, DataUrlHandler, MinifyHandler, TextHandler};

/// Main template processor that can be configured with different implementations
#[derive(Clone)]
pub struct TemplateProcessor<R: FileReader, M: Minifier> {
    reader: R,
    minifier: M,
    placeholder_regex: Regex,
}

impl<R: FileReader, M: Minifier> TemplateProcessor<R, M> {
    pub fn new(reader: R, minifier: M) -> Result<Self> {
        Ok(Self {
            reader,
            minifier,
            placeholder_regex: Regex::new(r"\{([^\{\}:]+)(?::([^\{\}]+))?\}")?,
        })
    }

    /// Process a template file
    pub fn process_file(&self, template_path: PathBuf, output_path: PathBuf) -> Result<()> {
        info!("Reading template: {}", template_path.display());
        let contents = self.reader.read_to_string(&template_path)?;
        
        let processed = self.process_template(&contents, &template_path)?;
        
        info!("Writing output: {}", output_path.display());
        std::fs::write(output_path, processed)?;
        
        Ok(())
    }

    /// Process template content
    pub fn process_template(&self, content: &str, template_path: &Path) -> Result<String> {
        let base_path = template_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        
        let result = self.placeholder_regex.replace_all(content, |caps: &regex::Captures| {
            let placeholder = self.parse_placeholder(caps);
            self.handle_placeholder(&placeholder, &base_path)
        });
        
        Ok(result.to_string())
    }

    fn parse_placeholder(&self, caps: &regex::Captures) -> Placeholder {
        Placeholder {
            file_path: caps[1].to_string(),
            modifier: caps.get(2).map(|m| m.as_str().to_string()),
            full_match: caps[0].to_string(),
        }
    }

    fn handle_placeholder(&self, placeholder: &Placeholder, base_path: &Path) -> String {
        let full_path = base_path.join(&placeholder.file_path);

        match placeholder.modifier.as_deref() {
            Some("dataUrl") => {
                info!("Processing dataUrl placeholder: {} -> {}", placeholder.file_path, full_path.display());
                let handler = DataUrlHandler::new(self.reader.clone());
                handler.handle(placeholder, base_path)
            }
            Some("minify") => {
                info!("Processing minify placeholder: {} -> {}", placeholder.file_path, full_path.display());
                let handler = MinifyHandler::new(self.minifier.clone(), self.reader.clone());
                handler.handle(placeholder, base_path)
            }
            _ => {
                let handler = TextHandler::new(self.reader.clone());
                handler.handle(placeholder, base_path)
            }
        }
    }
}

pub type DefaultProcessor = TemplateProcessor<crate::readers::RealFileReader, crate::minifiers::RealMinifier>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readers::MockFileReader;
    use crate::minifiers::NoOpMinifier;

    fn setup_test_processor() -> TemplateProcessor<MockFileReader, NoOpMinifier> {
        let mut reader = MockFileReader::new();
        
        // Setup test files
        reader.add_text_file("foo.js", "function hello() { console.log('hi'); }");
        reader.add_text_file("text.txt", "Hello World");
        reader.add_file("image.png", b"PNGDATA".to_vec());
        
        TemplateProcessor::new(reader, NoOpMinifier).unwrap()
    }

    #[test]
    fn should_process_text_placeholder() {
        let processor = setup_test_processor();
        let template = "<div>{text.txt}</div>";
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        assert_eq!(result, "<div>Hello World</div>");
    }

    #[test]
    fn should_inline_data_url() {
        let processor = setup_test_processor();
        let template = "<img src=\"{image.png:dataUrl}\">";
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        assert!(result.starts_with("<img src=\"data:image/"));
        assert!(result.contains("base64"));
    }

    #[test]
    fn should_minify_js() {
        let processor = setup_test_processor();
        let template = "<script>{foo.js:minify}</script>";
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        // With NoOpMinifier, content should be unchanged
        assert!(result.contains("function hello()"));
        assert!(result.contains("console.log"));
    }

    #[test]
    fn should_handle_missing_file_gracefully() {
        let processor = setup_test_processor();
        let template = "<div>{missing.txt}</div>";
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn should_handle_multiple_placeholders() {
        let processor = setup_test_processor();
        let template = r#"
            <script>{foo.js:minify}</script>
            <img src="{image.png:dataUrl}">
            <div>{text.txt}</div>
        "#;
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        
        assert!(result.contains("function hello()"));
        assert!(result.contains("data:image/"));
        assert!(result.contains("Hello World"));
    }

    #[test]
    fn should_resolve_paths_correctly() {
        let processor = setup_test_processor();
        let template = "<div>{subdir/text.txt}</div>";
        let template_path = PathBuf::from("templates/page.html");
        
        // This will look for templates/subdir/text.txt which doesn't exist in our mock
        let result = processor.process_template(template, &template_path).unwrap();
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn should_preserve_non_placeholder_content() {
        let processor = setup_test_processor();
        let template = "<html><body>Hello {text.txt}!</body></html>";
        let template_path = PathBuf::from(".");
        
        let result = processor.process_template(template, &template_path).unwrap();
        assert_eq!(result, "<html><body>Hello Hello World!</body></html>");
    }
}