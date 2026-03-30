#![allow(warnings)]

use std::error::Error;
use std::ffi::OsStr;
use std::process::Command;
use std::{env, fs::File, io::Write};
use anyhow::{anyhow, Context, Result};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags};
use sysinfo::System;
use thirtyfour::{common::config::WebDriverConfig, prelude::*};
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {

     let mut system = System::new_all();
     system.refresh_all();

     let process_name = OsStr::new("chromedriver");
     let chrome_driver = system.processes_by_exact_name(process_name).next();

     let chrome_driver_pid = match chrome_driver {
          Some(value) => {
               value.pid().as_u32()
          },
          None => {
               let child = Command::new(r"C:\chromedriver-win64\chromedriver.exe")
                    .spawn()
                    .with_context(|| "Could not spawn chromedriver")?;

               child.id()
          },
     };

     let sockets_info = get_sockets_info(
        AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
        ProtocolFlags::TCP | ProtocolFlags::UDP,
    ).unwrap();

    let sockets_info = sockets_info.into_iter()
          .filter(|pr| pr.associated_pids.contains(&chrome_driver_pid))
          .next();

     let local_port = match sockets_info {
          Some(value) => value.local_port(),
          None => return Err(anyhow!("Could not find")),
     };

     // chromedriver.exe
     // https://googlechromelabs.github.io/chrome-for-testing/#stable
     let caps = DesiredCapabilities::chrome();
     let server_url = format!("http://localhost:{local_port}/");
     let driver = WebDriver::new(server_url, caps).await?;

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