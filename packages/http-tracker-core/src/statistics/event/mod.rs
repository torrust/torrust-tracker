use std::net::{IpAddr, SocketAddr};

pub mod handler;
pub mod listener;
pub mod sender;

/// An statistics event. It is used to collect tracker metrics.
///
/// - `Tcp` prefix means the event was triggered by the HTTP tracker
/// - `4` or `6` prefixes means the IP version used by the peer
/// - Finally the event suffix is the type of request: `announce` or `scrape`
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Tcp4Announce { connection: ConnectionContext },
    Tcp4Scrape { connection: ConnectionContext },
    Tcp6Announce { connection: ConnectionContext },
    Tcp6Scrape { connection: ConnectionContext },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionContext {
    pub client_ip_addr: IpAddr,
    pub server_socket_addr: SocketAddr,
}
