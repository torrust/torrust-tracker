use futures::future::BoxFuture;
use futures::FutureExt;
#[cfg(test)]
use mockall::{automock, predicate::str};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;

use super::Event;

/// A trait to allow sending events.
#[cfg_attr(test, automock)]
pub trait Sender: Sync + Send {
    fn send_event(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>>;
}

/// An event sender implementation using a broadcast channel.
#[allow(clippy::module_name_repetitions)]
pub struct ChannelSender {
    pub(crate) sender: broadcast::Sender<Event>,
}

impl Sender for ChannelSender {
    fn send_event(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>> {
        async move { Some(self.sender.send(event)) }.boxed()
    }
}
