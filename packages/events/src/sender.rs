use std::fmt;

use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};

/// A trait for sending events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Sender: Sync + Send {
    type Event: Send + Clone;

    fn send_event(&self, event: Self::Event) -> BoxFuture<'_, Option<Result<usize, SendError<Self::Event>>>>;
}

/// Error returned by the [`send_event`] function on a [`Sender`].
#[derive(Debug)]
pub struct SendError<Event>(pub Event);

impl<Event> fmt::Display for SendError<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "channel closed")
    }
}

impl<T: fmt::Debug> std::error::Error for SendError<T> {}
