use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;
use axum_client_ip::{InsecureClientIp, SecureClientIp};
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
    insecure_ip: InsecureClientIp,
    secure_ip: SecureClientIp,
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

    let info_hash = announce_request.info_hash;

    debug!("http announce request: {:#?}", announce_request);
    debug!("info_hash: {:#?}", &info_hash);
    debug!("remote client ip, insecure_ip: {:#?}", &insecure_ip);
    debug!("remote client ip, secure_ip: {:#?}", &secure_ip);

    ok::response(&insecure_ip.0, &secure_ip.0)
}
