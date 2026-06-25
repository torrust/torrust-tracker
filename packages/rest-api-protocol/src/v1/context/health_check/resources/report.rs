//! API resources for the health check endpoint.
//!
//! These types define the serialization contract for the `/api/health_check`
//! endpoint response. They are transport-agnostic and do not depend on Axum
//! or any HTTP framework.
use serde::{Deserialize, Serialize};

/// Health status of the API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    /// The API is healthy and running.
    Ok,
    /// The API has encountered an error.
    Error,
}

/// Health check report returned by the `/api/health_check` endpoint.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Report {
    /// The overall health status.
    pub status: Status,
}
