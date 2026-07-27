//! Swarm coordination registry events.
//!
//! # Design contract: events are objective facts
//!
//! Every variant in [`Event`] describes *what happened* — a neutral, observable
//! fact. Events must not be designed around what a particular consumer should or
//! should not do in response. Policy decisions belong in the consumer or the
//! enforcement point, never in the event definition.
//!
//! See [ADR-20260727000000](../../../../docs/adrs/20260727000000_events_are_objective_facts.md)
//! for the full rationale, the concrete counter-example, and naming heuristics.
use torrust_info_hash::InfoHash;
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
        info_hash: InfoHash,
        peer: Peer,
    },
    PeerRemoved {
        info_hash: InfoHash,
        peer: Peer,
    },
    PeerUpdated {
        info_hash: InfoHash,
        old_peer: Peer,
        new_peer: Peer,
    },
    PeerDownloadCompleted {
        info_hash: InfoHash,
        peer: Peer,
    },
}

pub mod sender {
    use std::sync::Arc;

    use super::Event;

    pub type Sender = Option<Arc<dyn torrust_tracker_events::sender::Sender<Event = Event>>>;
    pub type Broadcaster = torrust_tracker_events::broadcaster::Broadcaster<Event>;

    #[cfg(test)]
    pub mod tests {

        use futures::future::{self, BoxFuture};
        use mockall::mock;
        use mockall::predicate::eq;
        use torrust_tracker_events::sender::{SendError, Sender};

        use crate::event::Event;

        mock! {
            pub EventSender {}

            impl Sender for EventSender {
                type Event = Event;

                fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
            }
        }

        pub fn expect_event(mock: &mut MockEventSender, event: Event) {
            mock.expect_send()
                .with(eq(event))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
        }

        pub fn expect_event_sequence(mock: &mut MockEventSender, event: Vec<Event>) {
            for e in event {
                expect_event(mock, e);
            }
        }
    }
}

pub mod receiver {
    use super::Event;

    pub type Receiver = Box<dyn torrust_tracker_events::receiver::Receiver<Event = Event>>;
}

pub mod bus {
    use crate::event::Event;

    pub type EventBus = torrust_tracker_events::bus::EventBus<Event>;
}

#[cfg(test)]
pub mod test {

    use torrust_tracker_primitives::peer::Peer;

    use super::Event;
    use crate::tests::sample_info_hash;

    #[test]
    fn events_should_be_comparable() {
        let info_hash = sample_info_hash();

        let event1 = Event::TorrentAdded {
            info_hash,
            announcement: Peer::default(),
        };

        let event2 = Event::TorrentRemoved { info_hash };

        let event1_clone = event1.clone();

        assert_eq!(event1, event1_clone);
        assert_ne!(event1, event2);
    }
}
