use std::{ffi::OsStr, ops::Deref, process::{Child, Command}, time::Duration};
// use sysinfo::{System, SystemExt, ProcessExt};
use anyhow::{Result, Context};
use sysinfo::System;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::time::sleep;
use log::*;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags};

#[derive(Debug)]
pub struct WebAppDriver {
    pub driver: WebDriver,
    pub child: Option<Child>,
    pub pid: u32,
    pub port: u16,
}

impl Deref for WebAppDriver {
    type Target = WebDriver;

    fn deref(&self) -> &Self::Target {
        &self.driver
    }
}

impl WebAppDriver  {

    pub fn into_inner(self) -> WebDriver {
        self.driver
    }

    pub async fn create_driver(port: u16) -> Result<WebDriver> {
        let server_url = format!("http://localhost:{}/", port);

        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new(&server_url, caps).await?;

        Ok(driver)
    }

    pub async fn ensure_running(timeout: Duration) -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();

        let process_name = OsStr::new("chromedriver");
        let chrome_driver = system.processes_by_exact_name(process_name).next();

        if let Some(proc) = chrome_driver {
            let pid = proc.pid().as_u32();
            let port = Self::find_port(pid)?;
            let driver = Self::create_driver(port).await?;
            info!("Chromedriver running at PID {} port {}", pid, port);

            Ok(Self { driver, child: None, pid, port })
        } else {
            debug!("Launching chromedriver");
            let chrome_driver_path = r"C:\chromedriver-win64\chromedriver.exe";
            let child = Command::new(chrome_driver_path)
                .spawn()
                .with_context(|| "Could not spawn chromedriver")?;

            let pid = child.id();
            let port = Self::wait_for_port(pid, timeout).await?;
            let driver = Self::create_driver(port).await?;
            
            Ok(Self { driver, child: Some(child), pid, port })
        }
    }

    fn find_port(pid: u32) -> Result<u16> {
        let flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let protocols = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets = get_sockets_info(flags, protocols)
            .unwrap();

        sockets.into_iter()
            .find(|s| s.associated_pids.contains(&pid))
            .map(|s| s.local_port())
            .ok_or_else(|| anyhow::anyhow!("Could not find port for chromedriver"))
    }

    async fn wait_for_port(pid: u32, timeout: Duration) -> Result<u16> {
        let mut elapsed = Duration::ZERO;
        let interval = Duration::from_millis(250);

        while elapsed < timeout {
            if let Ok(port) = Self::find_port(pid) {
                return Ok(port);
            }
            sleep(interval).await;
            elapsed += interval;
        }

        Err(anyhow::anyhow!("Timeout waiting for chromedriver port"))
    }
}