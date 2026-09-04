use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;
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
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core in-memory statistics event listener");

    tokio::spawn(async move {
        dispatch_in_memory_events(receiver, cancellation_token, stats_repository).await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core in-memory statistics event listener finished");
    })
}

#[must_use]
pub fn run_persistent_completed_statistics_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let db_downloads_metric_repository = db_downloads_metric_repository.clone();
    let stats_repository = repository.clone();
    tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Starting tracker core persistent completed statistics event listener");

    tokio::spawn(async move {
        dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            db_downloads_metric_repository,
            stats_repository,
        )
        .await;

        tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Tracker core persistent completed statistics event listener finished");
    })
}

async fn dispatch_in_memory_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, shutting down tracker core event listener.");
                break;
            }

            result = receiver.recv() => {
                if !handle_in_memory_receive_result(result, &stats_repository).await {
                    break;
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
) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: TRACKER_CORE_LOG_TARGET, "Received cancellation request, shutting down tracker core persistent completed statistics event listener.");
                break;
            }

            result = receiver.recv() => {
                if !handle_persistent_completed_statistics_receive_result(
                    result,
                    &db_downloads_metric_repository,
                    &stats_repository,
                ).await {
                    break;
                }
            }
        }
    }
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

    use tokio_util::sync::CancellationToken;
    use torrust_clock::clock;
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_tracker_events::receiver::RecvError;
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
            Box::pin(std::future::ready(self.results.pop_front().unwrap_or(Err(RecvError::Closed))))
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
        assert_eq!(downloads_repository.load_global_downloads().await.unwrap(), expected_value);
        assert_eq!(
            stats_repository.get_torrents_downloads_persisted_total().await,
            u64::from(expected_value.unwrap_or(0))
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
        dispatch_in_memory_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

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
        dispatch_in_memory_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_stop_when_the_in_memory_receiver_is_closed() {
        // Arrange
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([Err(RecvError::Closed)]));

        // Act
        dispatch_in_memory_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 0).await;
    }

    #[tokio::test]
    async fn it_should_prioritize_a_pre_cancelled_token_over_a_ready_in_memory_event() {
        // Arrange
        let stats_repository = Arc::new(Repository::default());
        let receiver = Box::new(ScriptedReceiver::new([Ok(peer_download_completed_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        dispatch_in_memory_events(receiver, cancellation_token, stats_repository.clone()).await;

        // Assert
        expect_in_memory_download_metrics_to_be(&stats_repository, 0).await;
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
        )
        .await;

        // Assert
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, None).await;
    }

    #[tokio::test]
    async fn it_should_prioritize_a_pre_cancelled_token_over_a_ready_persistent_event() {
        // Arrange
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
        )
        .await;

        // Assert
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, None).await;
    }
}
