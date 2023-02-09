use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;

use super::extractors::ExtractAnnounceParams;
use super::resources::ok::Ok;
use super::responses::ok_response;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn get_status_handler() -> Json<Ok> {
    ok_response()
}

/// # Panics
///
/// todo
#[allow(clippy::unused_async)]
pub async fn announce_handler(
    State(_tracker): State<Arc<Tracker>>,
    ExtractAnnounceParams(_announce_params): ExtractAnnounceParams,
) -> Json<Ok> {
    todo!()
}
