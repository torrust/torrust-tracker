use std::sync::Arc;

use tokio::sync::broadcast::Receiver;

use crate::broadcaster::Broadcaster;
use crate::sender;

pub struct EventBus<E: Sync + Send + Clone + 'static> {
    pub enable_sender: bool,
    pub broadcaster: Broadcaster<E>,
}

impl<E: Sync + Send + Clone + 'static> Default for EventBus<E> {
    fn default() -> Self {
        let enable_sender = true;
        let broadcaster = Broadcaster::<E>::default();

        Self::new(enable_sender, broadcaster)
    }
}

impl<E: Sync + Send + Clone + 'static> EventBus<E> {
    #[must_use]
    pub fn new(enable_sender: bool, broadcaster: Broadcaster<E>) -> Self {
        Self {
            enable_sender,
            broadcaster,
        }
    }

    #[must_use]
    pub fn sender(&self) -> Arc<Option<Box<dyn sender::Sender<Event = E>>>> {
        if self.enable_sender {
            Arc::new(Some(Box::new(self.broadcaster.clone())))
        } else {
            Arc::new(None)
        }
    }

    #[must_use]
    pub fn receiver(&self) -> Receiver<E> {
        self.broadcaster.subscribe()
    }
}
