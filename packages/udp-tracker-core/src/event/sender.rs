use std::sync::Arc;

use super::Event;

pub type Sender = Option<Arc<dyn torrust_tracker_events::sender::Sender<Event = Event>>>;
pub type Broadcaster = torrust_tracker_events::broadcaster::Broadcaster<Event>;
