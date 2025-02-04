use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bittorrent_udp_tracker_core::statistics;
use futures::future::BoxFuture;
use mockall::mock;
use tokio::sync::mpsc::error::SendError;

pub(crate) fn sample_ipv4_remote_addr() -> SocketAddr {
    sample_ipv4_socket_address()
}

pub(crate) fn sample_ipv4_socket_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
}

pub(crate) fn sample_issue_time() -> f64 {
    1_000_000_000_f64
}

mock! {
    pub(crate) UdpCoreStatsEventSender {}
    impl statistics::event::sender::Sender for UdpCoreStatsEventSender {
         fn send_event(&self, event: statistics::event::Event) -> BoxFuture<'static,Option<Result<(),SendError<statistics::event::Event> > > > ;
    }
}
