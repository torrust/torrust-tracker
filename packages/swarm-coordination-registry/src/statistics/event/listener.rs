use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_clock::clock::Time;
use torrust_tracker_events::receiver::RecvError;

use super::handler::handle_event;
use crate::event::receiver::Receiver;
use crate::statistics::repository::Repository;
use crate::{CurrentClock, SWARM_COORDINATION_REGISTRY_LOG_TARGET};

#[must_use]
pub fn run_event_listener(
    receiver: Receiver,
    cancellation_token: CancellationToken,
    repository: &Arc<Repository>,
) -> JoinHandle<()> {
    let stats_repository = repository.clone();

    tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Starting swarm coordination registry event listener");

    tokio::spawn(async move {
        dispatch_events(receiver, cancellation_token, stats_repository).await;

        tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Swarm coordination registry listener finished");
    })
}

async fn dispatch_events(mut receiver: Receiver, cancellation_token: CancellationToken, stats_repository: Arc<Repository>) {
    loop {
        tokio::select! {
            biased;

            () = cancellation_token.cancelled() => {
                tracing::info!(target: SWARM_COORDINATION_REGISTRY_LOG_TARGET, "Received cancellation request, shutting down swarm coordination registry event listener.");
                break;
            }

            result = receiver.recv() => {
                if !handle_receive_result(result, &stats_repository).await {
                    break;
                }
            }
        }
    }
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

    use futures::future::{self, BoxFuture};
    use tokio_util::sync::CancellationToken;
    use torrust_clock::clock;
    use torrust_clock::clock::stopped::Stopped;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;
    use torrust_tracker_events::receiver::RecvError;

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
            Box::pin(future::ready(self.results.pop_front().unwrap_or(Err(RecvError::Closed))))
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
            .get_metrics()
            .await
            .metric_collection
            .get_counter_value(&metric_name, &label_set)
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
        dispatch_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

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
        dispatch_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 1).await;
    }

    #[tokio::test]
    async fn it_should_stop_when_the_receiver_is_closed() {
        // Arrange
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Err(RecvError::Closed)]));

        // Act
        dispatch_events(receiver, CancellationToken::new(), stats_repository.clone()).await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 0).await;
    }

    #[tokio::test]
    async fn it_should_prioritize_pre_cancelled_token_over_a_ready_event() {
        // Arrange
        let stats_repository = Arc::new(Repository::new());
        let receiver: Receiver = Box::new(ScriptedReceiver::new([Ok(peer_added_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        dispatch_events(receiver, cancellation_token, stats_repository.clone()).await;

        // Assert
        expect_peer_added_metric_to_be(&stats_repository, 0).await;
    }
}
