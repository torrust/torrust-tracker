use super::super::super::poller::Poller;
use super::super::super::qbittorrent::QbittorrentClient;
use super::super::super::types::{Deadline, InfoHash, PollInterval};

/// Waits until the client reports the torrent identified by `hash` in its list.
///
/// This is the presence/registration barrier for the asynchronous add-torrent
/// flow. It does not guarantee seeding, downloading, or completion state.
///
/// Unlike a generic "has any torrent" check, this is robust when the client
/// already holds other torrents: it returns only once the specific torrent
/// uploaded by this scenario is confirmed present.
///
/// # Errors
///
/// Returns an error when polling times out or the torrent list query fails.
pub async fn wait_until_torrent_appears_in_client(
    client: &QbittorrentClient,
    hash: &InfoHash,
    timeout: Deadline,
    poll_interval: PollInterval,
) -> anyhow::Result<()> {
    let client_label = client.label();
    let poller = Poller::new(timeout, poll_interval);

    loop {
        if client.has_torrent_with_hash(hash).await? {
            tracing::info!(client = client_label, torrent = %hash, "torrent has appeared in client list");
            return Ok(());
        }

        let torrent_count = client.torrent_count().await?;
        tracing::info!(client = client_label, torrent = %hash, torrent_count = torrent_count, "waiting for torrent to appear");

        poller
            .retry_or_timeout(|| format!("timed out waiting for {client_label} to register torrent {hash}"))
            .await?;
    }
}
