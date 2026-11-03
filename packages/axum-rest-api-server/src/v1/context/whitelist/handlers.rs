//! API handlers for the [`whitelist`](crate::v1::context::whitelist)
//! API context.
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Response;
use torrust_info_hash::InfoHash;
use torrust_tracker_rest_api_application::v1::use_cases::whitelist::WhitelistApiService;
use torrust_tracker_rest_api_protocol::v1::context::whitelist::resources::whitelist::WhitelistError;

use super::responses::{
    failed_to_reload_whitelist_response, failed_to_remove_torrent_from_whitelist_response, failed_to_whitelist_torrent_response,
};
use crate::InfoHashParam;
use crate::v1::responses::{disabled_by_configuration_response, invalid_info_hash_param_response, ok_response};

/// It handles the request to add a torrent to the whitelist.
///
/// It returns:
///
/// - `200` response with a [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok) in json.
/// - `500` with serialized error in debug format if the torrent couldn't be whitelisted.
///
/// Refer to the [API endpoint documentation](crate::v1::context::whitelist#add-a-torrent-to-the-whitelist)
/// for more information about this endpoint.
pub async fn add_torrent_to_whitelist_handler(
    State(whitelist_service): State<Option<Arc<WhitelistApiService>>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    let Some(whitelist_service) = whitelist_service else {
        return disabled_response();
    };

    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => match whitelist_service.add_torrent(&info_hash).await {
            Ok(()) => ok_response(),
            Err(e) => failed_to_whitelist_torrent_response(e),
        },
    }
}

/// It handles the request to remove a torrent to the whitelist.
///
/// It returns:
///
/// - `200` response with a [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok) in json.
/// - `500` with serialized error in debug format if the torrent couldn't be
///   removed from the whitelisted.
///
/// Refer to the [API endpoint documentation](crate::v1::context::whitelist#remove-a-torrent-from-the-whitelist)
/// for more information about this endpoint.
pub async fn remove_torrent_from_whitelist_handler(
    State(whitelist_service): State<Option<Arc<WhitelistApiService>>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    let Some(whitelist_service) = whitelist_service else {
        return disabled_response();
    };

    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => match whitelist_service.remove_torrent(&info_hash).await {
            Ok(()) => ok_response(),
            Err(e) => failed_to_remove_torrent_from_whitelist_response(e),
        },
    }
}

/// It handles the request to reload the torrent whitelist from the database.
///
/// It returns:
///
/// - `200` response with a [`ActionStatus::Ok`](crate::v1::responses::ActionStatus::Ok) in json.
/// - `500` with serialized error in debug format if the torrent whitelist
///   couldn't be reloaded from the database.
///
/// Refer to the [API endpoint documentation](crate::v1::context::whitelist#reload-the-whitelist)
/// for more information about this endpoint.
pub async fn reload_whitelist_handler(State(whitelist_service): State<Option<Arc<WhitelistApiService>>>) -> Response {
    let Some(whitelist_service) = whitelist_service else {
        return disabled_response();
    };

    match whitelist_service.reload().await {
        Ok(()) => ok_response(),
        Err(e) => failed_to_reload_whitelist_response(e),
    }
}

fn disabled_response() -> Response {
    disabled_by_configuration_response(&WhitelistError::DisabledByConfiguration { capability: "listed" }.to_string())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use rstest::rstest;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_rest_api_application::v1::ports::whitelist::WhitelistCommandPort;
    use torrust_tracker_rest_api_application::v1::use_cases::whitelist::WhitelistApiService;
    use torrust_tracker_rest_api_protocol::v1::context::whitelist::resources::whitelist::WhitelistError;

    use super::{add_torrent_to_whitelist_handler, reload_whitelist_handler, remove_torrent_from_whitelist_handler};
    use crate::InfoHashParam;
    use crate::test_support::{Observed, observed};

    type Outcome = fn() -> Result<(), WhitelistError>;
    type CallLog = Arc<Mutex<Vec<String>>>;
    type ServiceState = State<Option<Arc<WhitelistApiService>>>;

    const INFO_HASH: &str = "443c7602b4fde83d1154d6d9da48808418b181b6";

    fn disabled() -> Observed {
        Observed::new(
            StatusCode::CONFLICT,
            "application/json",
            r#"{"status":"err","reason":"listed capability is disabled by configuration"}"#,
        )
    }

    fn ok() -> Observed {
        Observed::new(StatusCode::OK, "application/json", r#"{"status":"ok"}"#)
    }

    fn unhandled(reason: &str) -> Observed {
        Observed::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "text/plain; charset=utf-8",
            &format!(r#"Unhandled rejection: Err {{ reason: "{reason}" }}"#),
        )
    }

    fn invalid_info_hash(value: &str) -> Observed {
        Observed::new(
            StatusCode::BAD_REQUEST,
            "text/plain; charset=utf-8",
            &format!(r#"Invalid URL: invalid infohash param: string "{value}", expected a 40 character long string"#),
        )
    }

    #[allow(clippy::unnecessary_wraps, reason = "it must have the outcome type of the fake port")]
    fn succeeds() -> Result<(), WhitelistError> {
        Ok(())
    }

    fn database_error() -> Result<(), WhitelistError> {
        Err(WhitelistError::Database("boom".to_string()))
    }

    /// A `WhitelistCommandPort` that answers each call with a scripted outcome and records the call.
    struct FakeWhitelistPort {
        outcome: Outcome,
        calls: CallLog,
    }

    impl FakeWhitelistPort {
        fn answering(outcome: Outcome) -> Self {
            Self {
                outcome,
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
    impl WhitelistCommandPort for FakeWhitelistPort {
        async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
            self.record(format!("add_torrent({info_hash})"));
            (self.outcome)()
        }

        async fn remove_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
            self.record(format!("remove_torrent({info_hash})"));
            (self.outcome)()
        }

        async fn reload(&self) -> Result<(), WhitelistError> {
            self.record("reload".to_string());
            (self.outcome)()
        }
    }

    fn state_with(fake: FakeWhitelistPort) -> ServiceState {
        State(Some(Arc::new(WhitelistApiService::new(Box::new(fake)))))
    }

    fn state_answering(outcome: Outcome) -> ServiceState {
        state_with(FakeWhitelistPort::answering(outcome))
    }

    fn param(value: &str) -> Path<InfoHashParam> {
        Path(InfoHashParam(value.to_string()))
    }

    fn recorded(calls: &CallLog) -> Vec<String> {
        calls.lock().expect("the call log is not poisoned").clone()
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_whitelisting_a_torrent_and_the_listed_capability_is_not_available() {
        let response = add_torrent_to_whitelist_handler(State(None), param(INFO_HASH)).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_removing_a_torrent_and_the_listed_capability_is_not_available() {
        let response = remove_torrent_from_whitelist_handler(State(None), param(INFO_HASH)).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_answer_conflict_when_reloading_the_whitelist_and_the_listed_capability_is_not_available() {
        let response = reload_whitelist_handler(State(None)).await;

        assert_eq!(observed(response).await, disabled());
    }

    #[tokio::test]
    async fn it_should_reject_an_invalid_info_hash_without_calling_the_service_when_whitelisting_a_torrent() {
        let fake = FakeWhitelistPort::answering(succeeds);
        let calls = fake.call_log();

        let response = add_torrent_to_whitelist_handler(state_with(fake), param("abc")).await;

        assert_eq!(observed(response).await, invalid_info_hash("abc"));
        assert_eq!(recorded(&calls), Vec::<String>::new());
    }

    #[tokio::test]
    async fn it_should_reject_an_invalid_info_hash_without_calling_the_service_when_removing_a_torrent() {
        let fake = FakeWhitelistPort::answering(succeeds);
        let calls = fake.call_log();

        let response = remove_torrent_from_whitelist_handler(state_with(fake), param("abc")).await;

        assert_eq!(observed(response).await, invalid_info_hash("abc"));
        assert_eq!(recorded(&calls), Vec::<String>::new());
    }

    #[tokio::test]
    async fn it_should_answer_ok_and_forward_the_parsed_info_hash_to_the_service_when_whitelisting_a_torrent() {
        let fake = FakeWhitelistPort::answering(succeeds);
        let calls = fake.call_log();

        let response = add_torrent_to_whitelist_handler(state_with(fake), param(INFO_HASH)).await;

        assert_eq!(observed(response).await, ok());
        assert_eq!(recorded(&calls), vec![format!("add_torrent({INFO_HASH})")]);
    }

    #[tokio::test]
    async fn it_should_answer_ok_and_forward_the_parsed_info_hash_to_the_service_when_removing_a_torrent() {
        let fake = FakeWhitelistPort::answering(succeeds);
        let calls = fake.call_log();

        let response = remove_torrent_from_whitelist_handler(state_with(fake), param(INFO_HASH)).await;

        assert_eq!(observed(response).await, ok());
        assert_eq!(recorded(&calls), vec![format!("remove_torrent({INFO_HASH})")]);
    }

    #[tokio::test]
    async fn it_should_answer_ok_when_the_whitelist_is_reloaded() {
        let fake = FakeWhitelistPort::answering(succeeds);
        let calls = fake.call_log();

        let response = reload_whitelist_handler(state_with(fake)).await;

        assert_eq!(observed(response).await, ok());
        assert_eq!(recorded(&calls), vec!["reload".to_string()]);
    }

    /// The second case documents a difference with the auth-key handlers: `add_auth_key_handler` maps a service-reported
    /// `AuthKeyError::DisabledByConfiguration` to the disabled `409`, while this handler answers a `500`. The service never
    /// returns that error today, so no client can see it. See the "Documented inconsistency" section of the plan
    /// `docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-handlers-tests.md`.
    #[rstest]
    #[case::a_database_error(database_error as Outcome, "failed to whitelist torrent: boom")]
    #[case::a_capability_disabled_in_the_service(
        (|| Err(WhitelistError::DisabledByConfiguration { capability: "listed" })) as Outcome,
        "failed to whitelist torrent: listed capability is disabled by configuration"
    )]
    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_whatever_the_error_when_whitelisting_a_torrent(
        #[case] outcome: Outcome,
        #[case] reason: &str,
    ) {
        let response = add_torrent_to_whitelist_handler(state_answering(outcome), param(INFO_HASH)).await;

        assert_eq!(observed(response).await, unhandled(reason));
    }

    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_when_removing_a_torrent_fails() {
        let response = remove_torrent_from_whitelist_handler(state_answering(database_error), param(INFO_HASH)).await;

        assert_eq!(
            observed(response).await,
            unhandled("failed to remove torrent from whitelist: boom")
        );
    }

    #[tokio::test]
    async fn it_should_answer_an_internal_server_error_when_reloading_the_whitelist_fails() {
        let response = reload_whitelist_handler(state_answering(database_error)).await;

        assert_eq!(observed(response).await, unhandled("failed to reload whitelist: boom"));
    }
}
