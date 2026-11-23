use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use serde::Serialize;
use torrust_info_hash::InfoHash as TorrustInfoHash;
use torrust_tracker_udp_protocol::TransactionId;
use url::Url;

use crate::console::clients::udp::Error;
use crate::console::clients::udp::checker::{AnnounceParams, Client};

#[derive(Debug, Clone, Serialize)]
pub struct Checks {
    /// The configured tracker URL.
    url: String,
    /// The resolved address, or `None` when the URL did not resolve.
    remote_addr: Option<SocketAddr>,
    results: Vec<(Check, Result<(), Error>)>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Check {
    Setup,
    Connect,
    Announce,
    Scrape,
}

/// # Panics
///
/// Panics if the fixed sample info hash literal is invalid.
pub async fn run(udp_trackers: Vec<Url>, timeout: Duration) -> Vec<Result<Checks, Checks>> {
    let mut results = Vec::default();

    tracing::debug!("UDP trackers ...");

    let info_hash = TorrustInfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    for remote_url in udp_trackers {
        let mut checks = Checks {
            url: remote_url.to_string(),
            remote_addr: None,
            results: Vec::default(),
        };

        tracing::debug!("UDP tracker: {:?}", remote_url);

        // Setup
        let remote_addr = match resolve_socket_addr(&remote_url) {
            Ok(remote_addr) => remote_addr,
            Err(reason) => {
                checks.results.push((
                    Check::Setup,
                    Err(Error::UnableToResolveAddress {
                        url: remote_url.to_string(),
                        reason,
                    }),
                ));
                results.push(Err(checks));
                continue;
            }
        };
        checks.remote_addr = Some(remote_addr);

        let client = match Client::new(remote_addr, timeout).await {
            Ok(client) => {
                checks.results.push((Check::Setup, Ok(())));
                client
            }
            Err(err) => {
                checks.results.push((Check::Setup, Err(err)));
                results.push(Err(checks));
                continue;
            }
        };

        let transaction_id = TransactionId::new(1);

        // Connect Remote
        let connection_id = match client.send_connection_request(transaction_id).await {
            Ok(connection_id) => {
                checks.results.push((Check::Connect, Ok(())));
                connection_id
            }
            Err(err) => {
                checks.results.push((Check::Connect, Err(err)));
                results.push(Err(checks));
                continue;
            }
        };

        // Announce
        {
            let check = client
                .send_announce_request(transaction_id, connection_id, info_hash, &AnnounceParams::default())
                .await
                .map(|_| ());

            checks.results.push((Check::Announce, check));
        }

        // Scrape
        {
            let check = client
                .send_scrape_request(connection_id, transaction_id, &[info_hash])
                .await
                .map(|_| ());

            checks.results.push((Check::Scrape, check));
        }

        if checks.results.iter().any(|f| f.1.is_err()) {
            results.push(Err(checks));
        } else {
            results.push(Ok(checks));
        }
    }

    results
}

fn resolve_socket_addr(url: &Url) -> Result<SocketAddr, String> {
    let socket_addrs = url.socket_addrs(|| None).map_err(|err| err.to_string())?;

    socket_addrs
        .first()
        .copied()
        .ok_or_else(|| "the URL resolved to no socket addresses".to_string())
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use url::Url;

    use crate::console::clients::checker::checks::udp::resolve_socket_addr;

    #[test]
    fn it_should_resolve_the_socket_address_for_udp_scheme_urls_containing_a_domain() {
        let socket_addr = resolve_socket_addr(&Url::parse("udp://localhost:8080").unwrap()).unwrap();

        assert!(
            socket_addr == SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
                || socket_addr == SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080)
        );
    }

    #[test]
    fn it_should_resolve_the_socket_address_for_udp_scheme_urls_containing_an_ip() {
        let socket_addr = resolve_socket_addr(&Url::parse("udp://localhost:8080").unwrap()).unwrap();

        assert!(
            socket_addr == SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
                || socket_addr == SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080)
        );
    }

    #[test]
    fn it_should_return_an_error_instead_of_panicking_when_the_host_cannot_be_resolved() {
        // `.invalid` is reserved (RFC 6761) and never resolves. The test goes through the system
        // resolver, so it only asserts that an error is returned, not which one.
        let result = resolve_socket_addr(&Url::parse("udp://nonexistent.invalid:6969").unwrap());

        assert!(result.is_err());
    }
}
