//! Application services for the HTTP tracker.
//!
//! These modules contain logic that is specific for the HTTP tracker but it
//! does depend on the Axum web server. It could be reused for other web
//! servers.
//!
//! Refer to [`torrust_tracker`](crate) documentation.

use std::net::IpAddr;

use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{self, ClientIpSources, PeerIpResolutionError};
pub mod announce;
pub mod scrape;

/// Resolves the client's real IP address considering proxy headers
///
/// # Errors
///
/// This function returns an error if the IP address cannot be resolved.
pub fn resolve_remote_client_ip(
    on_reverse_proxy: bool,
    client_ip_sources: &ClientIpSources,
) -> Result<(IpAddr, Option<u16>), PeerIpResolutionError> {
    let ip = match peer_ip_resolver::invoke(on_reverse_proxy, client_ip_sources) {
        Ok(peer_ip) => Ok(peer_ip),
        Err(error) => Err(error),
    }?;

    let port = if client_ip_sources.connection_info_socket_address.is_some() {
        client_ip_sources
            .connection_info_socket_address
            .map(|socket_addr| socket_addr.port())
    } else {
        None
    };

    Ok((ip, port))
}
