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
                    Ok(event) if metrics_policy.get(&event_connection_id(&event)).copied().unwrap_or(false) => {
                        handle_event(event, &stats_repository, CurrentClock::now()).await;
                    }
                    Ok(event) => {
                        tracing::warn!(
                            target: UDP_TRACKER_LOG_TARGET,
                            configuration_instance_id = ?event_connection_id(&event),
                            "Ignoring UDP server event from an unknown or metrics-disabled listener"
                        );
                    }
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::broadcaster::Broadcaster;
    use torrust_tracker_events::sender::Sender as _;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::dispatch_events;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::statistics::repository::Repository;

    fn request_received_event(configuration_instance_id: ConfigurationInstanceId) -> Event {
        Event::UdpRequestReceived {
            context: ConnectionContext::new(
                configuration_instance_id,
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
                ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
            ),
        }
    }

    #[tokio::test]
    async fn it_should_update_metrics_only_for_an_enabled_configuration_instance() {
        // Arrange
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let disabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);
        let unknown_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 2);
        let broadcaster = Broadcaster::default();
        let receiver: Receiver = Box::new(broadcaster.subscribe());
        let repository = Arc::new(Repository::new());

        for configuration_instance_id in [enabled_id, disabled_id, unknown_id] {
            let _unused = broadcaster
                .send(request_received_event(configuration_instance_id))
                .await
                .unwrap()
                .unwrap();
        }
        drop(broadcaster);

        // Act
        dispatch_events(
            receiver,
            tokio_util::sync::CancellationToken::new(),
            repository.clone(),
            [(enabled_id, true), (disabled_id, false)].into(),
        )
        .await;

        // Assert
        assert_eq!(repository.get_stats().await.udp4_requests_received_total(), 1);
    }
}
