use std::net::SocketAddr;

pub mod handler;
pub mod listener;
pub mod sender;

/// An statistics event. It is used to collect tracker metrics.
///
/// - `Udp` prefix means the event was triggered by the UDP tracker.
/// - The event suffix is the type of request: `announce`, `scrape` or `connection`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    UdpConnect { context: ConnectionContext },
    UdpAnnounce { context: ConnectionContext },
    UdpScrape { context: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

    #[must_use]
    pub fn client_socket_addr(&self) -> SocketAddr {
        self.client_socket_addr
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server_socket_addr
    }
}
