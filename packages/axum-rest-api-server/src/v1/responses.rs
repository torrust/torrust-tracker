//! Common responses for the API v1 shared by all the contexts.
use axum::http::{StatusCode, header};
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

/// `409` response when a capability required by a route is disabled.
///
/// # Panics
///
/// Will panic if it cannot serialize the [`ActionStatus`] response to JSON.
#[must_use]
pub fn disabled_by_configuration_response(reason: &str) -> Response {
    (
        StatusCode::CONFLICT,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&ActionStatus::Err { reason: reason.into() }).unwrap(),
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

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::http::header::CONTENT_TYPE;
    use axum::response::Response;
    use rstest::rstest;

    use super::{
        bad_request_response, disabled_by_configuration_response, invalid_auth_key_param_response,
        invalid_info_hash_param_response, ok_response, unhandled_rejection_response,
    };
    use crate::test_support::MAX_BODY_BYTES;

    const JSON: &str = "application/json";
    const TEXT: &str = "text/plain; charset=utf-8";

    /// The parts of a response that clients depend on: status, content type and body.
    async fn sent(response: Response) -> (StatusCode, String, String) {
        let status = response.status();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .expect("the response has a content type")
            .to_str()
            .expect("the content type is text")
            .to_string();
        let body = to_bytes(response.into_body(), MAX_BODY_BYTES)
            .await
            .expect("the body is readable and within the limit");

        (
            status,
            content_type,
            String::from_utf8(body.to_vec()).expect("the body is text"),
        )
    }

    #[tokio::test]
    async fn it_should_answer_ok_with_a_json_status_ok_body() {
        let response = ok_response();

        let observed = sent(response).await;

        assert_eq!(observed, (StatusCode::OK, JSON.to_string(), r#"{"status":"ok"}"#.to_string()));
    }

    #[rstest]
    #[case::an_invalid_info_hash(
        invalid_info_hash_param_response("abc"),
        r#"Invalid URL: invalid infohash param: string "abc", expected a 40 character long string"#
    )]
    #[case::an_invalid_auth_key(invalid_auth_key_param_response("bad-key"), r#"Invalid auth key id param "bad-key""#)]
    #[case::a_custom_message(bad_request_response("custom"), "custom")]
    #[tokio::test]
    async fn it_should_answer_a_bad_request_with_the_message_as_plain_text(#[case] response: Response, #[case] message: &str) {
        let observed = sent(response).await;

        assert_eq!(observed, (StatusCode::BAD_REQUEST, TEXT.to_string(), message.to_string()));
    }

    #[rstest]
    #[case::a_plain_reason("capability is off", r#"{"status":"err","reason":"capability is off"}"#)]
    #[case::a_reason_with_quotes(r#"say "no""#, r#"{"status":"err","reason":"say \"no\""}"#)]
    #[tokio::test]
    async fn it_should_answer_conflict_with_the_reason_in_a_json_error_body_when_a_capability_is_disabled(
        #[case] reason: &str,
        #[case] body: &str,
    ) {
        let response = disabled_by_configuration_response(reason);

        let observed = sent(response).await;

        assert_eq!(observed, (StatusCode::CONFLICT, JSON.to_string(), body.to_string()));
    }

    #[rstest]
    #[case::a_plain_reason("failed", r#"Unhandled rejection: Err { reason: "failed" }"#)]
    #[case::a_reason_with_quotes(r#"say "no""#, r#"Unhandled rejection: Err { reason: "say \"no\"" }"#)]
    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_with_the_legacy_debug_text_of_the_reason(
        #[case] reason: &str,
        #[case] body: &str,
    ) {
        let response = unhandled_rejection_response(reason.to_string());

        let observed = sent(response).await;

        assert_eq!(
            observed,
            (StatusCode::INTERNAL_SERVER_ERROR, TEXT.to_string(), body.to_string())
        );
    }
}
