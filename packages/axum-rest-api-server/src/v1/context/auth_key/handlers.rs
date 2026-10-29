//! API handlers for the [`auth_key`](crate::v1::context::auth_key) API context.
use std::sync::Arc;

use axum::extract::{self, Path, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_rest_api_application::v1::use_cases::auth_key::AuthKeyApiService;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError;

use super::responses::{
    auth_key_response, failed_to_add_key_response, failed_to_delete_key_response, failed_to_generate_key_response,
    failed_to_reload_keys_response, invalid_auth_key_duration_response, invalid_auth_key_response,
};
use crate::v1::responses::{disabled_by_configuration_response, invalid_auth_key_param_response, ok_response};

/// It handles the request to add a new authentication key.
///
/// It returns these types of responses:
///
/// - `200` with a json [`AuthKey`](torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKey)
///   resource. If the key was generated successfully.
/// - `400` with an error if the key couldn't been added because of an invalid
///   request.
/// - `500` with serialized error in debug format. If the key couldn't be
///   generated.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
pub async fn add_auth_key_handler(
    State(auth_key_service): State<Option<Arc<AuthKeyApiService>>>,
    extract::Json(add_key_form): extract::Json<AddKeyForm>,
) -> Response {
    let Some(auth_key_service) = auth_key_service else {
        return disabled_response();
    };

    match auth_key_service.add_key(&add_key_form).await {
        Ok(auth_key) => auth_key_response(&auth_key),
        Err(err) => match &err {
            AuthKeyError::DurationOverflow { seconds_valid } => invalid_auth_key_duration_response(*seconds_valid),
            AuthKeyError::InvalidKey { key, reason } => invalid_auth_key_response(key, reason),
            AuthKeyError::DisabledByConfiguration { .. } => disabled_response(),
            // `Database` and any variant added later: a generic failure response.
            _ => failed_to_add_key_response(AuthKeyErrorDisplay(&err)),
        },
    }
}

/// It handles the request to generate a new authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`AuthKey`](torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKey)
///   resource. If the key was generated successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///   generated.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#generate-a-new-authentication-key)
/// for more information about this endpoint.
///
/// This endpoint has been deprecated. Use [`add_auth_key_handler`].
pub async fn generate_auth_key_handler(
    State(auth_key_service): State<Option<Arc<AuthKeyApiService>>>,
    Path(seconds_valid_or_key): Path<u64>,
) -> Response {
    let Some(auth_key_service) = auth_key_service else {
        return disabled_response();
    };

    let seconds_valid = seconds_valid_or_key;
    match auth_key_service.generate_key(seconds_valid).await {
        Ok(auth_key) => auth_key_response(&auth_key),
        Err(e) => failed_to_generate_key_response(AuthKeyErrorDisplay(&e)),
    }
}

/// A container for the `key` parameter extracted from the URL PATH.
///
/// It does not perform any validation, it just stores the value.
#[derive(Deserialize)]
pub struct KeyParam(String);

/// It handles the request to delete an authentication key.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok)
///   response. If the key was deleted successfully.
/// - `500` with serialized error in debug format. If the key couldn't be
///   deleted.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#delete-an-authentication-key)
/// for more information about this endpoint.
pub async fn delete_auth_key_handler(
    State(auth_key_service): State<Option<Arc<AuthKeyApiService>>>,
    Path(seconds_valid_or_key): Path<KeyParam>,
) -> Response {
    let Some(auth_key_service) = auth_key_service else {
        return disabled_response();
    };

    match auth_key_service.delete_key(&seconds_valid_or_key.0).await {
        Ok(()) => ok_response(),
        Err(torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError::InvalidKey {
            key: _,
            reason: _,
        }) => invalid_auth_key_param_response(&seconds_valid_or_key.0),
        Err(e) => failed_to_delete_key_response(AuthKeyErrorDisplay(&e)),
    }
}

/// It handles the request to reload the authentication keys from the database
/// into memory.
///
/// It returns two types of responses:
///
/// - `200` with an json [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok)
///   response. If the keys were successfully reloaded.
/// - `500` with serialized error in debug format. If the they couldn't be
///   reloaded.
///
/// Refer to the [API endpoint documentation](crate::v1::context::auth_key#reload-authentication-keys)
/// for more information about this endpoint.
pub async fn reload_keys_handler(State(auth_key_service): State<Option<Arc<AuthKeyApiService>>>) -> Response {
    let Some(auth_key_service) = auth_key_service else {
        return disabled_response();
    };

    match auth_key_service.reload_keys().await {
        Ok(()) => ok_response(),
        Err(e) => failed_to_reload_keys_response(AuthKeyErrorDisplay(&e)),
    }
}

fn disabled_response() -> Response {
    disabled_by_configuration_response(&AuthKeyError::DisabledByConfiguration { capability: "private" }.to_string())
}

/// Wrapper to allow passing an [`AuthKeyError`] reference to response
/// functions that expect `E: std::error::Error`.
struct AuthKeyErrorDisplay<'a>(&'a torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKeyError);

impl std::fmt::Display for AuthKeyErrorDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.0, f)
    }
}

impl std::error::Error for AuthKeyErrorDisplay<'_> {}

impl std::fmt::Debug for AuthKeyErrorDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::Json;
    use axum::body::to_bytes;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::http::header::CONTENT_TYPE;
    use axum::response::Response;
    use rstest::rstest;
    use torrust_tracker_rest_api_application::v1::ports::auth_key::AuthKeyPort;
    use torrust_tracker_rest_api_application::v1::use_cases::auth_key::AuthKeyApiService;
    use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
    use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::{AuthKey, AuthKeyError};

    use super::{KeyParam, add_auth_key_handler, delete_auth_key_handler, generate_auth_key_handler, reload_keys_handler};
    use crate::test_support::MAX_BODY_BYTES;

    type KeyOutcome = fn() -> Result<AuthKey, AuthKeyError>;
    type UnitOutcome = fn() -> Result<(), AuthKeyError>;
    type CallLog = Arc<Mutex<Vec<String>>>;
    type ServiceState = State<Option<Arc<AuthKeyApiService>>>;

    const KEY_JSON: &str = r#"{"key":"k1","valid_until":null,"expiry_time":null}"#;

    /// What a client receives: the status, the content type and the body.
    #[derive(Debug, PartialEq, Eq)]
    struct Observed {
        status: StatusCode,
        content_type: String,
        body: String,
    }

    fn expected(status: StatusCode, content_type: &str, body: &str) -> Observed {
        Observed {
            status,
            content_type: content_type.to_string(),
            body: body.to_string(),
        }
    }

    fn disabled() -> Observed {
        expected(
            StatusCode::CONFLICT,
            "application/json",
            r#"{"status":"err","reason":"private capability is disabled by configuration"}"#,
        )
    }

    fn ok() -> Observed {
        expected(StatusCode::OK, "application/json", r#"{"status":"ok"}"#)
    }

    fn unhandled(reason: &str) -> Observed {
        expected(
            StatusCode::INTERNAL_SERVER_ERROR,
            "text/plain; charset=utf-8",
            &format!(r#"Unhandled rejection: Err {{ reason: "{reason}" }}"#),
        )
    }

    fn bad_request(body: &str) -> Observed {
        expected(StatusCode::BAD_REQUEST, "text/plain; charset=utf-8", body)
    }

    async fn observed(response: Response) -> Observed {
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

        Observed {
            status,
            content_type,
            body: String::from_utf8(body.to_vec()).expect("the body is text"),
        }
    }

    #[allow(deprecated, reason = "the resource still carries the deprecated `valid_until` field")]
    #[allow(clippy::unnecessary_wraps, reason = "it must have the outcome type of the fake port")]
    fn a_key() -> Result<AuthKey, AuthKeyError> {
        Ok(AuthKey {
            key: "k1".to_string(),
            valid_until: None,
            expiry_time: None,
        })
    }

    #[allow(clippy::unnecessary_wraps, reason = "it must have the outcome type of the fake port")]
    fn nothing() -> Result<(), AuthKeyError> {
        Ok(())
    }

    /// An `AuthKeyPort` that answers each call with a scripted outcome and records the call.
    struct FakeAuthKeyPort {
        key_outcome: KeyOutcome,
        unit_outcome: UnitOutcome,
        calls: CallLog,
    }

    impl FakeAuthKeyPort {
        fn answering(key_outcome: KeyOutcome, unit_outcome: UnitOutcome) -> Self {
            Self {
                key_outcome,
                unit_outcome,
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn call_log(&self) -> CallLog {
            self.calls.clone()
        }

        fn record(&self, call: String) {
            self.calls.lock().expect("the call log is not poisoned").push(call);
        }
    }

    #[async_trait]
    impl AuthKeyPort for FakeAuthKeyPort {
        async fn add_key(&self, form: &AddKeyForm) -> Result<AuthKey, AuthKeyError> {
            self.record(format!("add_key({:?}, {:?})", form.opt_key, form.opt_seconds_valid));
            (self.key_outcome)()
        }

        async fn generate_key(&self, seconds_valid: u64) -> Result<AuthKey, AuthKeyError> {
            self.record(format!("generate_key({seconds_valid})"));
            (self.key_outcome)()
        }

        async fn delete_key(&self, key: &str) -> Result<(), AuthKeyError> {
            self.record(format!("delete_key({key:?})"));
            (self.unit_outcome)()
        }

        async fn reload_keys(&self) -> Result<(), AuthKeyError> {
            self.record("reload_keys".to_string());
            (self.unit_outcome)()
        }
    }

    fn state_with(fake: FakeAuthKeyPort) -> ServiceState {
        State(Some(Arc::new(AuthKeyApiService::new(Box::new(fake)))))
    }

    fn state_answering(key_outcome: KeyOutcome, unit_outcome: UnitOutcome) -> ServiceState {
        state_with(FakeAuthKeyPort::answering(key_outcome, unit_outcome))
    }

    fn database_error() -> AuthKeyError {
        AuthKeyError::Database("boom".to_string())
    }

    fn a_form() -> Json<AddKeyForm> {
        Json(AddKeyForm {
            opt_key: Some("k1".to_string()),
            opt_seconds_valid: Some(5),
        })
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_adding_a_key_and_the_private_capability_is_not_available() {
        let response = add_auth_key_handler(State(None), a_form()).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_generating_a_key_and_the_private_capability_is_not_available() {
        let response = generate_auth_key_handler(State(None), Path(60)).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_deleting_a_key_and_the_private_capability_is_not_available() {
        let response = delete_auth_key_handler(State(None), Path(KeyParam("abc".to_string()))).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_reloading_the_keys_and_the_private_capability_is_not_available() {
        let response = reload_keys_handler(State(None)).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[rstest]
    #[case::the_added_key(
        (|| a_key()) as KeyOutcome,
        expected(StatusCode::OK, "application/json; charset=utf-8", KEY_JSON)
    )]
    #[case::a_duration_overflow(
        (|| Err(AuthKeyError::DurationOverflow { seconds_valid: u64::MAX })) as KeyOutcome,
        bad_request(r#"Invalid URL: invalid auth key duration: "18446744073709551615""#)
    )]
    #[case::an_invalid_key(
        (|| Err(AuthKeyError::InvalidKey { key: "bad key".to_string(), reason: "too short".to_string() })) as KeyOutcome,
        bad_request(r#"Invalid URL: invalid auth key: string "bad key", too short"#)
    )]
    #[case::a_capability_disabled_in_the_service(
        (|| Err(AuthKeyError::DisabledByConfiguration { capability: "private" })) as KeyOutcome,
        disabled()
    )]
    #[case::a_database_error((|| Err(database_error())) as KeyOutcome, unhandled("failed to add key: boom"))]
    #[tokio::test]
    async fn it_should_map_the_outcome_of_adding_a_key_to_a_response(#[case] outcome: KeyOutcome, #[case] expected: Observed) {
        let state = state_answering(outcome, nothing);

        let response = add_auth_key_handler(state, a_form()).await;

        assert_eq!(observed(response).await, expected);
    }

    #[tokio::test]
    async fn it_should_forward_the_form_to_the_service_when_adding_a_key() {
        let fake = FakeAuthKeyPort::answering(a_key, nothing);
        let calls = fake.call_log();

        add_auth_key_handler(state_with(fake), a_form()).await;

        assert_eq!(
            *calls.lock().expect("the call log is not poisoned"),
            vec![r#"add_key(Some("k1"), Some(5))"#]
        );
    }

    #[tokio::test]
    async fn it_should_answer_the_generated_key_and_forward_the_seconds_to_the_service_when_generating_a_key() {
        let fake = FakeAuthKeyPort::answering(a_key, nothing);
        let calls = fake.call_log();

        let response = generate_auth_key_handler(state_with(fake), Path(60)).await;

        assert_eq!(
            (
                observed(response).await,
                calls.lock().expect("the call log is not poisoned").clone()
            ),
            (
                expected(StatusCode::OK, "application/json; charset=utf-8", KEY_JSON),
                vec!["generate_key(60)".to_string()]
            )
        );
    }

    #[rstest]
    #[case::a_database_error((|| Err(database_error())) as KeyOutcome, "failed to generate key: boom")]
    #[case::an_invalid_key(
        (|| Err(AuthKeyError::InvalidKey { key: "x".to_string(), reason: "y".to_string() })) as KeyOutcome,
        r#"failed to generate key: invalid key: \"x\", y"#
    )]
    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_whatever_the_error_when_generating_a_key(
        #[case] outcome: KeyOutcome,
        #[case] reason: &str,
    ) {
        let state = state_answering(outcome, nothing);

        let response = generate_auth_key_handler(state, Path(60)).await;

        assert_eq!(observed(response).await, unhandled(reason));
    }

    #[tokio::test]
    async fn it_should_answer_ok_and_forward_the_key_to_the_service_when_deleting_a_key() {
        let fake = FakeAuthKeyPort::answering(a_key, nothing);
        let calls = fake.call_log();

        let response = delete_auth_key_handler(state_with(fake), Path(KeyParam("abc".to_string()))).await;

        assert_eq!(
            (
                observed(response).await,
                calls.lock().expect("the call log is not poisoned").clone()
            ),
            (ok(), vec![r#"delete_key("abc")"#.to_string()])
        );
    }

    #[tokio::test]
    async fn it_should_name_the_path_parameter_when_the_service_rejects_the_key_to_delete() {
        let state = state_answering(a_key, || {
            Err(AuthKeyError::InvalidKey {
                key: "service-key".to_string(),
                reason: "rejected".to_string(),
            })
        });

        let response = delete_auth_key_handler(state, Path(KeyParam("abc".to_string()))).await;

        assert_eq!(observed(response).await, bad_request(r#"Invalid auth key id param "abc""#));
    }

    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_when_deleting_a_key_fails_in_the_database() {
        let state = state_answering(a_key, || Err(database_error()));

        let response = delete_auth_key_handler(state, Path(KeyParam("abc".to_string()))).await;

        assert_eq!(observed(response).await, unhandled("failed to delete key: boom"));
    }

    #[tokio::test]
    async fn it_should_answer_ok_when_the_keys_are_reloaded() {
        let state = state_answering(a_key, nothing);

        let response = reload_keys_handler(state).await;

        assert_eq!(observed(response).await, ok());
    }

    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_when_reloading_the_keys_fails() {
        let state = state_answering(a_key, || Err(database_error()));

        let response = reload_keys_handler(state).await;

        assert_eq!(observed(response).await, unhandled("failed to reload keys: boom"));
    }
}
