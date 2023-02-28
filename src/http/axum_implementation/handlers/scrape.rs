use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::http::axum_implementation::extractors::peer_ip;
use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::extractors::scrape_request::ExtractRequest;
use crate::http::axum_implementation::handlers::auth;
use crate::http::axum_implementation::requests::scrape::Scrape;
use crate::http::axum_implementation::{responses, services};
use crate::tracker::auth::KeyId;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn handle_without_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    remote_client_ip: RemoteClientIp,
) -> Response {
    debug!("http scrape request: {:#?}", &scrape_request);

    if tracker.is_private() {
        return handle_fake_scrape(&tracker, &scrape_request, &remote_client_ip).await;
    }

    handle_real_scrape(&tracker, &scrape_request, &remote_client_ip).await
}

#[allow(clippy::unused_async)]
pub async fn handle_with_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    Path(key_id): Path<KeyId>,
    remote_client_ip: RemoteClientIp,
) -> Response {
    debug!("http scrape request: {:#?}", &scrape_request);

    match auth::authenticate(&key_id, &tracker).await {
        Ok(_) => (),
        Err(_) => return handle_fake_scrape(&tracker, &scrape_request, &remote_client_ip).await,
    }

    handle_real_scrape(&tracker, &scrape_request, &remote_client_ip).await
}

async fn handle_real_scrape(tracker: &Arc<Tracker>, scrape_request: &Scrape, remote_client_ip: &RemoteClientIp) -> Response {
    let peer_ip = match peer_ip::resolve(tracker.config.on_reverse_proxy, remote_client_ip) {
        Ok(peer_ip) => peer_ip,
        Err(err) => return err,
    };

    let scrape_data = services::scrape::invoke(tracker, &scrape_request.info_hashes, &peer_ip).await;

    responses::scrape::Bencoded::from(scrape_data).into_response()
}

/// When authentication fails in `private` mode the tracker returns empty swarm metadata for all the requested infohashes.
async fn handle_fake_scrape(tracker: &Arc<Tracker>, scrape_request: &Scrape, remote_client_ip: &RemoteClientIp) -> Response {
    let peer_ip = match peer_ip::resolve(tracker.config.on_reverse_proxy, remote_client_ip) {
        Ok(peer_ip) => peer_ip,
        Err(err) => return err,
    };

    let scrape_data = services::scrape::fake_invoke(tracker, &scrape_request.info_hashes, &peer_ip).await;

    responses::scrape::Bencoded::from(scrape_data).into_response()
}
