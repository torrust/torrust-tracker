use axum::Json;
use serde::{Deserialize, Serialize};

#[allow(clippy::unused_async)]
pub async fn handler() -> Json<Report> {
    Json(Report { status: Status::Ok })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Report {
    pub status: Status,
}
