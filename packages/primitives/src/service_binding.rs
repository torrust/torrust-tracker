use std::fmt;
use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};
use url::Url;

const DUAL_STACK_IP_V4_MAPPED_V6_PREFIX: &str = "::ffff:";

/// Represents the supported network protocols.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Protocol {
    UDP,
    HTTP,
    HTTPS,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let proto_str = match self {
            Protocol::UDP => "udp",
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        };
        write!(f, "{proto_str}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum IpType {
    /// Represents a plain IPv4 or IPv6 address.
    Plain,

    /// Represents an IPv6 address that is a mapped IPv4 address.
    ///
    /// This is used for IPv6 addresses that represent an IPv4 address in a dual-stack network.
    ///
    /// For example: `[::ffff:192.0.2.33]`
    V4MappedV6,
}

impl fmt::Display for IpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ip_type_str = match self {
            Self::Plain => "plain",
            Self::V4MappedV6 => "v4_mapped_v6",
        };
        write!(f, "{ip_type_str}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum IpFamily {
    // IPv4
    Inet,
    // IPv6
    Inet6,
}

impl fmt::Display for IpFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ip_family_str = match self {
            Self::Inet => "inet",
            Self::Inet6 => "inet6",
        };
        write!(f, "{ip_family_str}")
    }
}

impl From<IpAddr> for IpFamily {
    fn from(ip: IpAddr) -> Self {
        if ip.is_ipv4() {
            return IpFamily::Inet;
        }

        if ip.is_ipv6() {
            return IpFamily::Inet6;
        }

        panic!("Unsupported IP address type: {ip}");
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("The port number cannot be zero. It must be an assigned valid port.")]
    PortZeroNotAllowed,
}

/// Represents a network service binding, encapsulating protocol and socket
/// address.
///
/// This struct is used to define how a service binds to a network interface and
/// port.
///
/// It's an URL without path and some restrictions:
///
/// - Only some schemes are accepted: `udp`, `http`, `https`.
/// - The port number must be greater than zero. The service should be already
///   listening on that port.
/// - The authority part of the URL must be a valid socket address (wildcard is
///   accepted).
///
/// Besides it accepts some non well-formed URLs, like:<http://127.0.0.1:7070>
/// or <https://127.0.0.1:7070>. Those URLs are not valid because they use non
/// standard ports (80 and 443).
///
/// NOTICE: It does not represent a public valid URL clients can connect to. It
/// represents the service's internal URL configuration after assigning a port.
/// If the port in the configuration is not zero, it's basically the same
/// information you get from the configuration (binding address + protocol).
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
/// use torrust_tracker_primitives::service_binding::{ServiceBinding, Protocol};
///
/// let service_binding = ServiceBinding::new(Protocol::HTTP, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070)).unwrap();
///
/// assert_eq!(service_binding.url().to_string(), "http://127.0.0.1:7070/".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ServiceBinding {
    /// The network protocol used by the service (UDP, HTTP, HTTPS).
    protocol: Protocol,

    /// The socket address (IP and port) to which the service binds.
    bind_address: SocketAddr,
}

impl ServiceBinding {
    /// # Errors
    ///
    /// This function will return an error if the port number is zero.
    pub fn new(protocol: Protocol, bind_address: SocketAddr) -> Result<Self, Error> {
        if bind_address.port() == 0 {
            return Err(Error::PortZeroNotAllowed);
        }

        Ok(Self { protocol, bind_address })
    }

    /// Returns the protocol used by the service.
    #[must_use]
    pub fn protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    #[must_use]
    pub fn bind_address(&self) -> SocketAddr {
        self.bind_address
    }

    #[must_use]
    pub fn bind_address_ip_type(&self) -> IpType {
        if self.is_v4_mapped_v6() {
            return IpType::V4MappedV6;
        }

        IpType::Plain
    }

    #[must_use]
    pub fn bind_address_ip_family(&self) -> IpFamily {
        self.bind_address.ip().into()
    }

    /// # Panics
    ///
    /// It never panics because the URL is always valid.
    #[must_use]
    pub fn url(&self) -> Url {
        Url::parse(&format!("{}://{}", self.protocol, self.bind_address))
            .expect("Service binding can always be parsed into a URL")
    }

    fn is_v4_mapped_v6(&self) -> bool {
        self.bind_address.ip().is_ipv6()
            && self
                .bind_address
                .ip()
                .to_string()
                .starts_with(DUAL_STACK_IP_V4_MAPPED_V6_PREFIX)
    }
}

impl From<ServiceBinding> for Url {
    fn from(service_binding: ServiceBinding) -> Self {
        service_binding.url()
    }
}

impl fmt::Display for ServiceBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

#[cfg(test)]
mod tests {

    mod the_service_binding {
        use std::net::SocketAddr;
        use std::str::FromStr;

        use rstest::rstest;
        use url::Url;

        use crate::service_binding::{Error, IpType, Protocol, ServiceBinding};

        #[rstest]
        #[case("wildcard_ip", Protocol::UDP, SocketAddr::from_str("0.0.0.0:6969").unwrap())]
        #[case("udp_service", Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap())]
        #[case("http_service", Protocol::HTTP, SocketAddr::from_str("127.0.0.1:7070").unwrap())]
        #[case("https_service", Protocol::HTTPS, SocketAddr::from_str("127.0.0.1:7070").unwrap())]
        fn should_allow_a_subset_of_urls(#[case] case: &str, #[case] protocol: Protocol, #[case] bind_address: SocketAddr) {
            let service_binding = ServiceBinding::new(protocol.clone(), bind_address);

            assert!(service_binding.is_ok(), "{}", format!("{case} failed: {service_binding:?}"));
        }

        #[test]
        fn should_not_allow_undefined_port_zero() {
            let service_binding = ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("127.0.0.1:0").unwrap());

            assert!(matches!(service_binding, Err(Error::PortZeroNotAllowed)));
        }

        #[test]
        fn should_return_the_bind_address() {
            let service_binding = ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap()).unwrap();

            assert_eq!(
                service_binding.bind_address(),
                SocketAddr::from_str("127.0.0.1:6969").unwrap()
            );
        }

        #[test]
        fn should_return_the_bind_address_plain_type_for_ipv4_ips() {
            let service_binding = ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap()).unwrap();

            assert_eq!(service_binding.bind_address_ip_type(), IpType::Plain);
        }

        #[test]
        fn should_return_the_bind_address_plain_type_for_ipv6_ips() {
            let service_binding =
                ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("[0:0:0:0:0:0:0:1]:6969").unwrap()).unwrap();

            assert_eq!(service_binding.bind_address_ip_type(), IpType::Plain);
        }

        #[test]
        fn should_return_the_bind_address_v4_mapped_v7_type_for_ipv4_ips_mapped_to_ipv6() {
            let service_binding =
                ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("[::ffff:192.0.2.33]:6969").unwrap()).unwrap();

            assert_eq!(service_binding.bind_address_ip_type(), IpType::V4MappedV6);
        }

        #[test]
        fn should_return_the_corresponding_url() {
            let service_binding = ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap()).unwrap();

            assert_eq!(service_binding.url(), Url::parse("udp://127.0.0.1:6969").unwrap());
        }

        #[test]
        fn should_be_converted_into_an_url() {
            let service_binding = ServiceBinding::new(Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap()).unwrap();

            let url: Url = service_binding.clone().into();

            assert_eq!(url, Url::parse("udp://127.0.0.1:6969").unwrap());
        }

        #[rstest]
        #[case("udp_service", Protocol::UDP, SocketAddr::from_str("127.0.0.1:6969").unwrap(), "udp://127.0.0.1:6969")]
        #[case("http_service", Protocol::HTTP, SocketAddr::from_str("127.0.0.1:7070").unwrap(), "http://127.0.0.1:7070/")]
        #[case("https_service", Protocol::HTTPS, SocketAddr::from_str("127.0.0.1:7070").unwrap(), "https://127.0.0.1:7070/")]
        fn should_always_have_a_corresponding_unique_url(
            #[case] case: &str,
            #[case] protocol: Protocol,
            #[case] bind_address: SocketAddr,
            #[case] expected_url: String,
        ) {
            let service_binding = ServiceBinding::new(protocol.clone(), bind_address).unwrap();

            assert_eq!(
                service_binding.url().to_string(),
                expected_url,
                "{case} failed: {service_binding:?}",
            );
        }
    }
}
