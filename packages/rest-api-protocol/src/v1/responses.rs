//! Protocol-level response types for the v1 REST API.
//!
//! These types define the serialization contract for API responses.
//! They are transport-agnostic and do not depend on Axum or any HTTP framework.
use serde::Serialize;

/// Response status used when requests have only two possible results
/// `Ok` or `Error` and no data is returned.
#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}
