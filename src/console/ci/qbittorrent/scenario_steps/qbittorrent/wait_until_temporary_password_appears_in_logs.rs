use std::time::Duration;

use super::super::super::poller::Poller;
use crate::console::ci::compose::DockerCompose;

/// Waits until qBittorrent logs expose a temporary `WebUI` password and returns it.
///
/// # Errors
///
/// Returns an error when no temporary password is discovered before timeout.
pub async fn wait_until_temporary_password_appears_in_logs(
    compose: &DockerCompose,
    service_name: &str,
    timeout: Duration,
    poll_interval: Duration,
) -> anyhow::Result<String> {
    let poller = Poller::new(timeout, poll_interval);

    loop {
        if let Ok(logs) = compose.logs(&[service_name]) {
            if let Some(password) = extract_temporary_webui_password(&logs) {
                return Ok(password);
            }
        }

        // TODO: Avoid log parsing by provisioning deterministic credentials during startup.
        // Investigate injecting WebUI credentials through config/environment before container launch.
        poller
            .retry_or_timeout(|| {
                format!("timed out waiting for temporary qBittorrent password in logs for service '{service_name}'")
            })
            .await?;
    }
}

fn extract_temporary_webui_password(logs: &str) -> Option<String> {
    const PREFIX: &str = "A temporary password is provided for this session:";

    logs.lines()
        .rev()
        .find_map(|line| line.split_once(PREFIX).map(|(_, password)| password.trim().to_string()))
        .filter(|password| !password.is_empty())
}
