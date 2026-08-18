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
) -> anyhow::Result<()> {
    let client_label = client.label();

    delete_torrent_if_present(client, hash, client_label).await?;

    wait_until_torrent_is_absent(client, hash, timeout, poll_interval, client_label).await
}

async fn delete_torrent_if_present(client: &QbittorrentClient, hash: &InfoHash, client_label: &str) -> anyhow::Result<()> {
    if !client.has_torrent_with_hash(hash).await? {
        return Ok(());
    }

    tracing::info!(client = client_label, torrent = %hash, "torrent already present, deleting for clean start");
    client.delete_torrent(hash).await
}

async fn wait_until_torrent_is_absent(
    client: &QbittorrentClient,
    hash: &InfoHash,
    timeout: Deadline,
    poll_interval: PollInterval,
    client_label: &str,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);

    loop {
        if !client.has_torrent_with_hash(hash).await? {
            tracing::info!(client = client_label, torrent = %hash, "torrent is absent");
            return Ok(());
        }

        tracing::info!(client = client_label, torrent = %hash, "waiting for torrent to be removed");

        poller
            .retry_or_timeout(|| format!("timed out waiting for {client_label} to remove torrent {hash}"))
            .await?;
    }
}
