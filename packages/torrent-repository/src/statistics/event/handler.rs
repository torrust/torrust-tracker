use std::sync::Arc;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;

/// # Panics
///
/// This function panics if the client IP address is not the same as the IP
/// version of the event.
pub async fn handle_event(_event: Event, stats_repository: &Arc<Repository>, _now: DurationSinceUnixEpoch) {
    /*match event {
        Event::TorrentAdded { .. } => {}
        Event::TorrentRemoved { .. } => {}
        Event::PeerAdded { .. } => {}
        Event::PeerRemoved { .. } => {}
    }*/

    tracing::debug!("metrics: {:?}", stats_repository.get_metrics().await);
}
