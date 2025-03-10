mod given_that_the_token_is_only_provided_in_the_authentication_header {
    use hyper::header;
    use torrust_axum_rest_tracker_api_server::environment::Started;
    use torrust_rest_tracker_api_client::common::http::Query;
    use torrust_rest_tracker_api_client::connection_info::ConnectionInfo;
    use torrust_rest_tracker_api_client::v1::client::{
        headers_with_auth_token, headers_with_request_id, Client, AUTH_BEARER_TOKEN_HEADER_PREFIX,
    };
    use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
    use torrust_tracker_test_helpers::{configuration, logging};
    use uuid::Uuid;

    use crate::server::v1::asserts::assert_token_not_valid;

    #[tokio::test]
    async fn it_should_authenticate_requests_when_the_token_is_provided_in_the_authentication_header() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let token = env.get_connection_info().api_token.unwrap();

        let response = Client::new(env.get_connection_info())
            .unwrap()
            .get_request_with_query("stats", Query::default(), Some(headers_with_auth_token(&token)))
            .await;

        assert_eq!(response.status(), 200);

        env.stop().await;
    }

    #[tokio::test]
    async fn it_should_not_authenticate_requests_when_the_token_is_empty() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let request_id = Uuid::new_v4();

        let mut headers = headers_with_request_id(request_id);

        // Send the header with an empty token
        headers.insert(
            header::AUTHORIZATION,
            format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} ")
                .parse()
                .expect("the auth token is not a valid header value"),
        );

        let response = Client::new(env.get_connection_info())
            .unwrap()
            .get_request_with_query("stats", Query::default(), Some(headers))
            .await;

        assert_token_not_valid(response).await;

        assert!(
            logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
            "Expected logs to contain: ERROR ... API ... request_id={request_id}"
        );

        env.stop().await;
    }

    #[tokio::test]
    async fn it_should_not_authenticate_requests_when_the_token_is_invalid() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let request_id = Uuid::new_v4();

        let mut headers = headers_with_request_id(request_id);

        // Send the header with an empty token
        headers.insert(
            header::AUTHORIZATION,
            "Bearer INVALID TOKEN"
                .parse()
                .expect("the auth token is not a valid header value"),
        );

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        let response = Client::new(connection_info)
            .unwrap()
            .get_request_with_query("stats", Query::default(), Some(headers))
            .await;

        assert_token_not_valid(response).await;

        assert!(
            logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
            "Expected logs to contain: ERROR ... API ... request_id={request_id}"
        );

        env.stop().await;
    }
}
mod given_that_the_token_is_only_provided_in_the_query_param {

    use torrust_axum_rest_tracker_api_server::environment::Started;
    use torrust_rest_tracker_api_client::common::http::{Query, QueryParam};
    use torrust_rest_tracker_api_client::connection_info::ConnectionInfo;
    use torrust_rest_tracker_api_client::v1::client::{headers_with_request_id, Client, TOKEN_PARAM_NAME};
    use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
    use torrust_tracker_test_helpers::{configuration, logging};
    use uuid::Uuid;

    use crate::server::v1::asserts::assert_token_not_valid;

    #[tokio::test]
    async fn it_should_authenticate_requests_when_the_token_is_provided_as_a_query_param() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let token = env.get_connection_info().api_token.unwrap();

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        let response = Client::new(connection_info)
            .unwrap()
            .get_request_with_query(
                "stats",
                Query::params([QueryParam::new(TOKEN_PARAM_NAME, &token)].to_vec()),
                None,
            )
            .await;

        assert_eq!(response.status(), 200);

        env.stop().await;
    }

    #[tokio::test]
    async fn it_should_not_authenticate_requests_when_the_token_is_empty() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let request_id = Uuid::new_v4();

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        let response = Client::new(connection_info)
            .unwrap()
            .get_request_with_query(
                "stats",
                Query::params([QueryParam::new(TOKEN_PARAM_NAME, "")].to_vec()),
                Some(headers_with_request_id(request_id)),
            )
            .await;

        assert_token_not_valid(response).await;

        assert!(
            logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
            "Expected logs to contain: ERROR ... API ... request_id={request_id}"
        );

        env.stop().await;
    }

    #[tokio::test]
    async fn it_should_not_authenticate_requests_when_the_token_is_invalid() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let request_id = Uuid::new_v4();

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        let response = Client::new(connection_info)
            .unwrap()
            .get_request_with_query(
                "stats",
                Query::params([QueryParam::new(TOKEN_PARAM_NAME, "INVALID TOKEN")].to_vec()),
                Some(headers_with_request_id(request_id)),
            )
            .await;

        assert_token_not_valid(response).await;

        assert!(
            logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
            "Expected logs to contain: ERROR ... API ... request_id={request_id}"
        );

        env.stop().await;
    }

    #[tokio::test]
    async fn it_should_allow_the_token_query_param_to_be_at_any_position_in_the_url_query() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let token = env.get_connection_info().api_token.unwrap();

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        // At the beginning of the query component
        let response = Client::new(connection_info)
            .unwrap()
            .get_request(&format!("torrents?token={token}&limit=1"))
            .await;

        assert_eq!(response.status(), 200);

        // At the end of the query component
        let response = Client::new(env.get_connection_info())
            .unwrap()
            .get_request(&format!("torrents?limit=1&token={token}"))
            .await;

        assert_eq!(response.status(), 200);

        env.stop().await;
    }
}

mod given_that_not_token_is_provided {

    use torrust_axum_rest_tracker_api_server::environment::Started;
    use torrust_rest_tracker_api_client::common::http::Query;
    use torrust_rest_tracker_api_client::connection_info::ConnectionInfo;
    use torrust_rest_tracker_api_client::v1::client::{headers_with_request_id, Client};
    use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
    use torrust_tracker_test_helpers::{configuration, logging};
    use uuid::Uuid;

    use crate::server::v1::asserts::assert_unauthorized;

    #[tokio::test]
    async fn it_should_not_authenticate_requests_when_the_token_is_missing() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let request_id = Uuid::new_v4();

        let connection_info = ConnectionInfo::anonymous(env.get_connection_info().origin);

        let response = Client::new(connection_info)
            .unwrap()
            .get_request_with_query("stats", Query::default(), Some(headers_with_request_id(request_id)))
            .await;

        assert_unauthorized(response).await;

        assert!(
            logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
            "Expected logs to contain: ERROR ... API ... request_id={request_id}"
        );

        env.stop().await;
    }
}

mod given_that_token_is_provided_via_get_param_and_authentication_header {
    use torrust_axum_rest_tracker_api_server::environment::Started;
    use torrust_rest_tracker_api_client::common::http::{Query, QueryParam};
    use torrust_rest_tracker_api_client::v1::client::{headers_with_auth_token, Client, TOKEN_PARAM_NAME};
    use torrust_tracker_test_helpers::{configuration, logging};

    #[tokio::test]
    async fn it_should_authenticate_requests_using_the_token_provided_in_the_authentication_header() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;

        let authorized_token = env.get_connection_info().api_token.unwrap();

        let non_authorized_token = "NonAuthorizedToken";

        let response = Client::new(env.get_connection_info())
            .unwrap()
            .get_request_with_query(
                "stats",
                Query::params([QueryParam::new(TOKEN_PARAM_NAME, non_authorized_token)].to_vec()),
                Some(headers_with_auth_token(&authorized_token)),
            )
            .await;

        // The token provided in the query param should be ignored and the token
        // in the authentication header should be used.
        assert_eq!(response.status(), 200);

        env.stop().await;
    }
}
