use super::super::qbittorrent_client::QbittorrentClient;
use super::add_torrent_file_to_client::add_torrent_file_to_client;

/// Adds a `.torrent` file to the leecher client.
///
/// This wraps the generic client step with an explicit leecher-oriented name for scenario narration.
///
/// # Errors
///
/// Returns an error when the qBittorrent API call fails.
pub(in super::super) async fn add_torrent_file_to_leecher(
    leecher: &QbittorrentClient,
    torrent_file_name: &str,
    torrent_bytes: &[u8],
    save_path: &str,
) -> anyhow::Result<()> {
    add_torrent_file_to_client(leecher, torrent_file_name, torrent_bytes, save_path).await
}
