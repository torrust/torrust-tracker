use std::fmt::Debug;
use std::net::SocketAddr;
use std::ops::Deref;

use socket2::{Domain, Socket, Type};
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;
use url::Url;

/// A UDP socket that has been successfully bound to a local address with a non-zero port.
///
/// # Invariant
///
/// The bound port is always non-zero. If port 0 is passed to [`BoundSocket::bind`], the OS
/// assigns an ephemeral port before construction completes, and the resulting address is
/// verified to have a non-zero port before the value is returned.
pub struct BoundSocket {
    socket: tokio::net::UdpSocket,
}

impl BoundSocket {
    /// Binds a UDP socket to `addr` and returns the bound socket.
    ///
    /// If `addr.port()` is 0 the OS assigns an ephemeral port; the resulting
    /// socket always has a non-zero port (see [`BoundSocket`] invariant).
    ///
    /// # Errors
    ///
    /// Returns an error if the socket cannot be created or bound, or if the
    /// OS unexpectedly assigns port 0 after a successful bind.
    pub fn bind(addr: SocketAddr, ipv6_v6only: bool) -> Result<Self, Box<std::io::Error>> {
        let bind_addr = format!("udp://{addr}");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, bind_addr, "UdpSocket::bind (binding)");

        let socket = Self::create_socket(addr, ipv6_v6only)?;
        let tokio_socket = tokio::net::UdpSocket::from_std(socket)?;

        let local_addr = tokio_socket.local_addr()?;
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr = %format!("udp://{local_addr}"), "UdpSocket::bind (bound)");

        if local_addr.port() == 0 {
            return Err(Box::new(std::io::Error::other(
                "bound socket has port 0 — OS did not assign an ephemeral port",
            )));
        }

        Ok(Self { socket: tokio_socket })
    }

    /// Creates a [`std::net::UdpSocket`] with `IPV6_V6ONLY` set according to
    /// the `ipv6_v6only` parameter.
    ///
    /// When `ipv6_v6only` is `true`, the socket is restricted to IPv6 only,
    /// allowing a separate IPv4 socket to bind on the same port
    /// (e.g. `0.0.0.0:6969` and `[::]:6969`).
    ///
    /// When `ipv6_v6only` is `false` (the default), the socket option is
    /// **not** explicitly set — the OS default applies. This means:
    ///
    /// | Platform | Default `IPV6_V6ONLY` | Behaviour with `false` |
    /// |---|---|---|
    /// | Linux | `0` (dual-stack) | Dual-stack — single `[::]` socket accepts IPv4 + IPv6 |
    /// | Windows, macOS, FreeBSD, Solaris | `1` (IPv6-only) | IPv6-only — must also bind `0.0.0.0:<port>` for IPv4 |
    /// | OpenBSD | `1` (forced) | IPv6-only — `IPV6_V6ONLY` cannot be disabled |
    ///
    /// We intentionally do **not** call `set_only_v6(false)` on any platform
    /// because:
    /// - On OpenBSD, `setsockopt(IPV6_V6ONLY, 0)` returns `EINVAL` (not
    ///   supported), which would cause a runtime panic.
    /// - On other non-Linux platforms, not touching the option preserves the
    ///   OS default (IPv6-only), which is the safe default.
    /// - On Linux, the OS default (dual-stack) is preserved without an extra
    ///   syscall.
    ///
    /// This means that operators on Windows, macOS, FreeBSD, and Solaris who
    /// want dual-stack behaviour must set `ipv6_v6only = false` explicitly
    /// (which is already the default) — the socket will remain IPv6-only on
    /// those platforms, matching their OS behaviour. To serve both IPv4 and
    /// IPv6 on those platforms, operators must configure a separate
    /// `0.0.0.0:<port>` entry. On Linux, a single `[::]:<port>` entry with
    /// `ipv6_v6only = false` (default) works as a dual-stack socket.
    fn create_socket(addr: SocketAddr, ipv6_v6only: bool) -> Result<std::net::UdpSocket, Box<std::io::Error>> {
        let domain = if addr.is_ipv6() { Domain::IPV6 } else { Domain::IPV4 };
        let socket = Socket::new(domain, Type::DGRAM, Some(socket2::Protocol::UDP))?;

        if addr.is_ipv6() && ipv6_v6only {
            socket.set_only_v6(true)?;
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
