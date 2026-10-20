use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::drain::drain;
use torrust_tracker_events::log_drain_report;
use torrust_tracker_events::receiver::RecvError;
use torrust_tracker_events::shutdown::Completion;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, SWARM_COORDINATION_REGISTRY_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
    drain_timeout: Duration,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();
    tokio::spawn(async move {
        let _ = run_event_listener_unspawned(receiver, cancellation_token, stats_repository, drain_timeout).await;

        tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry listener finished");
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
    drain_timeout: Duration,
) -> Completion {
    tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Starting swarm coordination registry event listener");
    dispatch_events(receiver, cancellation_token, stats_repository, drain_timeout).await
}

async fn dispatch_events(
    mut receiver: Receiver,
    cancellation_token: CancellationToken,
    stats_repository: Arc<Repository>,
    drain_timeout: Duration,
) -> Completion {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Received cancellation request, draining swarm coordination registry event listener.");

                drain_events(&mut receiver, drain_timeout, &stats_repository).await;

                return Completion::Cancelled;
            }

            result = receiver.recv() => {
                if !handle_receive_result(result, &stats_repository).await {
                    return Completion::Completed;
                }
            }
        }
    }
}

/// Handles the events still queued for the listener after its cancellation, within the timeout,
/// and logs what the drain did.
async fn drain_events(receiver: &mut Receiver, drain_timeout: Duration, stats_repository: &Arc<Repository>) {
    let report = drain(&mut **receiver, drain_timeout, |event| {
        handle_event(event, stats_repository, CurrentClock::now())
    })
    .await;

    log_drain_report!(
        SWARM_COORDINATION_REGISTRY_LOG_TARGET,
        "swarm_coordination_registry_statistics",
        report
    );
}

async fn handle_receive_result(result: Result<crate::event::Event, RecvError>, stats_repository: &Arc<Repository>) -> bool {
    match result {
        Ok(event) => {
            handle_event(event, stats_repository, CurrentClock::now()).await;
            true
        }
        Err(error) => handle_receive_error(&error),
    }
}

fn handle_receive_error(error: &RecvError) -> bool {
    match error {
        RecvError::Closed => {
            tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry event receiver closed.");
            false
        }
        RecvError::Lagged(number_of_events) => {
            tracing::warn!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry event receiver lagged by {} events.", number_of_events);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::time::Duration;

    use futures::future::BoxFuture;
    use tokio_util::sync::CancellationToken;
    use torrust_clock::clock;
    use torrust_clock::clock::stopped::Stopped;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;
    use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
    use torrust_tracker_events::receiver::RecvError;
    use torrust_tracker_events::shutdown::Completion;

    use super::dispatch_events;
    use crate::event::Event;
    use crate::event::receiver::Receiver;
    use crate::statistics::SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL;
    use crate::statistics::repository::Repository;
    use crate::tests::{sample_info_hash, sample_peer};

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

        fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>> {
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

    fn peer_added_event() -> Event {
        Event::PeerAdded {
            info_hash: sample_info_hash(),
            peer: sample_peer(),
        }
    }

    async fn expect_peer_added_metric_to_be(stats_repository: &Repository, expected_value: u64) {
        let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL);
        let label_set = LabelSet::from((
            torrust_metrics::label_name!("peer_role"),
            torrust_metrics::label::LabelValue::new("seeder"),
        ));
        let value = stats_repository
            .read_metrics(|metrics| metrics.metric_collection().get_counter_value(&metric_name, &label_set))
            .await
            .map_or(0, |counter| counter.value());

        assert_eq!(value, expected_value);
    }

    #[tokio::test]
    async fn it_should_handle_an_event_then_stop_when_the_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Ok(peer_added_event()), Err(RecvError::Closed)]));

        // Act
        dispatch_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_continue_after_lag_then_handle_an_event_then_stop_when_the_receiver_is_closed() {
        // Arrange
        clock::Stopped::local_set_to_unix_epoch();
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([
            Err(RecvError::Lagged(2)),
            Ok(peer_added_event()),
            Err(RecvError::Closed),
        ]));

        // Act
        dispatch_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_stop_when_the_receiver_is_closed() {
        // Arrange
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Err(RecvError::Closed)]));

        // Act
        dispatch_events(
            receiver,
            CancellationToken::new(),
            stats_repository.clone(),
            DEFAULT_DRAIN_TIMEOUT,
        )
        .await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 0).await;
    }

    #[tokio::test]
    async fn it_should_handle_a_queued_event_then_return_cancelled_when_cancellation_is_already_pending() {
        // Arrange
        // A peer-added event is queued for the listener, and its token is cancelled before the
        // listener takes the event.
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Ok(peer_added_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion = dispatch_events(receiver, cancellation_token, stats_repository.clone(), DEFAULT_DRAIN_TIMEOUT).await;

        // Assert
        // The queued event is counted, and the listener reports that it stopped because of the
        // cancellation.
        expect_peer_added_metric_to_be(&stats_repository, 1).await;
        assert_eq!(completion, Completion::Cancelled, "a drained listener must return Cancelled");
    }

    #[tokio::test]
    async fn it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired() {
        // Arrange
        // The same queued event and cancelled token, with a drain timeout of zero, so it has
        // expired before the first event could be admitted.
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Ok(peer_added_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        let completion = dispatch_events(receiver, cancellation_token, stats_repository.clone(), Duration::ZERO).await;

        // Assert
        // Nothing was counted, because the timeout stops admission, and the listener still returns
        // Cancelled so the supervisor sees a cooperative stop.
        expect_peer_added_metric_to_be(&stats_repository, 0).await;
        assert_eq!(
            completion,
            Completion::Cancelled,
            "an expired drain must still return Cancelled"
        );
    }
}
