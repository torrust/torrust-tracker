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
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::http::header::CONTENT_TYPE;
    use axum::response::IntoResponse;

    use super::health_check_handler;

    #[tokio::test]
    async fn it_should_answer_ok_with_a_json_report_whose_status_is_ok() {
        let (parts, body) = health_check_handler().await.into_response().into_parts();

        let body = to_bytes(body, usize::MAX).await.expect("the body is readable");
        assert_eq!(
            (parts.status, parts.headers[CONTENT_TYPE].as_bytes(), &body[..]),
            (StatusCode::OK, &b"application/json"[..], &br#"{"status":"Ok"}"#[..])
        );
    }
}
