use super::super::super::poller::Poller;
use super::super::super::qbittorrent::QbittorrentClient;
use super::super::super::types::{Deadline, InfoHash, PollInterval};

/// Ensures the torrent identified by `hash` is absent from the client's list.
///
/// If the torrent is already present it is deleted (files are kept on disk).
/// The function then polls until the client confirms it is gone, giving the
/// scenario a clean, deterministic starting state regardless of whether a
/// previous run left the torrent behind.
///
/// # Errors
///
/// Returns an error when the deletion request or the absence-polling times out
/// or fails.
pub async fn ensure_torrent_is_absent(
    client: &QbittorrentClient,
    hash: &InfoHash,
    timeout: Deadline,
    poll_interval: PollInterval,
    client_name: &str,
) -> anyhow::Result<()> {
    if client.has_torrent_with_hash(hash).await? {
        tracing::info!("{client_name}: torrent {hash} already present — deleting to start from a clean state");
        client.delete_torrent(hash).await?;
    }

    let poller = Poller::new(timeout, poll_interval);

    loop {
        if !client.has_torrent_with_hash(hash).await? {
            tracing::info!("{client_name}: torrent {hash} is absent");
            return Ok(());
        }

        tracing::info!("{client_name}: waiting for torrent {hash} to be removed");

        poller
            .retry_or_timeout(|| format!("timed out waiting for {client_name} to remove torrent {hash}"))
            .await?;
    }
}
