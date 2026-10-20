use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::drain::drain;
use torrust_tracker_events::log_drain_report;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_events::shutdown::Completion;
use torrust_tracker_swarm_coordination_registry::event::receiver::Receiver;

use super::handler::{handle_in_memory_event, handle_persistent_completed_statistics_event};
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, TRACKER_CORE_LOG_TARGET};

#[must_use]
pub fn run_in_memory_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
    drain_timeout: Duration,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    tokio::spawn(async move {
        let _ = run_in_memory_event_listener_unspawned(receiver, cancellation_token, stats_repository, drain_timeout).await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core in-memory statistics event listener finished");
    })
}

#[must_use]
pub fn run_persistent_completed_statistics_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    repository: &Arc<Repository>,
    drain_timeout: Duration,
) -> JoinHandle<()> {
    let db_downloads_metric_repository = db_downloads_metric_repository.clone();
    let stats_repository = repository.clone();
    tokio::spawn(async move {
        let _ = run_persistent_completed_statistics_event_listener_unspawned(
            receiver,
            cancellation_token,
            db_downloads_metric_repository,
            stats_repository,
            drain_timeout,
        )
        .await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core persistent completed statistics event listener finished");
    })
}

/// Runs the in-memory listener without spawning so a caller can retain task ownership.
///
/// After cancellation the listener keeps handling the events already queued for it until its
/// queue is empty or `drain_timeout` expires (see [`drain`]), logs what it did, and returns
/// [`Completion::Cancelled`].
pub async fn run_in_memory_event_listener_unspawned(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core in-memory statistics event listener");
    dispatch_in_memory_events(receiver, cancellation_token, stats_repository, drain_timeout).await
}

/// Runs the persistent-statistics listener without spawning so a caller can retain task ownership.
///
/// After cancellation the listener keeps persisting the events already queued for it until its
/// queue is empty or `drain_timeout` expires (see [`drain`]), logs what it did, and returns
/// [`Completion::Cancelled`].
pub async fn run_persistent_completed_statistics_event_listener_unspawned(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core persistent completed statistics event listener");
    dispatch_persistent_completed_statistics_events(
        receiver,
        cancellation_token,
        db_downloads_metric_repository,
        stats_repository,
        drain_timeout,
    )
    .await
}

async fn dispatch_in_memory_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, draining tracker core in-memory statistics event listener.");

                drain_in_memory_events(&mut receiver, drain_timeout, &stats_repository).await;

                return Completion::Cancelled;
            }

            result = receiver.recv() => {
                if !handle_in_memory_receive_result(result, &stats_repository).await {
                    return Completion::Completed;
                }
            }
        }
    }
}

async fn dispatch_persistent_completed_statistics_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, draining tracker core persistent completed statistics event listener.");

                drain_persistent_completed_statistics_events(
                    &mut receiver,
                    drain_timeout,
                    &db_downloads_metric_repository,
                    &stats_repository,
                )
                .await;

                return Completion::Cancelled;
            }

            result = receiver.recv() => {
                if !handle_persistent_completed_statistics_receive_result(
                    result,
                    &db_downloads_metric_repository,
                    &stats_repository,
                ).await {
                    return Completion::Completed;
                }
            }
        }
    }
}

/// Handles the events still queued for the in-memory listener after its cancellation, within
/// the timeout, and logs what the drain did.
async fn drain_in_memory_events(receiver: &mut Receiver, drain_timeout: Duration, stats_repository: &Arc<Repository>) {
    let report = drain(&mut **receiver, drain_timeout, |event| {
        handle_in_memory_event(event, stats_repository, CurrentClock::now())
    })
    .await;

    log_drain_report!(TRACKER_CORE_LOG_TARGET, "tracker_core_in_memory_statistics", report);
}

/// Persists the events still queued for the listener after its cancellation, within the timeout,
/// and logs what the drain did.
async fn drain_persistent_completed_statistics_events(
    receiver: &mut Receiver,
    drain_timeout: Duration,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: &Arc<Repository>,
) {
    let report = drain(&mut **receiver, drain_timeout, |event| {
        handle_persistent_completed_statistics_event(event, db_downloads_metric_repository, stats_repository, CurrentClock::now())
    })
    .await;

    log_drain_report!(
        TRACKER_CORE_LOG_TARGET,
        "tracker_core_persistent_completed_statistics",
        report
    );
}

async fn handle_in_memory_receive_result(
    result: Result<torrust_tracker_swarm_coordination_registry::event::Event, RecvError>,
    stats_repository: &Arc<Repository>,
) -> bool {
    match result {
        Ok(event) => {
            handle_in_memory_event(event, stats_repository, CurrentClock::now()).await;
            true
        }
        Err(error) => handle_receive_error(&error),
    }
}

async fn handle_persistent_completed_statistics_receive_result(
    result: Result<torrust_tracker_swarm_coordination_registry::event::Event, RecvError>,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: &Arc<Repository>,
) -> bool {
    match result {
        Ok(event) => {
            handle_persistent_completed_statistics_event(
                event,
                db_downloads_metric_repository,
                stats_repository,
                CurrentClock::now(),
            )
            .await;
            true
        }
        Err(error) => handle_receive_error(&error),
    }
}

fn handle_receive_error(error: &RecvError) -> bool {
    match error {
        RecvError::Closed => {
            tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core event receiver closed");
            false
        }
        RecvError::Lagged(number_of_events) => {
            tracing::warn!(target: TRACKER_CORE_LOG_TARGET, "Tracker core event receiver lagged by {} events", number_of_events);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_clock::clock;
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
    use torrust_tracker_events::receiver::RecvError;
    use torrust_tracker_events::shutdown::Completion;
    use torrust_tracker_swarm_coordination_registry::event::Event;

    use super::{dispatch_in_memory_events, dispatch_persistent_completed_statistics_events};
    use crate::databases::setup::initialize_database;
    use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use crate::statistics::repository::Repository;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_peer};

    struct ScriptedReceiver {
        results: VecDeque<Result<Event, RecvError>>,
    }

    impl ScriptedReceiver {
        fn new(results: impl IntoIterator<Item = Result<Event, RecvError>>) -> Self {
            Self {
                results: results.into_iter().collect(),
            }
        }
    }

    impl torrust_tracker_events::receiver::Receiver for ScriptedReceiver {
        type Event = Event;

        fn recv(&mut self) -> Pin<Box<dyn Future<Output = Result<Self::Event, RecvError>> + Send + '_>> {
            // Lazy, like a real receiver: `select!` creates this future even when another branch wins,
            // and an eager pop would consume an event the drain must still see.
            Box::pin(async move { self.results.pop_front().unwrap_or(Err(RecvError::Closed)) })
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

    fn peer_download_completed_event() -> Event {
        Event::PeerDownloadCompleted {
            info_hash: sample_info_hash(),
            peer: sample_peer(),
        }
    }

    async fn database_downloads_repository() -> Arc<DatabaseDownloadsMetricRepository> {
        let configuration = ephemeral_configuration();
        let stores = initialize_database(&configuration).await;

        Arc::new(DatabaseDownloadsMetricRepository::new(&stores.torrent_metrics_store))
    }

    async fn expect_in_memory_download_metrics_to_be(stats_repository: &Repository, expected_value: u64) {
        assert_eq!(stats_repository.get_torrents_downloads_total().await, expected_value);
        assert_eq!(
            stats_repository.get_torrents_downloads_in_session_total().await,
            expected_value
        );
    }

    async fn expect_persisted_download_metrics_to_be(
        downloads_repository: &DatabaseDownloadsMetricRepository,
        stats_repository: &Repository,
        expected_value: Option<u32>,
    ) {
        assert_eq!(
            downloads_repository.load_global_downloads().await.unwrap(),
            expected_value,
            "global downloads stored in the database"
        );
        assert_eq!(
            stats_repository.get_torrents_downloads_persisted_total().await,
            u64::from(expected_value.unwrap_or(0)),
            "persisted downloads counter of the statistics repository"
        );
    }

    #[tokio::test]
    async fn it_should_handle_an_event_then_stop_when_the_in_memory_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([
            Ok(peer_download_completed_event()),
            Err(RecvError::Closed),
        ]));

        // Act
        dispatch_in_memory_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_continue_after_lag_then_handle_an_event_then_stop_when_the_in_memory_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([
            Err(RecvError::Lagged(2)),
            Ok(peer_download_completed_event()),
            Err(RecvError::Closed),
        ]));

        // Act
        dispatch_in_memory_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_stop_when_the_in_memory_receiver_is_closed() {
        // Arrange
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([Err(RecvError::Closed)]));

        // Act
        dispatch_in_memory_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 0).await;
    }

    #[tokio::test]
    async fn it_should_handle_a_queued_event_then_return_cancelled_when_cancellation_is_already_pending() {
        // Arrange
        // A completed download is queued for the in-memory listener, and its token is cancelled
        // before the listener takes the event.
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([Ok(peer_download_completed_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion =
            dispatch_in_memory_events(receiver, cancellation_token, stats_repository.clone(), DEFAULT_DRAIN_TIMEOUT).await;

        // Assert
        // The queued download is counted, and the listener reports that it stopped because of the
        // cancellation.
        expect_in_memory_download_metrics_to_be(&stats_repository, 1).await;
        assert_eq!(completion, Completion::Cancelled, "a drained listener must return Cancelled");
    }

    #[tokio::test]
    async fn it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired() {
        // Arrange
        // The same queued download and cancelled token, with a drain timeout of zero, so it has
        // expired before the first event could be admitted.
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([Ok(peer_download_completed_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion = dispatch_in_memory_events(receiver, cancellation_token, stats_repository.clone(), Duration::ZERO).await;

        // Assert
        // Nothing was counted, because the timeout stops admission, and the listener still returns
        // Cancelled so the supervisor sees a cooperative stop.
        expect_in_memory_download_metrics_to_be(&stats_repository, 0).await;
        assert_eq!(
            completion,
            Completion::Cancelled,
            "an expired drain must still return Cancelled"
        );
    }

    #[tokio::test]
    async fn it_should_handle_an_event_then_stop_when_the_persistent_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([
            Ok(peer_download_completed_event()),
            Err(RecvError::Closed),
        ]));

        // Act
        dispatch_persistent_completed_statistics_events(
            receiver,
            CancellationToken::new(),
            downloads_repository.clone(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, Some(1)).await;
    }

    #[tokio::test]
    async fn it_should_continue_after_lag_then_handle_an_event_then_stop_when_the_persistent_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([
            Err(RecvError::Lagged(2)),
            Ok(peer_download_completed_event()),
            Err(RecvError::Closed),
        ]));

        // Act
        dispatch_persistent_completed_statistics_events(
            receiver,
            CancellationToken::new(),
            downloads_repository.clone(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, Some(1)).await;
    }

    #[tokio::test]
    async fn it_should_stop_when_the_persistent_receiver_is_closed() {
        // Arrange
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([Err(RecvError::Closed)]));

        // Act
        dispatch_persistent_completed_statistics_events(
            receiver,
            CancellationToken::new(),
            downloads_repository.clone(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, None).await;
    }

    #[tokio::test]
    async fn it_should_persist_a_queued_completed_download_when_cancellation_is_already_pending() {
        // Arrange
        // At shutdown a completed download is queued for the persistent listener, and the
        // listener's token is cancelled before the listener takes the event.
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([Ok(peer_download_completed_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            downloads_repository.clone(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        // The queued download reaches the database, and the persisted counter shows it.
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, Some(1)).await;
    }

    #[tokio::test]
    async fn it_should_persist_every_queued_event_then_return_cancelled_when_cancellation_is_already_pending() {
        // Arrange
        // Two completed downloads are queued for the persistent listener, and its token is
        // cancelled before the listener takes them.
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([
            Ok(peer_download_completed_event()),
            Ok(peer_download_completed_event()),
        ]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion = dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            downloads_repository.clone(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        // Both downloads were persisted, and the listener reports that it stopped because of the
        // cancellation, not because its receiver closed.
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, Some(2)).await;
        assert_eq!(completion, Completion::Cancelled, "a drained listener must return Cancelled");
    }

    #[tokio::test]
    async fn it_should_leave_the_queued_events_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired() {
        // Arrange
        // Two completed downloads are queued, the token is cancelled, and the drain timeout is zero,
        // so it has expired before the first event could be admitted.
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([
            Ok(peer_download_completed_event()),
            Ok(peer_download_completed_event()),
        ]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion = dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            downloads_repository.clone(),
            stats_repository.clone(),
            Duration::ZERO,
        )
        .await;

        // Assert
        // Nothing was persisted, because the timeout stops admission, and the listener still
        // returns Cancelled so the supervisor sees a cooperative stop.
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, None).await;
        assert_eq!(
            completion,
            Completion::Cancelled,
            "an expired drain must still return Cancelled"
        );
    }
}
