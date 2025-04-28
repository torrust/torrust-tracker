use super::Event;

pub type Receiver = Box<dyn torrust_tracker_events::receiver::Receiver<Event = Event>>;
