use anyhow::Result;
use std::{process::Child, time::Duration};
use log::*;
use crate::utils::{start_trunk, wait_for_http};

#[derive(Debug)]
pub struct WebApp {
    pub base_url: String,
    pub child: Option<Child>,
}

impl Drop for WebApp {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            info!("Killing child process…");
            if let Err(e) = child.kill() {
                warn!("Failed to kill child process: {:?}", e);
            } else {
                let _ = child.wait();
            }
        }
    }
}

impl WebApp {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn ensure_running(port: u16, timeout: Duration) -> Result<Self> {
        let base_url = format!("http://localhost:{port}");

        if wait_for_http(&base_url, Duration::from_secs(1)).await.is_ok() {
            info!("App already running");
            return Ok(Self { base_url, child: None });
        }

        info!("Starting trunk serve...");
        let child = start_trunk()?;

        wait_for_http(&base_url, timeout).await?;

        Ok(Self { base_url, child: Some(child) })
    }
}