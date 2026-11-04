use axum::Json;

use super::resources::{CheckReport, Report};

pub const fn ok(details: Vec<CheckReport>) -> Json<Report> {
    Json(Report::ok(details))
}

pub const fn error(message: String, details: Vec<CheckReport>) -> Json<Report> {
    Json(Report::error(message, details))
}

pub fn none() -> Json<Report> {
    Json(Report::none())
}

/// The report of an application that is shutting down and no longer accepts new traffic.
pub fn shutting_down() -> Json<Report> {
    Json(Report::error("shutting down".to_string(), Vec::default()))
}
