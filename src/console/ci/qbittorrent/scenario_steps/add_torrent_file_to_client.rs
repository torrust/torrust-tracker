use anyhow::Context;

use super::super::qbittorrent_client::QbittorrentClient;

/// Submits a `.torrent` file to a qBittorrent client.
///
/// This step only submits the torrent definition and save path. It does not guarantee that the
/// torrent has already appeared in the client list or reached a seeding/downloading state.
///
/// # Errors
///
/// Returns an error when the qBittorrent API call fails.
pub(in super::super) async fn add_torrent_file_to_client(
    client: &QbittorrentClient,
    torrent_file_name: &str,
    torrent_bytes: &[u8],
    save_path: &str,
) -> anyhow::Result<()> {
    client
        .add_torrent_file(torrent_file_name, torrent_bytes, save_path)
        .await
        .context("failed to add torrent file to qBittorrent client")
}
