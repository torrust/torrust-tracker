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
                    Ok(event) if metrics_policy.get(&event_connection_id(&event)).copied().unwrap_or(false) => {
                        handle_event(event, &stats_repository, CurrentClock::now()).await;
                    }
                    Ok(event) => {
                        tracing::warn!(
                            target: HTTP_TRACKER_LOG_TARGET,
                            configuration_instance_id = ?event_connection_id(&event),
                            "Ignoring HTTP tracker event from an unknown or metrics-disabled listener"
                        );
                    }
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::broadcaster::Broadcaster;
    use torrust_tracker_events::sender::Sender as _;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use super::dispatch_events;
    use crate::event::receiver::Receiver;
    use crate::event::{ConnectionContext, Event};
    use crate::statistics::repository::Repository;

    fn scrape_event(configuration_instance_id: ConfigurationInstanceId) -> Event {
        Event::TcpScrape {
            connection: ConnectionContext::new(
                configuration_instance_id,
                RemoteClientAddr::new(ResolvedIp::FromSocketAddr(IpAddr::V4(Ipv4Addr::LOCALHOST)), Some(8080)),
                ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap(),
            ),
        }
    }

    #[tokio::test]
    async fn it_should_update_metrics_only_for_an_enabled_configuration_instance() {
        // Arrange
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let disabled_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 1);
        let unknown_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 2);
        let broadcaster = Broadcaster::default();
        let receiver: Receiver = Box::new(broadcaster.subscribe());
        let repository = Arc::new(Repository::new());

        for configuration_instance_id in [enabled_id, disabled_id, unknown_id] {
            let _unused = broadcaster
                .send(scrape_event(configuration_instance_id))
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
        assert_eq!(repository.get_stats().await.tcp4_scrapes_handled(), 1);
    }
}
