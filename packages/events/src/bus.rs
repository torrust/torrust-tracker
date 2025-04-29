use std::sync::Arc;

use crate::broadcaster::Broadcaster;
use crate::{receiver, sender};

#[derive(Clone, Debug)]
pub struct EventBus<Event: Sync + Send + Clone + 'static> {
    pub enable_sender: bool,
    pub broadcaster: Broadcaster<Event>,
}

impl<Event: Sync + Send + Clone + 'static> Default for EventBus<Event> {
    fn default() -> Self {
        let enable_sender = true;
        let broadcaster = Broadcaster::<Event>::default();

        Self::new(enable_sender, broadcaster)
    }
}

impl<Event: Sync + Send + Clone + 'static> EventBus<Event> {
    #[must_use]
    pub fn new(enable_sender: bool, broadcaster: Broadcaster<Event>) -> Self {
        Self {
            enable_sender,
            broadcaster,
        }
    }

    #[must_use]
    pub fn sender(&self) -> Option<Arc<dyn sender::Sender<Event = Event>>> {
        if self.enable_sender {
            Some(Arc::new(self.broadcaster.clone()))
        } else {
            None
        }
    }

    #[must_use]
    pub fn receiver(&self) -> Box<dyn receiver::Receiver<Event = Event>> {
        Box::new(self.broadcaster.subscribe())
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{timeout, Duration};

    use super::*;

    #[tokio::test]
    async fn it_should_provide_an_event_sender_when_enabled() {
        let bus = EventBus::<String>::new(true, Broadcaster::default());

        assert!(bus.sender().is_some());
    }

    #[tokio::test]
    async fn it_should_not_provide_event_sender_when_disabled() {
        let bus = EventBus::<String>::new(false, Broadcaster::default());

        assert!(bus.sender().is_none());
    }

    #[tokio::test]
    async fn it_should_enabled_by_default() {
        let bus = EventBus::<String>::default();

        assert!(bus.sender().is_some());
    }

    #[tokio::test]
    async fn it_should_allow_sending_events_that_are_received_by_receivers() {
        let bus = EventBus::<String>::default();
        let sender = bus.sender().unwrap();
        let mut receiver = bus.receiver();

        let event = "hello".to_string();

        let _unused = sender.send(event.clone()).await.unwrap().unwrap();

        let result = timeout(Duration::from_secs(1), receiver.recv()).await;

        assert_eq!(result.unwrap().unwrap(), event);
    }

    #[tokio::test]
    async fn it_should_send_a_closed_events_to_receivers_when_sender_is_dropped() {
        let bus = EventBus::<String>::default();

        let mut receiver = bus.receiver();

        let future = receiver.recv();

        drop(bus); // explicitly drop sender

        let result = timeout(Duration::from_secs(1), future).await;

        assert!(matches!(result.unwrap(), Err(crate::receiver::RecvError::Closed)));
    }
}
