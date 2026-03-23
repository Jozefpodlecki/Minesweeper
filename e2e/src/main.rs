#![allow(warnings)]

use std::{env, fs::File, io::Write};

use thirtyfour::{common::config::WebDriverConfig, prelude::*};
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> WebDriverResult<()> {

     // chromedriver.exe
     // https://googlechromelabs.github.io/chrome-for-testing/#stable
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:51275/", caps).await?;

     driver.goto("http://localhost.:1420").await?;
     driver.refresh().await?;

     let title = driver.title().await?;
     assert!(title.contains("Minesweeper"));

     let play_button = driver.find_element(By::Css("[data-action='play']")).await?;
     play_button.click();
 
     sleep(Duration::from_secs(5)).await;
     
     driver.quit().await?;
     Ok(())
}