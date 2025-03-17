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
    client: ClientConnectionContext,
    server: ServerConnectionContext,
}

impl ConnectionContext {
    #[must_use]
    pub fn new(client_ip_addr: IpAddr, opt_client_port: Option<u16>, server_socket_addr: SocketAddr) -> Self {
        Self {
            client: ClientConnectionContext {
                ip_addr: client_ip_addr,
                port: opt_client_port,
            },
            server: ServerConnectionContext {
                socket_addr: server_socket_addr,
            },
        }
    }

    #[must_use]
    pub fn client_ip_addr(&self) -> IpAddr {
        self.client.ip_addr
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClientConnectionContext {
    ip_addr: IpAddr,

    /// It's provided if you use the `torrust-axum-http-tracker-server` crate.
    port: Option<u16>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ServerConnectionContext {
    socket_addr: SocketAddr,
}
