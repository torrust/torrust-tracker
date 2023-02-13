use axum::Json;

use crate::http::axum_implementation::resources::ok::Ok;

#[must_use]
pub fn response() -> Json<Ok> {
    Json(Ok {})
}
