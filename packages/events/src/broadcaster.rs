use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::broadcast::{self};

use crate::receiver::{Receiver, RecvError};
use crate::sender::{SendError, Sender};

const CHANNEL_CAPACITY: usize = 32768;

/// An event sender implementation using a broadcast channel.
#[derive(Clone)]
pub struct Broadcaster<Event: Sync + Send + Clone> {
    pub(crate) sender: broadcast::Sender<Event>,
}

impl<Event: Sync + Send + Clone> Default for Broadcaster<Event> {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
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
