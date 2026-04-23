use std::time::Duration;

use super::super::super::poller::Poller;
use super::super::super::qbittorrent_client::QbittorrentClient;

/// Waits until the client reports at least one torrent in its list.
///
/// This is a presence/registration barrier for the asynchronous add-torrent flow.
/// It does not guarantee seeding, downloading, or completion state.
///
/// # Errors
///
/// Returns an error when polling times out or the torrent list query fails.
pub async fn wait_until_client_has_any_torrent(
    client: &QbittorrentClient,
    timeout: Duration,
    poll_interval: Duration,
    client_name: &str,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);

    loop {
        if client.has_any_torrents().await? {
            tracing::info!("{client_name} has at least one torrent");
            return Ok(());
        }

        let torrent_count = client.torrent_count().await?;
        tracing::info!("{client_name} has {torrent_count} torrent(s)");

        poller
            .retry_or_timeout(|| {
                format!("timed out waiting for {client_name} torrent presence: {client_name} has {torrent_count}")
            })
            .await?;
    }
}
