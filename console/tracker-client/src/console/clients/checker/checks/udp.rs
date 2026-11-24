use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use serde::Serialize;
use torrust_info_hash::InfoHash as TorrustInfoHash;
use torrust_tracker_udp_protocol::TransactionId;
use url::Url;

use crate::console::clients::checker::config::redacted_entry;
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
        // The path of a tracker URL can hold a key, so the URL written to the output is redacted;
        // `remote_url` itself is still the one used on the network.
        let displayed_url = redacted_entry(remote_url.as_str());

        let mut checks = Checks {
            url: displayed_url.clone(),
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
                        url: displayed_url,
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
    use std::time::Duration;

    use url::Url;

    use crate::console::clients::checker::checks::udp::{resolve_socket_addr, run};

    #[test]
    fn it_should_resolve_the_socket_address_for_udp_scheme_urls_containing_a_domain() {
        let socket_addr = resolve_socket_addr(&Url::parse("udp://localhost:8080").unwrap()).unwrap();

        assert!(
            socket_addr == SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)
                || socket_addr == SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080)
        );
    }

    #[test]
    fn it_should_resolve_the_socket_address_for_udp_scheme_urls_containing_an_ipv4_address() {
        let socket_addr = resolve_socket_addr(&Url::parse("udp://127.0.0.1:8080").unwrap()).unwrap();

        assert_eq!(socket_addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080));
    }

    #[test]
    fn it_should_resolve_the_socket_address_for_udp_scheme_urls_containing_an_ipv6_address() {
        let socket_addr = resolve_socket_addr(&Url::parse("udp://[::1]:8080").unwrap()).unwrap();

        assert_eq!(socket_addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080));
    }

    #[test]
    fn it_should_return_an_error_instead_of_panicking_when_the_host_cannot_be_resolved() {
        // `.invalid` is reserved (RFC 6761) and never resolves. The test goes through the system
        // resolver, so it only asserts that an error is returned, not which one.
        let result = resolve_socket_addr(&Url::parse("udp://nonexistent.invalid:6969").unwrap());

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn it_should_report_a_failed_setup_check_for_every_tracker_whose_address_does_not_resolve() {
        // URLs without a port never resolve, so the test needs no DNS and no network traffic.
        let urls = vec![Url::parse("udp://127.0.0.1").unwrap(), Url::parse("udp://127.0.0.2").unwrap()];

        let results = run(urls, Duration::from_millis(10)).await;

        let failed_checks: Vec<serde_json::Value> = results
            .iter()
            .map(|result| serde_json::to_value(result.as_ref().expect_err("the check should have failed")).unwrap())
            .collect();
        assert_eq!(failed_checks.len(), 2);
        for (check, url) in failed_checks.iter().zip(["udp://127.0.0.1", "udp://127.0.0.2"]) {
            assert_eq!(check["url"], url);
            assert_eq!(check["remote_addr"], serde_json::Value::Null);
            assert_eq!(check["results"].as_array().map(Vec::len), Some(1), "check: {check}");
            assert_eq!(check["results"][0][0], "Setup", "check: {check}");
            assert!(check["results"][0][1]["Err"].is_string(), "check: {check}");
        }
    }

    #[tokio::test]
    async fn it_should_not_write_the_path_of_a_tracker_url_in_the_check_result() {
        // The first URL resolves (and then times out, nothing listens); the second never resolves
        // because it has no port. Neither result may carry the path, which can hold a key.
        let urls = vec![
            Url::parse("udp://127.0.0.1:1/announce/PRIVATE_KEY").unwrap(),
            Url::parse("udp://127.0.0.1/announce/PRIVATE_KEY").unwrap(),
        ];

        let results = run(urls, Duration::from_millis(10)).await;

        let output: Vec<serde_json::Value> = results
            .iter()
            .map(|result| serde_json::to_value(result.as_ref().expect_err("the check should have failed")).unwrap())
            .collect();
        assert_eq!(output.len(), 2);
        assert!(
            !serde_json::to_string(&output).unwrap().contains("PRIVATE_KEY"),
            "output: {output:?}"
        );
        assert_eq!(output[0]["url"], "udp://127.0.0.1:1/****");
        assert_eq!(output[1]["url"], "udp://127.0.0.1/****");
    }
}
