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
                log_cancellation();
                break;
            }

            result = receiver.recv() => {
                if !handle_received_event(result, &stats_repository, &metrics_policy).await {
                    break;
                }
            }
        }
    }
}

async fn handle_received_event(
    result: Result<crate::event::Event, RecvError>,
    stats_repository: &Repository,
    metrics_policy: &BTreeMap<ConfigurationInstanceId, bool>,
) -> bool {
    match result {
        Ok(event) => handle_event_if_enabled(event, stats_repository, metrics_policy).await,
        Err(RecvError::Closed) => {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver closed.");
            return false;
        }
        Err(RecvError::Lagged(events)) => {
            tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server statistics receiver lagged by {} events.", events);
        }
    }

    true
}

async fn handle_event_if_enabled(
    event: crate::event::Event,
    stats_repository: &Repository,
    metrics_policy: &BTreeMap<ConfigurationInstanceId, bool>,
) {
    let configuration_instance_id = event_connection_id(&event);

    if metrics_policy.get(&configuration_instance_id).copied() == Some(true) {
        handle_event(event, stats_repository, CurrentClock::now()).await;
    } else {
        tracing::warn!(
            target: UDP_TRACKER_LOG_TARGET,
            ?configuration_instance_id,
            "Ignoring UDP server event from an unknown or metrics-disabled listener"
        );
    }
}

fn log_cancellation() {
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down UDP tracker server event listener.");
}

const fn event_connection_id(event: &crate::event::Event) -> ConfigurationInstanceId {
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
    use std::collections::{BTreeMap, VecDeque};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures::FutureExt as _;
    use futures::future::BoxFuture;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::receiver::RecvError;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::dispatch_events;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::statistics::repository::Repository;

    struct ScriptedReceiver {
        results: VecDeque<Result<Event, RecvError>>,
        receives: Arc<AtomicUsize>,
    }

    impl ScriptedReceiver {
        fn new(results: impl IntoIterator<Item = Result<Event, RecvError>>) -> (Self, Arc<AtomicUsize>) {
            let receives = Arc::new(AtomicUsize::new(0));
            (
                Self {
                    results: results.into_iter().collect(),
                    receives: receives.clone(),
                },
                receives,
            )
        }
    }

    impl torrust_tracker_events::receiver::Receiver for ScriptedReceiver {
        type Event = Event;

        fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>> {
            self.receives.fetch_add(1, Ordering::SeqCst);
            futures::future::ready(self.results.pop_front().expect("scripted receive result")).boxed()
        }
    }

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
        let (scripted_receiver, _receives) = ScriptedReceiver::new([
            Ok(request_received_event(enabled_id)),
            Ok(request_received_event(disabled_id)),
            Ok(request_received_event(unknown_id)),
            Err(RecvError::Closed),
        ]);
        let receiver: Receiver = Box::new(scripted_receiver);
        let repository = Arc::new(Repository::new());

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

    #[tokio::test]
    async fn it_should_stop_when_the_receiver_is_closed() {
        // Arrange
        let (scripted_receiver, receives) = ScriptedReceiver::new([Err(RecvError::Closed)]);

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            tokio_util::sync::CancellationToken::new(),
            Arc::new(Repository::new()),
            BTreeMap::new(),
        )
        .await;

        // Assert
        assert_eq!(receives.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn it_should_process_enabled_events_after_lagging() {
        // Arrange
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let (scripted_receiver, _receives) = ScriptedReceiver::new([
            Err(RecvError::Lagged(2)),
            Ok(request_received_event(enabled_id)),
            Err(RecvError::Closed),
        ]);
        let repository = Arc::new(Repository::new());

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            tokio_util::sync::CancellationToken::new(),
            repository.clone(),
            [(enabled_id, true)].into(),
        )
        .await;

        // Assert
        assert_eq!(repository.get_stats().await.udp4_requests_received_total(), 1);
    }

    #[tokio::test]
    async fn it_should_prioritize_cancellation_over_a_ready_event() {
        // Arrange
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let (scripted_receiver, _receives) = ScriptedReceiver::new([Ok(request_received_event(enabled_id))]);
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        cancellation_token.cancel();
        let repository = Arc::new(Repository::new());

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            repository.clone(),
            [(enabled_id, true)].into(),
        )
        .await;

        // Assert
        assert_eq!(repository.get_stats().await.udp4_requests_received_total(), 0);
    }
}
