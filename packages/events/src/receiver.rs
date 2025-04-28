use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};
use tokio::sync::broadcast::error::RecvError;

/// A trait for receiving events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Receiver: Sync + Send {
    type Event: Send + Clone;

    fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>>;
}
