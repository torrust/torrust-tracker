use super::super::super::poller::Poller;
use super::super::super::qbittorrent::QbittorrentClient;
use super::super::super::types::{Deadline, InfoHash, PollInterval};

/// Waits until the torrent identified by `hash` reaches full completion.
///
/// Uses the `InfoHash` to look up the specific torrent rather than picking the
/// first entry in the list, making this step robust when the client holds
/// multiple torrents concurrently.
///
/// # Errors
///
/// Returns an error when polling times out or the torrent list query fails.
pub async fn wait_until_download_completes(
    client: &QbittorrentClient,
    hash: &InfoHash,
    timeout: Deadline,
    poll_interval: PollInterval,
) -> anyhow::Result<()> {
    let poller = Poller::new(timeout, poll_interval);
    let client_label = client.label();

    loop {
        if let Some(torrent) = client.torrent_by_hash(hash).await? {
            let progress_pct = torrent.progress.as_fraction() * 100.0;
            tracing::info!(
                client = client_label,
                torrent = %hash,
                progress = progress_pct,
                state = %torrent.state,
                "download progress"
            );

            if torrent.progress.is_complete() {
                tracing::info!(client = client_label, torrent = %hash, "download complete");
                return Ok(());
            }
        }

        poller
            .retry_or_timeout(|| format!("timed out waiting for torrent {hash} to complete"))
            .await?;
    }
}
