//! API handlers for the [`health_check`](crate::v1::context::health_check)
//! API context.

use axum::Json;
use torrust_tracker_rest_api_protocol::v1::context::health_check::resources::report::{Report, Status};

/// Endpoint for container health check.
pub async fn health_check_handler() -> Json<Report> {
    Json(Report { status: Status::Ok })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use super::health_check_handler;
    use crate::test_support::{Observed, observed};

    #[tokio::test]
    async fn it_should_answer_ok_with_a_json_report_whose_status_is_ok() {
        let response = health_check_handler().await.into_response();

        assert_eq!(
            observed(response).await,
            Observed::new(StatusCode::OK, "application/json", r#"{"status":"Ok"}"#)
        );
    }
}
