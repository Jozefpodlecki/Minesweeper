use std::env;

use anyhow::Result;
use flexi_logger::Logger;

use crate::processor::DefaultProcessor;
mod readers;
mod processor;
mod utils;
mod minifiers;
mod placeholders;

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    
    let current_dir = env::current_dir()?;
    let template_path = current_dir.join("prebuild").join("index.template.html");
    let output_path = current_dir.join("index.html");

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let processor = DefaultProcessor::new(
        readers::RealFileReader,
        minifiers::RealMinifier,
    )?;

    processor.process_file(template_path, output_path)?;

    Ok(())
}