use axum::Json;

use super::resources::Report;

pub fn ok() -> Json<Report> {
    Json(Report::ok())
}

pub fn error(message: String) -> Json<Report> {
    Json(Report::error(message))
}
