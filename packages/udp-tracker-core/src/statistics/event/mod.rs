use std::net::SocketAddr;

pub mod handler;
pub mod listener;
pub mod sender;

/// An statistics event. It is used to collect tracker metrics.
///
/// - `Udp` prefix means the event was triggered by the UDP tracker
/// - `4` or `6` prefixes means the IP version used by the peer
/// - Finally the event suffix is the type of request: `announce`, `scrape` or `connection`
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Udp4Connect { context: ConnectionContext },
    Udp4Announce { context: ConnectionContext },
    Udp4Scrape { context: ConnectionContext },
    Udp6Connect { context: ConnectionContext },
    Udp6Announce { context: ConnectionContext },
    Udp6Scrape { context: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionContext {
    client_socket_addr: SocketAddr,
    server_socket_addr: SocketAddr,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(client_socket_addr: SocketAddr, server_socket_addr: SocketAddr) -> Self {
        Self {
            client_socket_addr,
            server_socket_addr,
        }
    }
}
