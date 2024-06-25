//! Console clients.

use std::net::{SocketAddr, ToSocketAddrs as _};

use anyhow::Context as _;
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::debug;
use url::Url;

pub mod checker;
pub mod http;
pub mod udp;

const DEFAULT_TIMEOUT_SEC: &str = "5";

fn parse_info_hash(info_hash: &str) -> anyhow::Result<InfoHash> {
    info_hash
        .parse()
        .map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{info_hash}`: {e:?}")))
}

fn parse_url(addr: &str) -> anyhow::Result<Url> {
    Url::parse(addr).with_context(|| format!("failed to parse URL: `{addr}`"))
}

fn parse_socket_addr(addr: &str) -> anyhow::Result<SocketAddr> {
    debug!("Tracker socket address: {addr:#?}");

    // Check if the address is a valid URL. If so, extract the host and port.
    let resolved_addr = if let Ok(url) = Url::parse(addr) {
        debug!("Tracker socket address URL: {url:?}");

        let host = url
            .host_str()
            .with_context(|| format!("invalid host in URL: `{addr}`"))?
            .to_owned();

        let port = url
            .port()
            .with_context(|| format!("port not found in URL: `{addr}`"))?
            .to_owned();

        (host, port)
    } else {
        // If not a URL, assume it's a host:port pair.

        let parts: Vec<&str> = addr.split(':').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid address format: `{}`. Expected format is host:port",
                addr
            ));
        }

        let host = parts[0].to_owned();

        let port = parts[1]
            .parse::<u16>()
            .with_context(|| format!("invalid port: `{}`", parts[1]))?
            .to_owned();

        (host, port)
    };

    debug!("Resolved address: {resolved_addr:#?}");

    // Perform DNS resolution.
    let socket_addrs: Vec<_> = resolved_addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        Err(anyhow::anyhow!("DNS resolution failed for `{}`", addr))
    } else {
        Ok(socket_addrs[0])
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;

    use crate::console::clients::DEFAULT_TIMEOUT_SEC;

    #[test]
    fn check_timeout_default_is_same_as_configuration_default() {
        assert_eq!(DEFAULT_TIMEOUT_SEC, &CLIENT_TIMEOUT_DEFAULT.as_secs().to_string());
    }
}
