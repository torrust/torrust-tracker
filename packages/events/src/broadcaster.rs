use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::broadcast::{self};

use crate::receiver::{Receiver, RecvError};
use crate::sender::{SendError, Sender};

const CHANNEL_CAPACITY: usize = 32768;

/// An event sender implementation using a broadcast channel.
#[derive(Clone)]
pub struct Broadcaster<E: Sync + Send + Clone> {
    pub(crate) sender: broadcast::Sender<E>,
}

impl<E: Sync + Send + Clone> Default for Broadcaster<E> {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }
}

impl<E: Sync + Send + Clone> Broadcaster<E> {
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<E> {
        self.sender.subscribe()
    }
}

impl<E: Sync + Send + Clone> Sender for Broadcaster<E> {
    type Event = E;

    fn send_event(&self, event: E) -> BoxFuture<'_, Option<Result<usize, SendError<E>>>> {
        async move { Some(self.sender.send(event).map_err(std::convert::Into::into)) }.boxed()
    }
}

impl<E: Sync + Send + Clone> Receiver for broadcast::Receiver<E> {
    type Event = E;

    fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>> {
        async move { self.recv().await.map_err(std::convert::Into::into) }.boxed()
    }
}

impl<E> From<broadcast::error::SendError<E>> for SendError<E> {
    fn from(err: broadcast::error::SendError<E>) -> Self {
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
