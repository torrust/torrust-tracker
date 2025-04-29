use std::fmt;

use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};

/// A trait for receiving events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Receiver: Sync + Send {
    type Event: Send + Clone;

    fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>>;
}

/// An error returned from the [`recv`] function on a [`Receiver`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecvError {
    /// There are no more active senders implying no further messages will ever
    /// be sent.
    Closed,

    /// The receiver lagged too far behind. Attempting to receive again will
    /// return the oldest message still retained by the channel.
    ///
    /// Includes the number of skipped messages.
    Lagged(u64),
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecvError::Closed => write!(f, "channel closed"),
            RecvError::Lagged(amt) => write!(f, "channel lagged by {amt}"),
        }
    }
}

impl std::error::Error for RecvError {}
