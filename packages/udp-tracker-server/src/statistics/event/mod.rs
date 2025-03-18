use std::net::SocketAddr;
use std::time::Duration;

pub mod handler;
pub mod listener;
pub mod sender;

/// An statistics event. It is used to collect tracker metrics.
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    UdpIncomingRequest {
        context: ConnectionContext,
    },
    UdpRequestAborted {
        context: ConnectionContext,
    },
    UdpRequestBanned {
        context: ConnectionContext,
    },
    UdpRequestAccepted {
        context: ConnectionContext,
        kind: UdpRequestKind,
    },
    UdpResponse {
        context: ConnectionContext,
        kind: UdpResponseKind,
        req_processing_time: Duration,
    },
    UdpError {
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
