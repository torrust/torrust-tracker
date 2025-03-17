use std::net::{IpAddr, SocketAddr};

pub mod handler;
pub mod listener;
pub mod sender;

/// An statistics event. It is used to collect tracker metrics.
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    TcpAnnounce { connection: ConnectionContext },
    TcpScrape { connection: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionContext {
    pub client_ip_addr: IpAddr,
    pub server_socket_addr: SocketAddr,
}
