use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::drain::drain;
use torrust_tracker_events::log_drain_report;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_events::shutdown::Completion;
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
    drain_timeout: Duration,
) -> JoinHandle<()> {
    let repository_clone = repository.clone();
    tokio::spawn(async move {
        let _ = run_event_listener_unspawned(receiver, cancellation_token, repository_clone, metrics_policy, drain_timeout).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener finished");
    })
}

/// Runs the listener without spawning so a caller can retain task ownership.
///
/// After cancellation the listener keeps handling the events already queued for it until its
/// queue is empty or `drain_timeout` expires (see [`drain`]), logs what it did, and returns
/// [`Completion::Cancelled`].
pub async fn run_event_listener_unspawned(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    metrics_policy: BTreeMap<ConfigurationInstanceId, bool>,
    drain_timeout: Duration,
) -> Completion {
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener");
    dispatch_events(receiver, cancellation_token, stats_repository, metrics_policy, drain_timeout).await
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    metrics_policy: BTreeMap<ConfigurationInstanceId, bool>,
    drain_timeout: Duration,
) -> Completion {
    // issue: #2039
    // Only this aggregate metrics consumer filters disabled listeners. The
    // banning listener receives the same unfiltered objective event stream.
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                log_cancellation();

                drain_events(&mut receiver, drain_timeout, &stats_repository, &metrics_policy).await;

                return Completion::Cancelled;
            }

            result = receiver.recv() => {
                if !handle_received_event(result, &stats_repository, &metrics_policy).await {
                    return Completion::Completed;
                }
            }
        }
    }
}

/// Handles the events still queued for the listener after its cancellation, within the timeout,
/// and logs what the drain did. The metrics policy applies to a drained event as to any other.
async fn drain_events(
    receiver: &mut Receiver,
    drain_timeout: Duration,
    stats_repository: &Arc<Repository>,
    metrics_policy: &BTreeMap<ConfigurationInstanceId, bool>,
) {
    let report = drain(&mut **receiver, drain_timeout, |event| {
        handle_event_if_enabled(event, stats_repository, metrics_policy)
    })
    .await;

    log_drain_report!(UDP_TRACKER_LOG_TARGET, "udp_tracker_server_statistics", report);
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
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, draining UDP tracker server event listener.");
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
    use std::time::Duration;

    use futures::FutureExt as _;
    use futures::future::BoxFuture;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
    use torrust_tracker_events::receiver::{RecvError, TryRecvError};
    use torrust_tracker_events::shutdown::Completion;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;

    use super::dispatch_events;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::statistics::metrics::Metrics;
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
            // Lazy, like a real receiver: `select!` creates this future even when another branch wins,
            // and an eager pop would consume an event the drain must still see.
            async move {
                self.receives.fetch_add(1, Ordering::SeqCst);
                self.results.pop_front().expect("scripted receive result")
            }
            .boxed()
        }

        fn try_recv(&mut self) -> Result<Self::Event, TryRecvError> {
            match self.results.pop_front() {
                Some(Ok(event)) => Ok(event),
                Some(Err(RecvError::Closed)) => Err(TryRecvError::Closed),
                Some(Err(RecvError::Lagged(skipped))) => Err(TryRecvError::Lagged(skipped)),
                None => Err(TryRecvError::Empty),
            }
        }

        fn len(&self) -> usize {
            self.results.iter().filter(|result| result.is_ok()).count()
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
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        assert_eq!(repository.read_metrics(Metrics::udp4_requests_received_total).await, 1);
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
            DEFAULT_DRAIN_TIMEOUT,
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
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        assert_eq!(repository.read_metrics(Metrics::udp4_requests_received_total).await, 1);
    }

    #[tokio::test]
    async fn it_should_handle_a_queued_event_then_return_cancelled_when_cancellation_is_already_pending() {
        // Arrange
        // A request-received event from a metrics-enabled instance is queued for the listener, and
        // its token is cancelled before the listener takes the event.
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let (scripted_receiver, _receives) = ScriptedReceiver::new([Ok(request_received_event(enabled_id))]);
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        cancellation_token.cancel();
        let repository = Arc::new(Repository::new());

        // Act
        let completion = dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            repository.clone(),
            [(enabled_id, true)].into(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        // The queued request is counted, and the listener reports that it stopped because of the
        // cancellation.
        assert_eq!(
            repository.read_metrics(Metrics::udp4_requests_received_total).await,
            1,
            "`udp4_requests_received_total` counted after the queued event was drained"
        );
        assert_eq!(completion, Completion::Cancelled, "a drained listener must return Cancelled");
    }

    #[tokio::test]
    async fn it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired() {
        // Arrange
        // The same queued event and cancelled token, with a drain timeout of zero, so it has expired
        // before the first event could be admitted.
        let enabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let (scripted_receiver, _receives) = ScriptedReceiver::new([Ok(request_received_event(enabled_id))]);
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        cancellation_token.cancel();
        let repository = Arc::new(Repository::new());

        // Act
        let completion = dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            repository.clone(),
            [(enabled_id, true)].into(),
            Duration::ZERO,
        )
        .await;

        // Assert
        // Nothing was counted, because the timeout stops admission, and the listener still returns
        // Cancelled so the supervisor sees a cooperative stop.
        assert_eq!(
            repository.read_metrics(Metrics::udp4_requests_received_total).await,
            0,
            "`udp4_requests_received_total` stays at zero after a drain whose timeout had already expired"
        );
        assert_eq!(
            completion,
            Completion::Cancelled,
            "an expired drain must still return Cancelled"
        );
    }

    #[tokio::test]
    async fn it_should_apply_the_metrics_policy_to_a_drained_event() {
        // Arrange
        // A request-received event from a metrics-disabled instance is queued, and the token is
        // cancelled before the listener takes it.
        let disabled_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);
        let (scripted_receiver, _receives) = ScriptedReceiver::new([Ok(request_received_event(disabled_id))]);
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        cancellation_token.cancel();
        let repository = Arc::new(Repository::new());

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            repository.clone(),
            [(disabled_id, false)].into(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        // The drain reads the event but, like a live event, it is not counted: the policy decides, not
        // the drain.
        assert_eq!(
            repository.read_metrics(Metrics::udp4_requests_received_total).await,
            0,
            "`udp4_requests_received_total` stays at zero after draining an event from a metrics-disabled instance"
        );
    }
}
