use std::fmt::Debug;
use std::net::SocketAddr;
use std::ops::Deref;

use socket2::{Domain, Socket, Type};
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use url::Url;

/// Wrapper for Tokio [`UdpSocket`][`tokio::net::UdpSocket`] that is bound to a particular socket.
pub struct BoundSocket {
    socket: tokio::net::UdpSocket,
}

impl BoundSocket {
    /// # Errors
    ///
    /// Will return an error if the socket can't be bound the the provided address.
    pub fn new(addr: SocketAddr, ipv6_v6only: bool) -> Result<Self, Box<std::io::Error>> {
        let bind_addr = format!("udp://{addr}");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, bind_addr, "UdpSocket::new (binding)");

        let socket = Self::create_socket(addr, ipv6_v6only)?;
        let tokio_socket = tokio::net::UdpSocket::from_std(socket)?;

        let local_addr = format!("udp://{}", tokio_socket.local_addr()?);
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, "UdpSocket::new (bound)");

        Ok(Self { socket: tokio_socket })
    }

    /// Creates a [`std::net::UdpSocket`] with `IPV6_V6ONLY` set according to
    /// the `ipv6_v6only` parameter. When `true`, the socket is restricted to
    /// IPv6 only, allowing a separate IPv4 socket to bind on the same port
    /// (e.g. `0.0.0.0:6969` and `[::]:6969`). When `false` (default), the
    /// socket operates in dual-stack mode and accepts both IPv4 and IPv6
    /// connections on a single bind.
    fn create_socket(addr: SocketAddr, ipv6_v6only: bool) -> Result<std::net::UdpSocket, Box<std::io::Error>> {
        let domain = if addr.is_ipv6() { Domain::IPV6 } else { Domain::IPV4 };
        let socket = Socket::new(domain, Type::DGRAM, Some(socket2::Protocol::UDP))?;

        if addr.is_ipv6() {
            socket.set_only_v6(ipv6_v6only)?;
        }

        socket.set_nonblocking(true)?;
        socket.bind(&addr.into())?;

        Ok(socket.into())
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

    /// # Panics
    ///
    /// It should never panic because the conversion to a [`ServiceBinding`]
    /// is infallible.
    #[must_use]
    pub fn service_binding(&self) -> ServiceBinding {
        ServiceBinding::new(Protocol::UDP, self.address()).expect("Conversion to ServiceBinding should not fail")
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
