pub mod announce;
pub mod banning;
pub mod connect;
pub mod scrape;

#[cfg(test)]
pub(crate) mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_events::sender::SendError;

    use crate::event::Event;

    pub fn sample_ipv4_remote_addr() -> SocketAddr {
        sample_ipv4_socket_address()
    }

    pub fn sample_ipv4_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
    }

    pub fn sample_issue_time() -> f64 {
        1_000_000_000_f64
    }

    mock! {
        pub(crate) UdpCoreStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for UdpCoreStatsEventSender {
            type Event = Event;

            fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
        }
    }
}
