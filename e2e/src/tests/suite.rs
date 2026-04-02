use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::{Result, anyhow};
use log::*;
use thirtyfour::WebDriver;

use crate::{app::WebApp, driver::WebAppDriver, tests::{error::*, page::*}};

pub struct TestSuite;

#[derive(Debug, Clone)]
pub struct TestContext {
    pub app: Arc<WebApp>,
    pub driver: Arc<WebAppDriver>
}

impl TestContext {
    pub fn new(app: WebApp, driver: WebAppDriver) -> Self {
        Self {
            app: app.into(),
            driver: driver.into(),
        }
    }

    pub async fn reset(&self) -> Result<()> {
        // self.driver.delete_all_cookies().await?;

        // self.driver.execute(r#"
            
        // "#, vec![]).await?;

        self.driver.goto(self.app.base_url()).await?;

        Ok(())
    }
}

pub type TestFuture = Pin<Box<dyn Future<Output = Result<()>> + Send>>;

pub struct TestCase {
    pub name: &'static str,
    pub run: Box<dyn Fn(TestContext) -> TestFuture + Send + Sync>,
}

impl TestSuite {
    pub async fn run(context: TestContext) -> Result<()> {
        let tests = vec![
            should_have_title("Minesweeper".to_string()),
            should_show_error_popup(),
            should_()
        ];

        for test in tests {
            context.reset().await?;
            info!("Running {}", test.name);

            let result = (test.run)(context.clone()).await;

            match result {
                Ok(_) => info!("PASS {}", test.name),
                Err(e) => {
                    error!("FAIL {}: {:?}", test.name, e);
                    return Err(e);
                }
            }
        }

        let TestContext { driver, .. } = context;
        
        let driver = Arc::try_unwrap(driver)
            .map_err(|_| anyhow!("Driver still has multiple references"))?;

        let driver = driver.into_inner();
        driver.quit().await?;

        Ok(())
    }
}