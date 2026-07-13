//! Tracker REST API client, scoped to E2E test needs.
//!
//! Wraps the official [`torrust_tracker_rest_api_client::v1::client::ApiHttpClient`] so that
//! future scenario steps can call any REST API endpoint through the same client
//! without having to reconstruct connection details each time.
use anyhow::Context;
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_client::v1::client::ApiHttpClient;
use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::Torrent;

use super::super::types::InfoHash;
use super::config_builder::TrackerConfig;

/// Wrapper around the official Torrust Tracker REST API client.
///
/// Provides typed, high-level helpers for the endpoints used in E2E test scenarios.
/// All other endpoints are still reachable through the inner [`ApiHttpClient`].
pub(crate) struct TrackerApiClient {
    inner: ApiHttpClient,
}

impl TrackerApiClient {
    /// Creates a new client connected to the tracker REST API on the given host port.
    ///
    /// # Errors
    ///
    /// Returns an error if the origin URL cannot be parsed or the HTTP client
    /// cannot be built.
    pub(crate) fn new(host_port: u16, tracker_config: &TrackerConfig) -> anyhow::Result<Self> {
        let origin = Origin::new(&format!("http://127.0.0.1:{host_port}")) // DevSkim: ignore DS137138
            .context("failed to parse tracker REST API origin")?;

        let connection_info = ConnectionInfo::authenticated(origin, tracker_config.access_token());

        let inner = ApiHttpClient::new(connection_info).context("failed to build tracker REST API client")?;

        Ok(Self { inner })
    }

    /// Returns the full [`Torrent`] resource for the torrent identified by `hash`.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, the server returns a non-2xx
    /// status, or the response body cannot be deserialized.
    pub(crate) async fn get_torrent(&self, hash: &InfoHash) -> anyhow::Result<Torrent> {
        let response = self.inner.get_torrent(hash.as_str(), None).await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "tracker REST API returned status {} for torrent {hash}",
                response.status()
            ));
        }

        response
            .json::<Torrent>()
            .await
            .with_context(|| format!("failed to deserialize tracker torrent response for {hash}"))
    }
}
