#![allow(warnings)]

use std::{env, fs::File, io::Write};

use thirtyfour::{common::config::WebDriverConfig, prelude::*};
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> WebDriverResult<()> {

     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:61106/", caps).await?;

     driver.goto("http://localhost.:1420").await?;
     driver.refresh().await?;

     let title = driver.title().await?;
     assert!(title.contains("Jozef Podlecki"));

     let dark_mode_button = driver.find_element(By::Css("[data-dark-mode-toggle='true']")).await?;
     dark_mode_button.click().await?;
     
     let github_link = driver.find_element(By::Css("a[href*='github.com']")).await?;
     assert!(github_link.is_displayed().await?);

     let projects_section = driver.find_element(By::XPath("//*[text()='Projects']")).await?;
     assert!(projects_section.is_displayed().await?);
 
     let experience_section = driver.find_element(By::XPath("//*[text()='Experience']")).await?;
     assert!(experience_section.is_displayed().await?);

     sleep(Duration::from_secs(5)).await;
     
     driver.quit().await?;
     Ok(())
}