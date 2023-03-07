use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::http::axum_implementation::extractors::authentication_key::Extract as ExtractKey;
use crate::http::axum_implementation::extractors::client_ip_sources::Extract as ExtractClientIpSources;
use crate::http::axum_implementation::extractors::scrape_request::ExtractRequest;
use crate::http::axum_implementation::requests::scrape::Scrape;
use crate::http::axum_implementation::services::peer_ip_resolver::{self, ClientIpSources};
use crate::http::axum_implementation::{responses, services};
use crate::tracker::Tracker;

/* code-review: authentication, authorization and peer IP resolution could be moved
   from the handler (Axum) layer into the app layer `services::announce::invoke`.
   That would make the handler even simpler and the code more reusable and decoupled from Axum.
*/

#[allow(clippy::unused_async)]
pub async fn handle_without_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
) -> Response {
    debug!("http scrape request: {:#?}", &scrape_request);

    if tracker.requires_authentication() {
        return handle_fake_scrape(&tracker, &scrape_request, &client_ip_sources).await;
    }

    handle_real_scrape(&tracker, &scrape_request, &client_ip_sources).await
}

#[allow(clippy::unused_async)]
pub async fn handle_with_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
    ExtractKey(key): ExtractKey,
) -> Response {
    debug!("http scrape request: {:#?}", &scrape_request);

    match tracker.authenticate(&key).await {
        Ok(_) => (),
        Err(_) => return handle_fake_scrape(&tracker, &scrape_request, &client_ip_sources).await,
    }

    handle_real_scrape(&tracker, &scrape_request, &client_ip_sources).await
}

async fn handle_real_scrape(tracker: &Arc<Tracker>, scrape_request: &Scrape, client_ip_sources: &ClientIpSources) -> Response {
    let peer_ip = match peer_ip_resolver::invoke(tracker.config.on_reverse_proxy, client_ip_sources) {
        Ok(peer_ip) => peer_ip,
        Err(error) => return responses::error::Error::from(error).into_response(),
    };

    let scrape_data = services::scrape::invoke(tracker, &scrape_request.info_hashes, &peer_ip).await;

    responses::scrape::Bencoded::from(scrape_data).into_response()
}

/// When authentication fails in `private` mode the tracker returns empty swarm metadata for all the requested infohashes.
async fn handle_fake_scrape(tracker: &Arc<Tracker>, scrape_request: &Scrape, remote_client_ip: &ClientIpSources) -> Response {
    let peer_ip = match peer_ip_resolver::invoke(tracker.config.on_reverse_proxy, remote_client_ip) {
        Ok(peer_ip) => peer_ip,
        Err(error) => return responses::error::Error::from(error).into_response(),
    };

    let scrape_data = services::scrape::fake_invoke(tracker, &scrape_request.info_hashes, &peer_ip).await;

    responses::scrape::Bencoded::from(scrape_data).into_response()
}
