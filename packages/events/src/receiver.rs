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

/// An error returned from the [`recv`](Receiver::recv) function on a [`Receiver`].
///
/// This enum is intentionally exhaustive, not `#[non_exhaustive]`: each variant selects a
/// different recovery action in a receive loop (`Closed` ends it, `Lagged` continues), so a new
/// variant must be a breaking change that makes every consumer decide what to do with it.
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
            Self::Closed => write!(f, "channel closed"),
            Self::Lagged(amt) => write!(f, "channel lagged by {amt}"),
        }
    }
}

impl std::error::Error for RecvError {}
