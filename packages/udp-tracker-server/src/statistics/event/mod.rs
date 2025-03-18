use std::net::SocketAddr;
use std::time::Duration;

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
    UdpRequestAborted {
        context: ConnectionContext,
    },
    UdpRequestBanned {
        context: ConnectionContext,
    },

    // UDP4
    Udp4IncomingRequest {
        context: ConnectionContext,
    },
    Udp4Request {
        context: ConnectionContext,
        kind: UdpRequestKind,
    },
    Udp4Response {
        context: ConnectionContext,
        kind: UdpResponseKind,
        req_processing_time: Duration,
    },
    Udp4Error {
        context: ConnectionContext,
    },

    // UDP6
    Udp6IncomingRequest {
        context: ConnectionContext,
    },
    Udp6Request {
        context: ConnectionContext,
        kind: UdpRequestKind,
    },
    Udp6Response {
        context: ConnectionContext,
        kind: UdpResponseKind,
        req_processing_time: Duration,
    },
    Udp6Error {
        context: ConnectionContext,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum UdpRequestKind {
    Connect,
    Announce,
    Scrape,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UdpResponseKind {
    Ok { req_kind: UdpRequestKind },
    Error, // todo: add the request kind `{ req_kind: Option(UdpRequestKind) }` when we know it.
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

    #[must_use]
    pub fn client_socket_addr(&self) -> SocketAddr {
        self.client_socket_addr
    }

    #[must_use]
    pub fn server_socket_addr(&self) -> SocketAddr {
        self.server_socket_addr
    }
}
