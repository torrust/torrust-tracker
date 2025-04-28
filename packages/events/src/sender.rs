use futures::future::BoxFuture;
#[cfg(test)]
use mockall::{automock, predicate::str};
use tokio::sync::broadcast::error::SendError;

/// A trait for sending events.
#[cfg_attr(test, automock(type Event=();))]
pub trait Sender: Sync + Send {
    type Event: Send + Clone;

    fn send_event(&self, event: Self::Event) -> BoxFuture<'_, Option<Result<usize, SendError<Self::Event>>>>;
}
