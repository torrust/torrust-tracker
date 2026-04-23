use std::time::Duration;

use super::super::poller::Poller;
use super::super::qbittorrent_client::QbittorrentClient;

/// Waits until the client reports at least one torrent in its list.
///
/// This is a presence/registration barrier for the asynchronous add-torrent flow.
/// It does not guarantee seeding, downloading, or completion state.
///
/// # Errors
///
/// Returns an error when polling times out or the torrent list query fails.
pub(in super::super) async fn wait_until_client_has_any_torrent(
    client: &QbittorrentClient,
    timeout: Duration,
    poll_interval: Duration,
    client_name: &str,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);

    loop {
        let torrent_count = client.torrent_count().await?;

        tracing::info!("{client_name} has {torrent_count} torrent(s)");

        if torrent_count >= 1 {
            tracing::info!("{client_name} has at least one torrent");
            return Ok(());
        }

        poller
            .retry_or_timeout(|| {
                format!("timed out waiting for {client_name} torrent presence: {client_name} has {torrent_count}")
            })
            .await?;
    }
}
