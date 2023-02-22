use std::sync::Arc;

use axum::extract::State;
use log::debug;

use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::extractors::scrape_request::ExtractRequest;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn handle(
    State(_tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    _remote_client_ip: RemoteClientIp,
) -> String {
    debug!("http scrape request: {:#?}", &scrape_request);

    format!("{:#?}", &scrape_request)
}
