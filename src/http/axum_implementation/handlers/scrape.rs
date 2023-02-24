use std::sync::Arc;

use axum::extract::State;
use log::debug;

use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::extractors::scrape_request::ExtractRequest;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn handle(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(scrape_request): ExtractRequest,
    _remote_client_ip: RemoteClientIp,
) -> String {
    debug!("http scrape request: {:#?}", &scrape_request);

    /*
    todo:
        - Add the service that sends the event for statistics.
        - Build the HTTP bencoded response.
    */

    let scrape_data = tracker.scrape(&scrape_request.info_hashes).await;

    debug!("scrape data: {:#?}", &scrape_data);

    "todo".to_string()
}
