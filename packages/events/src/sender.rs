use std::fmt;
use std::fmt::Debug;

use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};

/// A trait for sending events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Sender: Sync + Send {
    type Event: Send + Clone;

    /// Sends an event to all active receivers.
    ///
    /// Returns a future that resolves to an `Option<Result<usize, SendError<Self::Event>>>`:
    ///
    /// - `Some(Ok(n))` — the event was successfully sent to `n` receivers.
    /// - `Some(Err(e))` — an error occurred while sending the event.
    /// - `None` — the sender is inactive or disconnected, and the event was not sent.
    ///
    /// The `Option` allows implementations to express cases where sending is not possible
    /// (e.g., when the sender is disabled or there are no active receivers).
    ///
    /// The `usize` typically represents the number of receivers the message was delivered to,
    /// but its semantics may vary depending on the concrete implementation.
    fn send(&self, event: Self::Event) -> BoxFuture<'_, Option<Result<usize, SendError<Self::Event>>>>;
}

/// Error returned by the [`send`] function on a [`Sender`].
#[derive(Debug)]
pub struct SendError<Event>(pub Event);

impl<Event> fmt::Display for SendError<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "channel closed")
    }
}

impl<Event: fmt::Debug> std::error::Error for SendError<Event> {}
