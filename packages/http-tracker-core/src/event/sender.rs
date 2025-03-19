use futures::future::BoxFuture;
use futures::FutureExt;
#[cfg(test)]
use mockall::{automock, predicate::str};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;

use super::Event;

const CHANNEL_CAPACITY: usize = 1024;

/// A trait for sending sending.
#[cfg_attr(test, automock)]
pub trait Sender: Sync + Send {
    fn send_event(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>>;
}

/// An event sender implementation using a broadcast channel.
pub struct Broadcaster {
    pub(crate) sender: broadcast::Sender<Event>,
}

impl Sender for Broadcaster {
    fn send_event(&self, event: Event) -> BoxFuture<'_, Option<Result<usize, SendError<Event>>>> {
        async move { Some(self.sender.send(event)) }.boxed()
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }
}

impl Broadcaster {
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}
