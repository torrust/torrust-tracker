use axum::Json;

use super::resources::{CheckReport, Report};

pub fn ok(details: Vec<CheckReport>) -> Json<Report> {
    Json(Report::ok(details))
}

pub fn error(message: String, details: Vec<CheckReport>) -> Json<Report> {
    Json(Report::error(message, details))
}

pub fn none() -> Json<Report> {
    Json(Report::none())
}
