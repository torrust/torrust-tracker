use super::super::super::poller::Poller;
use super::super::super::qbittorrent::QbittorrentClient;
use super::super::super::types::{Deadline, PollInterval};

/// Waits until the client first torrent reaches full completion.
///
/// # Errors
///
/// Returns an error when polling times out or the torrent list query fails.
pub async fn wait_until_download_completes(
    client: &QbittorrentClient,
    timeout: Deadline,
    poll_interval: PollInterval,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);

    loop {
        if let Some(torrent) = client.first_torrent().await? {
            tracing::info!(
                "Torrent progress: {:.1}% (state: {})",
                torrent.progress.as_fraction() * 100.0,
                torrent.state
            );

            if torrent.progress.is_complete() {
                tracing::info!("Torrent download complete (100%)");
                return Ok(());
            }
        }

        poller
            .retry_or_timeout(|| "timed out waiting for download to complete".to_string())
            .await?;
    }
}
