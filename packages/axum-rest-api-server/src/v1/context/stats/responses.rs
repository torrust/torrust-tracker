//! API responses for the [`stats`](crate::v1::context::stats)
//! API context.
use axum::response::{IntoResponse, Json, Response};
use torrust_metrics::prometheus::PrometheusSerializable;
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};

/// `200` response that contains the [`LabeledStats`] resource as json.
#[must_use]
pub fn labeled_stats_response(stats: &LabeledStats) -> Response {
    Json(stats).into_response()
}

#[must_use]
pub fn labeled_metrics_response(stats: &LabeledStats) -> Response {
    stats.metrics.to_prometheus().into_response()
}

/// `200` response that contains the [`Stats`] resource as json.
#[must_use]
pub fn stats_response(stats: &Stats) -> Response {
    Json(stats).into_response()
}

/// `200` response that contains the [`Stats`] resource in Prometheus Text Exposition Format.
#[allow(deprecated)]
#[must_use]
pub fn metrics_response(stats: &Stats) -> Response {
    let mut lines = vec![];

    lines.push(format!("torrents {}", stats.torrents));
    lines.push(format!("seeders {}", stats.seeders));
    lines.push(format!("completed {}", stats.completed));
    lines.push(format!("completed_in_session {}", stats.completed_in_session));
    lines.push(format!("completed_persisted {}", stats.completed_persisted));
    lines.push(format!("completed_persisted_enabled {}", stats.completed_persisted_enabled));
    lines.push(format!("leechers {}", stats.leechers));

    // TCP
    lines.push(format!("tcp4_connections_handled {}", stats.tcp4_connections_handled));
    lines.push(format!("tcp4_announces_handled {}", stats.tcp4_announces_handled));
    lines.push(format!("tcp4_scrapes_handled {}", stats.tcp4_scrapes_handled));
    lines.push(format!("tcp6_connections_handled {}", stats.tcp6_connections_handled));
    lines.push(format!("tcp6_announces_handled {}", stats.tcp6_announces_handled));
    lines.push(format!("tcp6_scrapes_handled {}", stats.tcp6_scrapes_handled));

    // UDP
    lines.push(format!("udp_requests_discarded {}", stats.udp_requests_discarded));
    lines.push(format!("udp_requests_aborted {}", stats.udp_requests_aborted));
    lines.push(format!("udp_requests_banned {}", stats.udp_requests_banned));
    lines.push(format!("udp_banned_ips_total {}", stats.udp_banned_ips_total));
    lines.push(format!(
        "udp_avg_connect_processing_time_ns {}",
        stats.udp_avg_connect_processing_time_ns
    ));
    lines.push(format!(
        "udp_avg_announce_processing_time_ns {}",
        stats.udp_avg_announce_processing_time_ns
    ));
    lines.push(format!(
        "udp_avg_scrape_processing_time_ns {}",
        stats.udp_avg_scrape_processing_time_ns
    ));

    // UDPv4
    lines.push(format!("udp4_requests {}", stats.udp4_requests));
    lines.push(format!("udp4_connections_handled {}", stats.udp4_connections_handled));
    lines.push(format!("udp4_announces_handled {}", stats.udp4_announces_handled));
    lines.push(format!("udp4_scrapes_handled {}", stats.udp4_scrapes_handled));
    lines.push(format!("udp4_responses {}", stats.udp4_responses));
    lines.push(format!("udp4_errors_handled {}", stats.udp4_errors_handled));

    // UDPv6
    lines.push(format!("udp6_requests {}", stats.udp6_requests));
    lines.push(format!("udp6_connections_handled {}", stats.udp6_connections_handled));
    lines.push(format!("udp6_announces_handled {}", stats.udp6_announces_handled));
    lines.push(format!("udp6_scrapes_handled {}", stats.udp6_scrapes_handled));
    lines.push(format!("udp6_responses {}", stats.udp6_responses));
    lines.push(format!("udp6_errors_handled {}", stats.udp6_errors_handled));

    // Return the plain text response
    lines.join("\n").into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::{labeled_metrics_response, labeled_stats_response, metrics_response, stats_response};
    use crate::test_support::{Observed, a_labeled_stats, a_stats, observed};

    const JSON: &str = "application/json";
    const TEXT: &str = "text/plain; charset=utf-8";

    #[tokio::test]
    async fn it_should_answer_the_statistics_as_a_json_object() {
        let response = stats_response(&a_stats());

        assert_eq!(
            observed(response).await,
            Observed::new(
                StatusCode::OK,
                JSON,
                concat!(
                    r#"{"torrents":1,"seeders":2,"completed":3,"completed_in_session":4,"completed_persisted":5,"#,
                    r#""completed_persisted_enabled":true,"leechers":7,"#,
                    r#""tcp4_connections_handled":8,"tcp4_announces_handled":9,"tcp4_scrapes_handled":10,"#,
                    r#""tcp6_connections_handled":11,"tcp6_announces_handled":12,"tcp6_scrapes_handled":13,"#,
                    r#""udp_requests_discarded":14,"udp_requests_aborted":15,"udp_requests_banned":16,"#,
                    r#""udp_banned_ips_total":17,"udp_avg_connect_processing_time_ns":18,"#,
                    r#""udp_avg_announce_processing_time_ns":19,"udp_avg_scrape_processing_time_ns":20,"#,
                    r#""udp4_requests":21,"udp4_connections_handled":22,"udp4_announces_handled":23,"#,
                    r#""udp4_scrapes_handled":24,"udp4_responses":25,"udp4_errors_handled":26,"#,
                    r#""udp6_requests":27,"udp6_connections_handled":28,"udp6_announces_handled":29,"#,
                    r#""udp6_scrapes_handled":30,"udp6_responses":31,"udp6_errors_handled":32}"#
                )
            )
        );
    }

    /// TODO(#2637): the boolean line `completed_persisted_enabled true` is not a number, so a strict Prometheus parser may
    /// reject it, and the text has no trailing newline. Whether that is a bug is pending re-evaluation (see `lessons.md` in
    /// `docs/issues/open/2637-1347-add-tests-axum-rest-api-server/`); this test pins the current text and must be updated on
    /// purpose if it changes.
    #[tokio::test]
    async fn it_should_answer_the_statistics_as_one_name_and_value_line_per_statistic() {
        let response = metrics_response(&a_stats());

        assert_eq!(
            observed(response).await,
            Observed::new(
                StatusCode::OK,
                TEXT,
                concat!(
                    "torrents 1\n",
                    "seeders 2\n",
                    "completed 3\n",
                    "completed_in_session 4\n",
                    "completed_persisted 5\n",
                    "completed_persisted_enabled true\n",
                    "leechers 7\n",
                    "tcp4_connections_handled 8\n",
                    "tcp4_announces_handled 9\n",
                    "tcp4_scrapes_handled 10\n",
                    "tcp6_connections_handled 11\n",
                    "tcp6_announces_handled 12\n",
                    "tcp6_scrapes_handled 13\n",
                    "udp_requests_discarded 14\n",
                    "udp_requests_aborted 15\n",
                    "udp_requests_banned 16\n",
                    "udp_banned_ips_total 17\n",
                    "udp_avg_connect_processing_time_ns 18\n",
                    "udp_avg_announce_processing_time_ns 19\n",
                    "udp_avg_scrape_processing_time_ns 20\n",
                    "udp4_requests 21\n",
                    "udp4_connections_handled 22\n",
                    "udp4_announces_handled 23\n",
                    "udp4_scrapes_handled 24\n",
                    "udp4_responses 25\n",
                    "udp4_errors_handled 26\n",
                    "udp6_requests 27\n",
                    "udp6_connections_handled 28\n",
                    "udp6_announces_handled 29\n",
                    "udp6_scrapes_handled 30\n",
                    "udp6_responses 31\n",
                    "udp6_errors_handled 32"
                )
            )
        );
    }

    #[tokio::test]
    async fn it_should_answer_the_labeled_statistics_as_a_json_object_with_the_metric_collection() {
        let response = labeled_stats_response(&a_labeled_stats());

        assert_eq!(
            observed(response).await,
            Observed::new(
                StatusCode::OK,
                JSON,
                concat!(
                    r#"{"metrics":[{"type":"counter","name":"requests_total","unit":null,"description":null,"#,
                    r#""samples":[{"value":1,"recorded_at":"1970-01-01T00:00:01+00:00","labels":[]}]}]}"#
                )
            )
        );
    }

    #[tokio::test]
    async fn it_should_answer_the_labeled_statistics_as_prometheus_text() {
        let response = labeled_metrics_response(&a_labeled_stats());

        assert_eq!(
            observed(response).await,
            Observed::new(StatusCode::OK, TEXT, "\n# TYPE requests_total counter\nrequests_total 1")
        );
    }
}
