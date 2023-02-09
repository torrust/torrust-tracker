// Resource responses

use axum::Json;

use super::resources::ok::Ok;

#[must_use]
pub fn ok_response() -> Json<Ok> {
    Json(Ok {})
}
