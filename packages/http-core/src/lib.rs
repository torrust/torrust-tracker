pub mod container;
pub mod event;
pub mod services;
pub mod statistics;

use torrust_clock::clock;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

pub const HTTP_TRACKER_LOG_TARGET: &str = "HTTP TRACKER";

#[cfg(test)]
pub(crate) mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::sync::{Arc, Mutex};

    use futures::future::BoxFuture;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_events::sender::SendError;
    use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId, peer};

    use crate::event::Event;

    /// An event sender that keeps every event it receives, so a test can assert
    /// on the events a service publishes explicitly.
    ///
    /// Like the production broadcaster, it records an event only when the
    /// future returned by `send` is polled, so a service that drops that
    /// future publishes nothing.
    #[derive(Default)]
    pub struct RecordingSender {
        events: Arc<Mutex<Vec<Event>>>,
    }

    impl RecordingSender {
        #[must_use]
        pub fn new() -> Arc<Self> {
            Arc::new(Self::default())
        }

        /// # Panics
        ///
        /// Will panic if another thread panicked while recording an event.
        #[must_use]
        pub fn events(&self) -> Vec<Event> {
            self.events.lock().unwrap().clone()
        }
    }

    impl torrust_tracker_events::sender::Sender for RecordingSender {
        type Event = Event;

        fn send(&self, event: Event) -> BoxFuture<'static, Option<Result<usize, SendError<Event>>>> {
            let events = Arc::clone(&self.events);
            Box::pin(async move {
                events.lock().unwrap().push(event);
                Some(Ok(1))
            })
        }
    }

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }

    pub fn sample_peer_using_ipv4() -> peer::Peer {
        sample_peer()
    }

    pub fn sample_peer_using_ipv6() -> peer::Peer {
        let mut peer = sample_peer();
        peer.peer_addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
            8080,
        );
        peer
    }

    pub fn sample_peer() -> peer::Peer {
        peer::Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0),
            event: AnnounceEvent::Started,
        }
    }
}

#[cfg(test)]
mod root_tests {
    use std::net::SocketAddr;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_events::sender::Sender;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

    use crate::HTTP_TRACKER_LOG_TARGET;
    use crate::event::{ConnectionContext, Event};
    use crate::tests::{RecordingSender, sample_info_hash, sample_peer, sample_peer_using_ipv4, sample_peer_using_ipv6};

    /// Scrape events that differ only by their public URL, so a test can tell them apart.
    fn scrape_event(public_url: &str) -> Event {
        let connection = ConnectionContext::new(
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
            RemoteClientAddr::new(ResolvedIp::FromSocketAddr("126.0.0.1".parse().unwrap()), Some(8080)),
            ServiceBinding::new(Protocol::HTTP, SocketAddr::new("127.0.0.1".parse().unwrap(), 7070)).unwrap(),
        )
        .with_public_url(Some(public_url.to_string()));

        Event::TcpScrape { connection }
    }

    #[test]
    fn it_should_log_http_tracker_events_under_the_http_tracker_target() {
        assert_eq!(HTTP_TRACKER_LOG_TARGET, "HTTP TRACKER");
    }

    #[test]
    fn it_should_record_no_event_before_any_event_is_sent() {
        let sender = RecordingSender::new();

        assert_eq!(sender.events(), vec![]);
    }

    #[tokio::test]
    async fn it_should_record_an_event_when_the_send_future_is_polled() {
        let sender = RecordingSender::new();

        let result = sender.send(scrape_event("http://a.example")).await;

        assert_eq!(result.and_then(Result::ok), Some(1));
        assert_eq!(sender.events(), vec![scrape_event("http://a.example")]);
    }

    #[tokio::test]
    async fn it_should_record_events_in_the_order_they_are_sent() {
        let sender = RecordingSender::new();

        sender.send(scrape_event("http://a.example")).await;
        sender.send(scrape_event("http://b.example")).await;

        assert_eq!(
            sender.events(),
            vec![scrape_event("http://a.example"), scrape_event("http://b.example")]
        );
    }

    #[tokio::test]
    async fn it_should_record_nothing_when_the_send_future_is_dropped_without_being_polled() {
        let sender = RecordingSender::new();

        drop(sender.send(scrape_event("http://dropped.example")));
        sender.send(scrape_event("http://polled.example")).await;

        assert_eq!(sender.events(), vec![scrape_event("http://polled.example")]);
    }

    #[test]
    fn it_should_sample_a_fixed_info_hash() {
        assert_eq!(sample_info_hash().to_string(), "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0"); // DevSkim: ignore DS173237
    }

    #[test]
    fn it_should_sample_an_ipv4_peer_that_is_the_default_sample_peer() {
        assert_eq!(sample_peer_using_ipv4(), sample_peer());
        assert_eq!(sample_peer().peer_addr, "126.0.0.1:8080".parse::<SocketAddr>().unwrap());
    }

    #[test]
    fn it_should_sample_an_ipv6_peer_that_differs_from_the_ipv4_sample_only_by_its_address() {
        let mut expected = sample_peer();
        expected.peer_addr = "[6969:6969:6969:6969:6969:6969:6969:6969]:8080".parse().unwrap();

        assert_eq!(sample_peer_using_ipv6(), expected);
    }
}
