use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;

use super::handler::handle_event;
use crate::CurrentClock;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;

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
    let repository_clone = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, repository_clone, metrics_policy).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    metrics_policy: BTreeMap<ConfigurationInstanceId, bool>,
) {
    // issue: #2039
    // Only this aggregate metrics consumer filters disabled listeners. The
    // banning listener receives the same unfiltered objective event stream.
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down UDP tracker server event listener.");
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
                                tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver closed.");
                                break;
                            }
                            RecvError::Lagged(n) => {
                                tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver lagged by {} events.", n);
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
        crate::event::Event::UdpRequestReceived { context }
        | crate::event::Event::UdpRequestDiscarded { context }
        | crate::event::Event::UdpRequestAborted { context }
        | crate::event::Event::UdpRequestBanned { context }
        | crate::event::Event::UdpRequestAccepted { context, .. }
        | crate::event::Event::UdpResponseSent { context, .. }
        | crate::event::Event::UdpError { context, .. } => context.configuration_instance_id(),
    }
}
