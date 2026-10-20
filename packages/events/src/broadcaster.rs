use futures::FutureExt;
use futures::future::BoxFuture;
use tokio::sync::broadcast::{self};

use crate::receiver::{Receiver, RecvError, TryRecvError};
use crate::sender::{SendError, Sender};

const CHANNEL_CAPACITY: usize = 65536;

/// An event sender and receiver implementation using a broadcast channel.
#[derive(Clone, Debug)]
pub struct Broadcaster<Event: Sync + Send + Clone> {
    pub(crate) sender: broadcast::Sender<Event>,
}

impl<Event: Sync + Send + Clone> Default for Broadcaster<Event> {
    fn default() -> Self {
        let (sender, _receiver) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }
}

impl<Event: Sync + Send + Clone> Broadcaster<Event> {
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

impl<Event: Sync + Send + Clone> Sender for Broadcaster<Event> {
    type Event = Event;

    fn send(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>> {
        async move { Some(self.sender.send(event).map_err(std::convert::Into::into)) }.boxed()
    }
}

impl<Event: Sync + Send + Clone> Receiver for broadcast::Receiver<Event> {
    type Event = Event;

    fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>> {
        async move { self.recv().await.map_err(std::convert::Into::into) }.boxed()
    }

    fn try_recv(&mut self) -> Result<Self::Event, TryRecvError> {
        Self::try_recv(self).map_err(std::convert::Into::into)
    }

    fn len(&self) -> usize {
        Self::len(self)
    }
}

impl<Event> From<broadcast::error::SendError<Event>> for SendError<Event> {
    fn from(err: broadcast::error::SendError<Event>) -> Self {
        Self(err.0)
    }
}

impl From<broadcast::error::TryRecvError> for TryRecvError {
    fn from(err: broadcast::error::TryRecvError) -> Self {
        match err {
            broadcast::error::TryRecvError::Empty => Self::Empty,
            broadcast::error::TryRecvError::Closed => Self::Closed,
            broadcast::error::TryRecvError::Lagged(amt) => Self::Lagged(amt),
        }
    }
}

impl From<broadcast::error::RecvError> for RecvError {
    fn from(err: broadcast::error::RecvError) -> Self {
        match err {
            broadcast::error::RecvError::Lagged(amt) => Self::Lagged(amt),
            broadcast::error::RecvError::Closed => Self::Closed,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{Duration, timeout};

    use super::*;

    #[tokio::test]
    async fn it_should_allow_sending_an_event_and_received_it() {
        let broadcaster = Broadcaster::<String>::default();

        let mut receiver = broadcaster.subscribe();

        let event = "test";

        let _unused = broadcaster.send(event.to_owned()).await.unwrap().unwrap();

        let received_event = receiver.recv().await.unwrap();

        assert_eq!(received_event, event);
    }

    #[tokio::test]
    async fn it_should_return_the_number_of_receivers_when_and_event_is_sent() {
        let broadcaster = Broadcaster::<String>::default();
        let mut _receiver = broadcaster.subscribe();

        let number_of_receivers = broadcaster.send("test".into()).await;

        assert!(matches!(number_of_receivers, Some(Ok(1))));
    }

    #[tokio::test]
    async fn it_should_fail_when_trying_tos_send_with_no_subscribers() {
        let event = String::from("test");

        let broadcaster = Broadcaster::<String>::default();

        let result: Result<usize, SendError<String>> = broadcaster.send(event).await.unwrap();

        assert!(matches!(result, Err(SendError::<String>(_event))));
    }

    #[tokio::test]
    async fn it_should_allow_subscribing_multiple_receivers() {
        let broadcaster = Broadcaster::<u8>::default();
        let mut r1 = broadcaster.subscribe();
        let mut r2 = broadcaster.subscribe();

        let _ = broadcaster.send(1).await;

        let val1 = timeout(Duration::from_secs(1), r1.recv()).await.unwrap().unwrap();
        let val2 = timeout(Duration::from_secs(1), r2.recv()).await.unwrap().unwrap();

        assert_eq!(val1, 1);
        assert_eq!(val2, 1);
    }

    #[test]
    fn it_should_report_empty_without_waiting_when_the_receiver_has_read_every_event() {
        // Arrange
        // A channel whose only receiver has read the one event sent, with its sender alive.
        let (sender, mut receiver) = broadcast::channel::<u32>(4);
        sender.send(1).unwrap();
        assert_eq!(Receiver::try_recv(&mut receiver), Ok(1));

        // Act
        let result = Receiver::try_recv(&mut receiver);

        // Assert
        // The receiver reports `Empty`, not `Closed`, because a sender could still send.
        assert_eq!(result, Err(TryRecvError::Empty));
    }

    #[test]
    fn it_should_report_the_number_of_skipped_events_then_continue_with_the_oldest_retained_one() {
        // Arrange
        // A channel that holds two events, and five sent: three were overwritten before the
        // receiver read anything.
        let (sender, mut receiver) = broadcast::channel::<u32>(2);
        for event in 1..=5 {
            sender.send(event).unwrap();
        }

        // Act
        let first = Receiver::try_recv(&mut receiver);
        let second = Receiver::try_recv(&mut receiver);

        // Assert
        // The first call reports the three skipped events, and the next returns the oldest
        // event that is still retained.
        assert_eq!(first, Err(TryRecvError::Lagged(3)));
        assert_eq!(second, Ok(4));
    }

    #[test]
    fn it_should_report_closed_only_after_the_buffered_events_were_read() {
        // Arrange
        // One event is buffered and the only sender is dropped.
        let (sender, mut receiver) = broadcast::channel::<u32>(4);
        sender.send(1).unwrap();
        drop(sender);

        // Act
        let buffered = Receiver::try_recv(&mut receiver);
        let after = Receiver::try_recv(&mut receiver);

        // Assert
        // The buffered event is delivered first, and only then does the receiver report closed.
        assert_eq!(buffered, Ok(1));
        assert_eq!(after, Err(TryRecvError::Closed));
    }

    #[test]
    fn it_should_count_sent_positions_not_yet_received_including_overwritten_ones() {
        // Arrange
        // Capacity two, five events sent, and none received, then one event received after the
        // lag is reported.
        let (sender, mut receiver) = broadcast::channel::<u32>(2);
        for event in 1..=5 {
            sender.send(event).unwrap();
        }
        let before_any_receive = Receiver::len(&receiver);

        // Act
        let _lag = Receiver::try_recv(&mut receiver);
        let after_the_lag_was_reported = Receiver::len(&receiver);

        // Assert
        // Before the lag is reported, `len` counts all five sent positions, although only two are
        // readable. Once the lag was reported, only the two retained events remain.
        assert_eq!(before_any_receive, 5);
        assert_eq!(after_the_lag_was_reported, 2);
    }

    #[test]
    fn it_should_report_a_receiver_as_empty_only_when_it_has_no_unread_events() {
        // Arrange
        // One event is sent and not yet read by the only receiver.
        let (sender, mut receiver) = broadcast::channel::<u32>(4);
        sender.send(1).unwrap();
        let empty_with_an_unread_event = Receiver::is_empty(&receiver);

        // Act
        let _event = Receiver::try_recv(&mut receiver);
        let empty_after_reading_it = Receiver::is_empty(&receiver);

        // Assert
        // The receiver is not empty while the event is unread, and is empty once it has been read.
        assert!(!empty_with_an_unread_event, "a receiver with an unread event is not empty");
        assert!(empty_after_reading_it, "a receiver that has read every event is empty");
    }
}
