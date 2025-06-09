mod error;
mod request_aborted;
mod request_accepted;
mod request_banned;
mod request_received;
mod response_sent;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;

pub async fn handle_event(event: Event, stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    match event {
        Event::UdpRequestAborted { context } => {
            request_aborted::handle_event(context, stats_repository, now).await;
        }
        Event::UdpRequestBanned { context } => {
            request_banned::handle_event(context, stats_repository, now).await;
        }
        Event::UdpRequestReceived { context } => {
            request_received::handle_event(context, stats_repository, now).await;
        }
        Event::UdpRequestAccepted { context, kind } => {
            request_accepted::handle_event(context, kind, stats_repository, now).await;
        }
        Event::UdpResponseSent {
            context,
            kind,
            req_processing_time,
        } => {
            response_sent::handle_event(context, kind, req_processing_time, stats_repository, now).await;
        }
        Event::UdpError { context, kind, error } => {
            error::handle_event(context, kind, error, stats_repository, now).await;
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}
