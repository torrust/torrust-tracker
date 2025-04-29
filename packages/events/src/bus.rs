use std::sync::Arc;

use crate::broadcaster::Broadcaster;
use crate::{receiver, sender};

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
