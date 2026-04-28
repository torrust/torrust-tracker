use super::super::super::poller::Poller;
use super::super::super::qbittorrent::{QbittorrentClient, QbittorrentCredentials};
use super::super::super::types::{Deadline, PollInterval};

/// Attempts login using provided credentials and retries until accepted.
///
/// # Errors
///
/// Returns an error when login does not succeed before timeout.
pub async fn login_client(
    client: &QbittorrentClient,
    credentials: &QbittorrentCredentials,
    timeout: Deadline,
    poll_interval: PollInterval,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);
    let client_label = client.label();

    loop {
        let last_error = match client.login(credentials).await {
            Ok(()) => {
                tracing::info!(client = client_label, "qBittorrent WebUI login succeeded");
                return Ok(());
            }
            Err(error) => error.to_string(),
        };

        tracing::info!(
            client = client_label,
            error = last_error,
            "waiting for qBittorrent WebUI authentication"
        );

        poller
            .retry_or_timeout(|| {
                format!("timed out waiting for qBittorrent WebUI authentication readiness. Last error: {last_error}")
            })
            .await?;
    }
}
