use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use torrust_tracker_udp_core::services::banning::BanService;

use super::handler::handle_event;
use crate::CurrentClock;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: &Arc<RwLock<BanService>>,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let ban_service_clone = ban_service.clone();
    let repository_clone = repository.clone();

    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Starting UDP tracker server event listener (banning)");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, ban_service_clone, repository_clone).await;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "UDP tracker server event listener (banning) finished");
    })
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    ban_service: Arc<RwLock<BanService>>,
    repository: Arc<Repository>,
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                log_cancellation();
                break;
            }

            result = receiver.recv() => {
                if !handle_received_event(result, &ban_service, &repository).await {
                    break;
                }
            }
        }
    }
}

async fn handle_received_event(
    result: Result<crate::event::Event, RecvError>,
    ban_service: &Arc<RwLock<BanService>>,
    repository: &Repository,
) -> bool {
    match result {
        Ok(event) => handle_event(event, ban_service, repository, CurrentClock::now()).await,
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
    tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Received cancellation request, shutting down UDP tracker server event listener.");
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures::{FutureExt as _, future::BoxFuture};
    use tokio::sync::RwLock;
    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::receiver::RecvError;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_udp_core::event::ConnectionContext;
    use torrust_tracker_udp_core::services::banning::BanService;

    use super::dispatch_events;
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
            self.receives.fetch_add(1, Ordering::SeqCst);
            futures::future::ready(self.results.pop_front().expect("scripted receive result")).boxed()
        }
    }

    fn error_event(error: ErrorKind) -> Event {
        Event::UdpError {
            context: ConnectionContext::new(
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
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
            Arc::new(Repository::new()),
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
            Arc::new(Repository::new()),
        )
        .await;

        // Assert
        assert_eq!(ban_service.read().await.get_banned_ips_total(), 1);
    }

    #[tokio::test]
    async fn it_should_prioritize_cancellation_over_a_ready_event() {
        // Arrange
        let (scripted_receiver, _receives) =
            ScriptedReceiver::new([Ok(error_event(ErrorKind::ConnectionCookie("expired".into())))]);
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();
        let ban_service = ban_service();

        // Act
        dispatch_events(
            Box::new(scripted_receiver),
            cancellation_token,
            ban_service.clone(),
            Arc::new(Repository::new()),
        )
        .await;

        // Assert
        assert_eq!(ban_service.read().await.get_banned_ips_total(), 0);
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
            Arc::new(Repository::new()),
        )
        .await;

        // Assert
        assert_eq!(ban_service.read().await.get_banned_ips_total(), 0);
    }
}
