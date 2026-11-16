//! Random data generators for testing.
use std::net::Ipv4Addr;

use rand::distr::Alphanumeric;
use rand::{RngExt, rng};

/// Returns a random alphanumeric string of a certain size.
///
/// It is useful for generating random names, IDs, etc for testing.
pub fn string(size: usize) -> String {
    rng().sample_iter(&Alphanumeric).take(size).map(char::from).collect()
}

/// Returns a random loopback address `127.a.b.c`, with `a`, `b` and `c` in `1..=254`, so never
/// `127.0.0.1`.
///
/// A test that must rebind a port after the code under test releases it can bind port `0` on this
/// address instead of `127.0.0.1`. Another test that binds `127.0.0.1:0` and receives the released
/// port then does not block the rebind, because two sockets on different specific addresses can
/// share a port. It does not protect against a socket bound to `0.0.0.0` on the same port.
///
/// Only Linux routes the whole `127.0.0.0/8` range to the loopback interface without extra
/// configuration; macOS and the BSDs configure only `127.0.0.1`. A test that binds this address
/// should be ignored on other systems.
///
/// issue: #2393
#[must_use]
pub fn loopback_ipv4() -> Ipv4Addr {
    let mut rng = rng();
    Ipv4Addr::new(
        127,
        rng.random_range(1..=254),
        rng.random_range(1..=254),
        rng.random_range(1..=254),
    )
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, TcpListener, UdpSocket};

    use super::loopback_ipv4;

    #[test]
    fn it_should_return_a_loopback_address_other_than_127_0_0_1() {
        for _ in 0..1_000 {
            let address = loopback_ipv4();

            assert!(address.is_loopback(), "{address} should be a loopback address");
            assert!(
                address.octets()[1..].iter().all(|octet| (1..=254).contains(octet)),
                "{address} should have its last three octets in 1..=254, so it is never 127.0.0.1"
            );
        }
    }

    #[test]
    #[cfg_attr(not(target_os = "linux"), ignore = "needs the Linux 127.0.0.0/8 loopback range, see #2393")]
    fn it_should_return_an_address_that_can_bind_a_port_another_socket_holds_on_127_0_0_1() {
        // Arrange
        let tcp_on_localhost = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind TCP on 127.0.0.1");
        let udp_on_localhost = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind UDP on 127.0.0.1");
        let address = loopback_ipv4();

        // Act
        let tcp = TcpListener::bind(SocketAddr::from((address, tcp_on_localhost.local_addr().unwrap().port())));
        let udp = UdpSocket::bind(SocketAddr::from((address, udp_on_localhost.local_addr().unwrap().port())));

        // Assert
        assert!(
            tcp.is_ok(),
            "{address} should bind the TCP port 127.0.0.1 holds: {:?}",
            tcp.err()
        );
        assert!(
            udp.is_ok(),
            "{address} should bind the UDP port 127.0.0.1 holds: {:?}",
            udp.err()
        );
    }
}
