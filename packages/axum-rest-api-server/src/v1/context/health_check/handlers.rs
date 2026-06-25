//! API handlers for the [`stats`](crate::v1::context::health_check)
//! API context.

use axum::Json;
use torrust_tracker_rest_api_protocol::v1::context::health_check::resources::report::{Report, Status};

/// Endpoint for container health check.
pub async fn health_check_handler() -> Json<Report> {
    Json(Report { status: Status::Ok })
}
