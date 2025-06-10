use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::broadcast::{self};

use crate::receiver::{Receiver, RecvError};
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
}

impl<Event> From<broadcast::error::SendError<Event>> for SendError<Event> {
    fn from(err: broadcast::error::SendError<Event>) -> Self {
        SendError(err.0)
    }
}

impl From<broadcast::error::RecvError> for RecvError {
    fn from(err: broadcast::error::RecvError) -> Self {
        match err {
            broadcast::error::RecvError::Lagged(amt) => RecvError::Lagged(amt),
            broadcast::error::RecvError::Closed => RecvError::Closed,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{timeout, Duration};

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
}
