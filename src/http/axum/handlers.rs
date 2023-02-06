use axum::response::Json;

use super::resources::ok::Ok;
use super::responses::ok_response;

#[allow(clippy::unused_async)]
pub async fn get_status_handler() -> Json<Ok> {
    ok_response()
}
