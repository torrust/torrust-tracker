use std::sync::Arc;

use tokio::sync::broadcast::Receiver;

use crate::event::sender::{self, Broadcaster};
use crate::event::Event;

pub struct Keeper {
    pub enable_sender: bool,
    pub broadcaster: Broadcaster,
}

impl Default for Keeper {
    fn default() -> Self {
        let enable_sender = true;
        let broadcaster = Broadcaster::default();

        Self::new(enable_sender, broadcaster)
    }
}

impl Keeper {
    #[must_use]
    pub fn new(enable_sender: bool, broadcaster: Broadcaster) -> Self {
        Self {
            enable_sender,
            broadcaster,
        }
    }

    #[must_use]
    pub fn sender(&self) -> Arc<Option<Box<dyn sender::Sender>>> {
        if self.enable_sender {
            Arc::new(Some(Box::new(self.broadcaster.clone())))
        } else {
            Arc::new(None)
        }
    }

    #[must_use]
    pub fn receiver(&self) -> Receiver<Event> {
        self.broadcaster.subscribe()
    }
}
