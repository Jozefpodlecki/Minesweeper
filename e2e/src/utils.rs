use std::time::Duration;
use anyhow::{anyhow, Result};
use reqwest::Client;
use tokio::time::sleep;
use std::process::{Child, Command, Stdio};

pub fn start_trunk() -> Result<Child> {
    let child = Command::new("trunk")
        .arg("serve")
        .stdout(Stdio::null())   // or piped() if you want logs
        .stderr(Stdio::null())
        .spawn()?;

    Ok(child)
}

pub async fn wait_for_http(url: &str, timeout: Duration) -> Result<()> {
    let client = Client::new();
    let interval = Duration::from_millis(300);
    let mut elapsed = Duration::ZERO;

    while elapsed < timeout {
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => {
                return Ok(());
            }
            _ => {}
        }

        sleep(interval).await;
        elapsed += interval;
    }

    Err(anyhow!("Timeout waiting for {}", url))
}