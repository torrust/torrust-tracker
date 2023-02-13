use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;
use log::debug;

use crate::http::axum_implementation::requests::announce::ExtractAnnounceRequest;
use crate::http::axum_implementation::resources::ok::Ok;
use crate::http::axum_implementation::responses::ok;
use crate::tracker::Tracker;

/// WIP
#[allow(clippy::unused_async)]
pub async fn handle(
    State(_tracker): State<Arc<Tracker>>,
    ExtractAnnounceRequest(announce_request): ExtractAnnounceRequest,
) -> Json<Ok> {
    /* todo:
        - Extract remote client ip from request
        - Build the `Peer`
        - Call the `tracker.announce` method
        - Send event for stats
        - Move response from Warp to shared mod
        - Send response
    */

    // Sample announce URL used for debugging:
    // http://0.0.0.0:7070/announce?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548

    debug!("http announce request: {:#?}", announce_request);

    let info_hash = announce_request.info_hash;

    debug!("info_hash: {:#?}", &info_hash);

    ok::response()
}
