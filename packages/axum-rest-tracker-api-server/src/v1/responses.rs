//! Common responses for the API v1 shared by all the contexts.
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/* code-review:
    When Axum cannot parse a path or query param it shows a message like this:

    For the "seconds_valid_or_key" path param:

    "Invalid URL: Cannot parse "-1" to a `u64`"

    That message is not an informative message, specially if you have more than one param.
    We should show a message similar to the one we use when we parse the value in the handler.
    For example:

    "Invalid URL: invalid infohash param: string \"INVALID VALUE\", expected a 40 character long string"

    We can customize the error message by using a custom type with custom serde deserialization.
    The same we are using for the "InfoHashVisitor".

    Input data from HTTP requests should use struts with primitive types (first level of validation).
    We can put the second level of validation in the application and domain services.
*/

/// Response status used when requests have only two possible results
/// `Ok` or `Error` and no data is returned.
#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

// OK response

/// # Panics
///
/// Will panic if it can't convert the `ActionStatus` to json
#[must_use]
pub fn ok_response() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&ActionStatus::Ok).unwrap(),
    )
        .into_response()
}

// Error responses

#[must_use]
pub fn invalid_info_hash_param_response(info_hash: &str) -> Response {
    bad_request_response(&format!(
        "Invalid URL: invalid infohash param: string \"{info_hash}\", expected a 40 character long string"
    ))
}

#[must_use]
pub fn invalid_auth_key_param_response(invalid_key: &str) -> Response {
    bad_request_response(&format!("Invalid auth key id param \"{invalid_key}\""))
}

#[must_use]
pub fn bad_request_response(body: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body.to_owned(),
    )
        .into_response()
}

/// This error response is to keep backward compatibility with the old API.
/// It should be a plain text or json.
#[must_use]
pub fn unhandled_rejection_response(reason: String) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("Unhandled rejection: {:?}", ActionStatus::Err { reason: reason.into() }),
    )
        .into_response()
}
