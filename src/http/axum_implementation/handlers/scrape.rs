use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::http::axum_implementation::extractors::peer_ip;
use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::extractors::scrape_request::ExtractRequest;
use crate::http::axum_implementation::{responses, services};
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn handle(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    remote_client_ip: RemoteClientIp,
) -> Response {
    debug!("http scrape request: {:#?}", &scrape_request);

    let peer_ip = match peer_ip::resolve(tracker.config.on_reverse_proxy, &remote_client_ip) {
        Ok(peer_ip) => peer_ip,
        Err(err) => return err,
    };

    let scrape_data = services::scrape::invoke(tracker.clone(), &scrape_request.info_hashes, &peer_ip).await;

    responses::scrape::Bencoded::from(scrape_data).into_response()
}
