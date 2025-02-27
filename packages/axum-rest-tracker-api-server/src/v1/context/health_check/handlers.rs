//! API handlers for the [`stats`](crate::v1::context::health_check)
//! API context.

use axum::Json;

use super::resources::{Report, Status};

/// Endpoint for container health check.
pub async fn health_check_handler() -> Json<Report> {
    Json(Report { status: Status::Ok })
}
