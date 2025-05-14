use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer::{Peer, PeerAnnouncement};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    TorrentAdded {
        info_hash: InfoHash,
        announcement: PeerAnnouncement,
    },
    TorrentRemoved {
        info_hash: InfoHash,
    },
    PeerAdded {
        peer: Peer,
    },
    PeerRemoved {
        peer: Peer,
    },
    PeerUpdated {
        old_peer: Peer,
        new_peer: Peer,
    },
    PeerDownloadCompleted {
        peer: Peer,
    },
}

pub mod sender {
    use std::sync::Arc;

    use super::Event;

    pub type Sender = Option<Arc<dyn torrust_tracker_events::sender::Sender<Event = Event>>>;
    pub type Broadcaster = torrust_tracker_events::broadcaster::Broadcaster<Event>;
}

pub mod receiver {
    use super::Event;

    pub type Receiver = Box<dyn torrust_tracker_events::receiver::Receiver<Event = Event>>;
}

pub mod bus {
    use crate::event::Event;

    pub type EventBus = torrust_tracker_events::bus::EventBus<Event>;
}
