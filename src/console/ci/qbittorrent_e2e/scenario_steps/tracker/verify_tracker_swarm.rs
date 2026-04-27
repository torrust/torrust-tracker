use anyhow::Context;
use torrust_axum_rest_tracker_api_server::v1::context::torrent::resources::torrent::Torrent;

use super::super::super::tracker::TrackerApiClient;
use super::super::super::types::InfoHash;

/// Queries the tracker REST API and asserts that the torrent shows at least one
/// seeder and at least one completed transfer.
///
/// This confirms that:
/// - the seeder announced itself to the tracker (`seeders >= 1`)
/// - the leecher sent a `completed` event after finishing the download (`completed >= 1`)
///
/// # Errors
///
/// Returns an error if the API request fails or either assertion does not hold.
pub async fn verify_tracker_swarm(client: &TrackerApiClient, hash: &InfoHash) -> anyhow::Result<()> {
    let torrent: Torrent = client
        .get_torrent(hash)
        .await
        .with_context(|| format!("failed to query tracker swarm for torrent {hash}"))?;

    tracing::info!(
        "Tracker swarm for {hash}: seeders={}, completed={}, leechers={}",
        torrent.seeders,
        torrent.completed,
        torrent.leechers
    );

    anyhow::ensure!(
        torrent.seeders >= 1,
        "expected at least 1 seeder in tracker for torrent {hash}, got {} \
         — seeder did not announce to the tracker",
        torrent.seeders
    );

    anyhow::ensure!(
        torrent.completed >= 1,
        "expected at least 1 completed transfer in tracker for torrent {hash}, got {} \
         — leecher did not send a completed event",
        torrent.completed
    );

    tracing::info!("Tracker swarm verification passed for {hash}");

    Ok(())
}
