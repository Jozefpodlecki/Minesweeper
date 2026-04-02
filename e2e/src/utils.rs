use log::*;
use std::{env, time::Duration};
use anyhow::{anyhow, Result};
use reqwest::Client;
use tokio::{io::BufReader, task, time::sleep};
use std::process::{Child, Command, Stdio};
use tokio::io::AsyncBufReadExt;

pub fn start_trunk() -> Result<Child> {

    let mut app_dir = env::current_dir()?;

    if app_dir.file_name().map(|n| n == "e2e").unwrap_or(false) {
        app_dir.pop();
        info!("Detected running from `e2e` folder, moving up to project root: {}", app_dir.display());
    }

    let mut child = Command::new("trunk")
        .arg("serve")
        .current_dir(app_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        task::spawn(async move {
            let reader = BufReader::new(tokio::process::ChildStdout::from_std(stdout).unwrap());
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                info!("[trunk stdout] {}", line);
            }
        });
    }

    Ok(child)
}

pub async fn wait_for_http(url: &str, timeout: Duration) -> Result<()> {
    let client = Client::new();
    let interval = Duration::from_millis(300);
    let mut elapsed = Duration::ZERO;

    while elapsed < timeout {
        info!("Trying to reach {}", url);
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => {
                return Ok(());
            }
            _ => {}
        }

        info!("Sleeping for {}ms...", interval.as_millis());
        sleep(interval).await;
        elapsed += interval;
    }

    Err(anyhow!("Timeout waiting for {}", url))
}