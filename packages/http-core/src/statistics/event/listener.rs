use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_primitives::ConfigurationInstanceId;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, HTTP_TRACKER_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    run_event_listener_with_metrics_policy(receiver, cancellation_token, repository, BTreeMap::new())
}

#[must_use]
pub fn run_event_listener_with_metrics_policy(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
    metrics_policy: BTreeMap<ConfigurationInstanceId, bool>,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Starting HTTP tracker core event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, stats_repository, metrics_policy).await;

        tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "HTTP tracker core event listener finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    metrics_policy: BTreeMap<ConfigurationInstanceId, bool>,
) {
    // issue: #2039
    // Metrics policy is enforced here, at the aggregate-repository consumer,
    // rather than when the objective fact is produced.
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down HTTP tracker core event listener.");
                break;
            }

            result = receiver.recv() => {
                match result {
                    Ok(event) if metrics_policy.is_empty() || metrics_policy.get(&event_connection_id(&event)).copied().unwrap_or(false) => {
                        handle_event(event, &stats_repository, CurrentClock::now()).await;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        match e {
                            RecvError::Closed => {
                                tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Http tracker core statistics receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: HTTP_TRACKER_LOG_TARGET, "Http tracker core statistics receiver lagged by {} events.", n);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn event_connection_id(event: &crate::event::Event) -> ConfigurationInstanceId {
    match event {
        crate::event::Event::TcpAnnounce { connection, .. } | crate::event::Event::TcpScrape { connection } => {
            connection.configuration_instance_id()
        }
    }
}
