use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::drain::drain;
use torrust_tracker_events::log_drain_report;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_events::shutdown::Completion;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::services::banning::BanService;

use super::handler::handle_event;
use crate::CurrentClock;
use crate::banning::PublicationOrder;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: &Arc<RwLock<BanService>>,
    publication_order: &Arc<PublicationOrder>,
    repository: &Arc<Repository>,
    drain_timeout: Duration,
) -> JoinHandle<()> {
    let ban_service_clone = ban_service.clone();
    let publication_order_clone = publication_order.clone();
    let repository_clone = repository.clone();
    tokio::spawn(async move {
        let _ = run_event_listener_unspawned(
            receiver,
            cancellation_token,
            ban_service_clone,
            publication_order_clone,
            repository_clone,
            drain_timeout,
        )
        .await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener (banning) finished");
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
    ban_service: Arc<RwLock<BanService>>,
    publication_order: Arc<PublicationOrder>,
    repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener (banning)");
    dispatch_events(
        receiver,
        cancellation_token,
        ban_service,
        publication_order,
        repository,
        drain_timeout,
    )
    .await
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: Arc<RwLock<BanService>>,
    publication_order: Arc<PublicationOrder>,
    repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                log_cancellation();

                drain_events(&mut receiver, drain_timeout, &ban_service, &publication_order, &repository).await;

                return Completion::Cancelled;
            }

            result = receiver.recv() => {
                if !handle_received_event(result, &ban_service, &publication_order, &repository).await {
                    return Completion::Completed;
                }
            }
        }
    }
}

/// Handles the events still queued for the listener after its cancellation, within the timeout,
/// and logs what the drain did.
async fn drain_events(
    receiver: &mut Receiver,
    drain_timeout: Duration,
    ban_service: &Arc<RwLock<BanService>>,
    publication_order: &Arc<PublicationOrder>,
    repository: &Arc<Repository>,
) {
    let report = drain(&mut **receiver, drain_timeout, |event| {
        handle_event(event, ban_service, publication_order, repository, CurrentClock::now())
    })
    .await;

    log_drain_report!(UDP_TRACKER_LOG_TARGET, "udp_tracker_server_banning", report);
}

async fn handle_received_event(
    result: Result<crate::event::Event, RecvError>,
    ban_service: &Arc<RwLock<BanService>>,
    publication_order: &PublicationOrder,
    repository: &Repository,
) -> bool {
    match result {
        Ok(event) => handle_event(event, ban_service, publication_order, repository, CurrentClock::now()).await,
        Err(RecvError::Closed) => {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server receiver  (banning) closed.");
            return false;
        }
        Err(RecvError::Lagged(events)) => {
            tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Udp tracker server receiver (banning) lagged by {} events.", events);
        }
    }

    true
}

fn log_cancellation() {
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, draining UDP tracker server event listener (banning).");
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use futures::FutureExt as _;
    use futures::future::BoxFuture;
    use tokio::sync::RwLock;
    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
    use torrust_tracker_events::receiver::RecvError;
    use torrust_tracker_events::shutdown::Completion;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::dispatch_events;
    use crate::banning::PublicationOrder;
    use crate::event::{ErrorKind, Event};
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

        fn try_recv(&mut self) -> Result<Self::Event, torrust_tracker_events::receiver::TryRecvError> {
            use torrust_tracker_events::receiver::TryRecvError;

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

    const EVENT_CLIENT_IP: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    fn error_event(error: ErrorKind) -> Event {
        Event::UdpError {
            context: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                SocketAddr::new(EVENT_CLIENT_IP, 8080),
                ServiceBinding::new(Protocol::UDP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6969)).unwrap(),
            ),
            kind: None,
            error,
        }
    }

    fn ban_service() -> Arc<RwLock<BanService>> {
        Arc::new(RwLock::new(BanService::new(1)))
    }

    #[tokio::test]
    async fn it_should_stop_when_the_receiver_is_closed() {
        // Arrange
        let (scripted_receiver, receives) = ScriptedReceiver::new([Err(RecvError::Closed)]);

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            CancellationToken::new(),
            ban_service(),
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        assert_eq!(receives.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn it_should_process_connection_cookie_errors_after_lagging() {
        // Arrange
        let (scripted_receiver, _receives) = ScriptedReceiver::new([
            Err(RecvError::Lagged(2)),
            Ok(error_event(ErrorKind::ConnectionCookie("expired".into()))),
            Err(RecvError::Closed),
        ]);
        let ban_service = ban_service();

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            CancellationToken::new(),
            ban_service.clone(),
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        assert_eq!(
            ban_service.read().await.get_count(&EVENT_CLIENT_IP),
            Some(1),
            "connection ID errors recorded for {EVENT_CLIENT_IP} after the listener lagged"
        );
    }

    #[tokio::test]
    async fn it_should_handle_a_queued_event_then_return_cancelled_when_cancellation_is_already_pending() {
        // Arrange
        // A connection-cookie error is queued for the banning listener, and its token is cancelled
        // before the listener takes the event.
        let (scripted_receiver, _receives) =
            ScriptedReceiver::new([Ok(error_event(ErrorKind::ConnectionCookie("expired".into())))]);
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();
        let ban_service = ban_service();

        // Act
        let completion = dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            ban_service.clone(),
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        // The queued error is counted against the client, and the listener reports that it stopped
        // because of the cancellation.
        assert_eq!(
            ban_service.read().await.get_count(&EVENT_CLIENT_IP),
            Some(1),
            "connection ID errors recorded for {EVENT_CLIENT_IP}"
        );
        assert_eq!(completion, Completion::Cancelled, "a drained listener must return Cancelled");
    }

    #[tokio::test]
    async fn it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired() {
        // Arrange
        // The same queued error and cancelled token, with a drain timeout of zero, so it has expired
        // before the first event could be admitted.
        let (scripted_receiver, _receives) =
            ScriptedReceiver::new([Ok(error_event(ErrorKind::ConnectionCookie("expired".into())))]);
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();
        let ban_service = ban_service();

        // Act
        let completion = dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            ban_service.clone(),
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            Duration::ZERO,
        )
        .await;

        // Assert
        // Nothing was counted, because the timeout stops admission, and the listener still returns
        // Cancelled so the supervisor sees a cooperative stop.
        assert_eq!(
            ban_service.read().await.get_count(&EVENT_CLIENT_IP),
            None,
            "connection ID errors recorded for {EVENT_CLIENT_IP}"
        );
        assert_eq!(
            completion,
            Completion::Cancelled,
            "an expired drain must still return Cancelled"
        );
    }

    #[tokio::test]
    async fn it_should_ignore_non_connection_cookie_errors() {
        // Arrange
        let (scripted_receiver, _receives) = ScriptedReceiver::new([
            Ok(error_event(ErrorKind::BadRequest("invalid request".into()))),
            Err(RecvError::Closed),
        ]);
        let ban_service = ban_service();

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            CancellationToken::new(),
            ban_service.clone(),
            Arc::new(PublicationOrder::default()),
            Arc::new(Repository::new()),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        assert_eq!(
            ban_service.read().await.get_count(&EVENT_CLIENT_IP),
            None,
            "connection ID errors recorded for {EVENT_CLIENT_IP}"
        );
    }
}
