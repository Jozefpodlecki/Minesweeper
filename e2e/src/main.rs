// #![allow(warnings)]

use std::time::Duration;

use flexi_logger::{Duplicate, FileSpec, Logger};
use log::*;
use anyhow::*;

use crate::{app::WebApp, driver::WebAppDriver, tests::{TestContext, TestSuite}};

mod utils;
mod driver;
mod app;
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
     Logger::try_with_str("debug")?
          .log_to_file(FileSpec::default())
          .duplicate_to_stdout(Duplicate::All)
          .start()?;

     let port = 1420;
     let timeout = Duration::from_secs(2);
     let app = WebApp::ensure_running(port, timeout).await?;

     let driver = WebAppDriver::ensure_running(timeout).await?;
     driver.goto(app.base_url()).await?;
     driver.refresh().await?;

     let context = TestContext::new(app, driver);
     TestSuite::run(context).await?;

     Ok(())
}