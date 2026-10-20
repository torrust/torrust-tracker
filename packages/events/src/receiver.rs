use std::fmt;

use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};

/// A trait for receiving events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Receiver: Sync + Send {
    type Event: Send + Clone;

    fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>>;

    /// Takes the next event this receiver has not read yet, without waiting.
    ///
    /// # Errors
    ///
    /// Returns [`TryRecvError::Empty`] when the receiver has read every event sent so
    /// far, [`TryRecvError::Lagged`] when sends overwrote events it had not read, and
    /// [`TryRecvError::Closed`] when it has read everything and no sender is left.
    /// Buffered events are delivered before `Closed`.
    fn try_recv(&mut self) -> Result<Self::Event, TryRecvError>;

    /// The number of events sent and not yet received by this receiver.
    ///
    /// It counts positions, not recoverable events: it includes events that sends
    /// already overwrote, until a [`TryRecvError::Lagged`] result advances the
    /// receiver. Use it to report how much was left unread, not how much a drain
    /// could still have processed.
    fn len(&self) -> usize;

    /// Whether this receiver has no unread events. See [`len`](Receiver::len).
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
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

/// An error returned from the [`try_recv`](Receiver::try_recv) function on a [`Receiver`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TryRecvError {
    /// The receiver has read every event sent so far, but a sender is still alive,
    /// so a later event may arrive.
    Empty,

    /// There are no more active senders implying no further messages will ever
    /// be sent, and the receiver has read every buffered event.
    Closed,

    /// The receiver lagged too far behind. Attempting to receive again will
    /// return the oldest message still retained by the channel.
    ///
    /// Includes the number of skipped messages.
    Lagged(u64),
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "channel empty"),
            Self::Closed => write!(f, "channel closed"),
            Self::Lagged(amt) => write!(f, "channel lagged by {amt}"),
        }
    }
}

impl std::error::Error for TryRecvError {}
