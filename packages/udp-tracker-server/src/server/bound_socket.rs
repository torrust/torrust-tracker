use std::fmt::Debug;
use std::net::SocketAddr;
use std::ops::Deref;

use bittorrent_udp_tracker_core::UDP_TRACKER_LOG_TARGET;
use url::Url;

/// Wrapper for Tokio [`UdpSocket`][`tokio::net::UdpSocket`] that is bound to a particular socket.
pub struct BoundSocket {
    socket: tokio::net::UdpSocket,
}

impl BoundSocket {
    /// # Errors
    ///
    /// Will return an error if the socket can't be bound the the provided address.
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<std::io::Error>> {
        let bind_addr = format!("udp://{addr}");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, bind_addr, "UdpSocket::new (binding)");

        let socket = tokio::net::UdpSocket::bind(addr).await;

        let socket = match socket {
            Ok(socket) => socket,
            Err(e) => Err(e)?,
        };

        let local_addr = format!("udp://{}", socket.local_addr()?);
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "UdpSocket::new (bound)");

        Ok(Self { socket })
    }

    /// # Panics
    ///
    /// Will panic if the socket can't get the address it was bound to.
    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.socket.local_addr().expect("it should get local address")
    }

    /// # Panics
    ///
    /// Will panic if the address the socket was bound to is not a valid address
    /// to be used in a URL.
    #[must_use]
    pub fn url(&self) -> Url {
        Url::parse(&format!("udp://{}", self.address())).expect("UDP socket address should be valid")
    }
}

impl Deref for BoundSocket {
    type Target = tokio::net::UdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl Debug for BoundSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local_addr = match self.socket.local_addr() {
            Ok(socket) => format!("Receiving From: {socket}"),
            Err(err) => format!("Socket Broken: {err}"),
        };

        f.debug_struct("UdpSocket").field("addr", &local_addr).finish_non_exhaustive()
    }
}
