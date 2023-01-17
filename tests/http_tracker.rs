/// Integration tests for HTTP tracker server
///
/// cargo test `http_tracker_server` -- --nocapture
mod common;
mod http;

mod http_tracker_server {

    mod receiving_an_announce_request {
        use crate::common::http::Query;
        use crate::http::asserts::assert_internal_server_error;
        use crate::http::client::Client;
        use crate::http::server::start_default_http_tracker;

        #[tokio::test]
        async fn should_fail_when_the_request_is_empty() {
            let http_tracker_server = start_default_http_tracker().await;

            let response = Client::new(http_tracker_server.get_connection_info())
                .announce(Query::default())
                .await;

            assert_internal_server_error(response).await;
        }
    }

    mod receiving_an_scrape_request {
        use crate::common::http::Query;
        use crate::http::asserts::assert_internal_server_error;
        use crate::http::client::Client;
        use crate::http::server::start_default_http_tracker;

        #[tokio::test]
        async fn should_fail_when_the_request_is_empty() {
            let http_tracker_server = start_default_http_tracker().await;

            let response = Client::new(http_tracker_server.get_connection_info())
                .scrape(Query::default())
                .await;

            assert_internal_server_error(response).await;
        }
    }
}
