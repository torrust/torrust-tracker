//! API handlers for the [`stats`](crate::v1::context::stats)
//! API context.
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Response;
use serde::Deserialize;
use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;

use super::responses::{labeled_metrics_response, labeled_stats_response, metrics_response, stats_response};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    #[default]
    Json,
    Prometheus,
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    /// The [`Format`] of the stats.
    #[serde(default)]
    pub format: Option<Format>,
}

/// It handles the request to get the tracker global metrics.
pub async fn get_stats_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let stats = stats_service.get_stats().await;

    params.0.format.map_or_else(
        || stats_response(&stats),
        |format| match format {
            Format::Json => stats_response(&stats),
            Format::Prometheus => metrics_response(&stats),
        },
    )
}

/// It handles the request to get the tracker extendable metrics.
pub async fn get_metrics_handler(State(stats_service): State<Arc<StatsApiService>>, params: Query<QueryParams>) -> Response {
    let labeled_stats = stats_service.get_labeled_stats().await;

    params.0.format.map_or_else(
        || labeled_stats_response(&labeled_stats),
        |format| match format {
            Format::Json => labeled_stats_response(&labeled_stats),
            Format::Prometheus => labeled_metrics_response(&labeled_stats),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::extract::{Query, State};
    use axum::http::{StatusCode, Uri};
    use rstest::rstest;
    use torrust_tracker_rest_api_application::v1::ports::stats::StatsQueryPort;
    use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;
    use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};

    use super::{QueryParams, get_metrics_handler, get_stats_handler};
    use crate::test_support::{a_labeled_stats, a_stats, observed};

    type CallLog = Arc<Mutex<Vec<Call>>>;

    const JSON: &str = "application/json";
    const TEXT: &str = "text/plain; charset=utf-8";

    /// A call that the fake port received.
    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Call {
        Stats,
        LabeledStats,
    }

    /// A `StatsQueryPort` that answers the fixed statistics and records each call.
    struct FakeStatsQueryPort {
        calls: CallLog,
    }

    #[async_trait]
    impl StatsQueryPort for FakeStatsQueryPort {
        async fn get_stats(&self) -> Stats {
            self.calls.lock().expect("the call log is not poisoned").push(Call::Stats);
            a_stats()
        }

        async fn get_labeled_stats(&self) -> LabeledStats {
            self.calls
                .lock()
                .expect("the call log is not poisoned")
                .push(Call::LabeledStats);
            a_labeled_stats()
        }
    }

    fn fake_with_log() -> (State<Arc<StatsApiService>>, CallLog) {
        let calls: CallLog = Arc::new(Mutex::new(Vec::new()));
        let service = StatsApiService::new(Box::new(FakeStatsQueryPort { calls: calls.clone() }));

        (State(Arc::new(service)), calls)
    }

    fn uri(query_string: &str) -> Uri {
        format!("/api/v1/stats?{query_string}")
            .parse()
            .expect("the test URI is valid")
    }

    fn query(query_string: &str) -> Query<QueryParams> {
        Query::try_from_uri(&uri(query_string)).expect("the query string is valid")
    }

    fn recorded(calls: &CallLog) -> Vec<Call> {
        calls.lock().expect("the call log is not poisoned").clone()
    }

    #[rstest]
    #[case::no_format("", JSON, r#"{"torrents":1,"#)]
    #[case::json("format=json", JSON, r#"{"torrents":1,"#)]
    #[case::prometheus("format=prometheus", TEXT, "torrents 1\nseeders 2\n")]
    #[tokio::test]
    async fn it_should_answer_the_statistics_in_the_requested_format(
        #[case] query_string: &str,
        #[case] content_type: &str,
        #[case] body_start: &str,
    ) {
        let (state, calls) = fake_with_log();

        let response = observed(get_stats_handler(state, query(query_string)).await).await;

        assert_eq!(
            (
                response.status,
                response.content_type.as_deref(),
                response.body.starts_with(body_start),
                recorded(&calls)
            ),
            (StatusCode::OK, Some(content_type), true, vec![Call::Stats]),
            "the body was: {}",
            response.body
        );
    }

    #[rstest]
    #[case::no_format("", JSON, r#"{"metrics":["#)]
    #[case::json("format=json", JSON, r#"{"metrics":["#)]
    #[case::prometheus("format=prometheus", TEXT, "requests_total 1")]
    #[tokio::test]
    async fn it_should_answer_the_labeled_metrics_in_the_requested_format(
        #[case] query_string: &str,
        #[case] content_type: &str,
        #[case] body_part: &str,
    ) {
        let (state, calls) = fake_with_log();

        let response = observed(get_metrics_handler(state, query(query_string)).await).await;

        assert_eq!(
            (
                response.status,
                response.content_type.as_deref(),
                response.body.contains(body_part),
                recorded(&calls)
            ),
            (StatusCode::OK, Some(content_type), true, vec![Call::LabeledStats]),
            "the body was: {}",
            response.body
        );
    }

    #[rstest]
    #[case::no_format("", "None")]
    #[case::json("format=json", "Some(Json)")]
    #[case::prometheus("format=prometheus", "Some(Prometheus)")]
    fn it_should_read_the_format_in_lowercase(#[case] query_string: &str, #[case] expected: &str) {
        let params = query(query_string).0;

        assert_eq!(format!("{:?}", params.format), expected);
    }

    #[rstest]
    #[case::capitals("format=JSON")]
    #[case::an_unknown_format("format=xml")]
    #[case::an_empty_format("format=")]
    fn it_should_reject_a_format_that_is_not_json_or_prometheus(#[case] query_string: &str) {
        let result = Query::<QueryParams>::try_from_uri(&uri(query_string));

        assert!(result.is_err(), "the format was accepted: {result:?}");
    }
}
