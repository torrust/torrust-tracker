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

/// `200` response that contains the [`Stats`] resource as `name value` lines that Prometheus can scrape.
///
/// Every value is a number (`completed_persisted_enabled` is `1` or `0`) and the body ends with a line feed. The text has no
/// `# HELP` or `# TYPE` lines and no labels; use `GET /metrics?format=prometheus` for the full exposition format.
#[allow(deprecated)]
#[must_use]
pub fn metrics_response(stats: &Stats) -> Response {
    let mut lines = vec![];

    lines.push(format!("torrents {}", stats.torrents));
    lines.push(format!("seeders {}", stats.seeders));
    lines.push(format!("completed {}", stats.completed));
    lines.push(format!("completed_in_session {}", stats.completed_in_session));
    lines.push(format!("completed_persisted {}", stats.completed_persisted));
    // Prometheus needs a numeric sample value, so the boolean is printed as `1` or `0`.
    lines.push(format!(
        "completed_persisted_enabled {}",
        u8::from(stats.completed_persisted_enabled)
    ));
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

    // Return the plain text response, terminated by a line feed as the Prometheus text format requires.
    format!("{}\n", lines.join("\n")).into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::Stats;

    use super::{labeled_metrics_response, labeled_stats_response, metrics_response, stats_response};
    use crate::test_support::{Observed, a_labeled_stats, a_stats, observed};

    const JSON: &str = "application/json";
    const TEXT: &str = "text/plain; charset=utf-8";

    const STATS_TEXT: &str = concat!(
        "# HELP torrents Total number of torrents.\n",
        "# TYPE torrents gauge\n",
        "torrents 1\n",
        "# HELP seeders Total number of seeders for all torrents.\n",
        "# TYPE seeders gauge\n",
        "seeders 2\n",
        "# HELP completed Deprecated ambiguous completed-download total. Use completed_in_session and completed_persisted instead.\n",
        "# TYPE completed counter\n",
        "completed 3\n",
        "# HELP completed_in_session Completed downloads observed since the current tracker process started.\n",
        "# TYPE completed_in_session counter\n",
        "completed_in_session 4\n",
        "# HELP completed_persisted Completed downloads restored from and maintained in persistent storage. Zero when persistence is disabled.\n",
        "# TYPE completed_persisted counter\n",
        "completed_persisted 5\n",
        "# HELP completed_persisted_enabled Whether completed_persisted is backed by persistent storage (1 enabled, 0 disabled).\n",
        "# TYPE completed_persisted_enabled gauge\n",
        "completed_persisted_enabled 1\n",
        "# HELP leechers Total number of leechers for all torrents.\n",
        "# TYPE leechers gauge\n",
        "leechers 7\n",
        "# HELP tcp4_connections_handled Total number of TCP (HTTP tracker) connections from IPv4 peers.\n",
        "# TYPE tcp4_connections_handled counter\n",
        "tcp4_connections_handled 8\n",
        "# HELP tcp4_announces_handled Total number of TCP (HTTP tracker) announce requests from IPv4 peers.\n",
        "# TYPE tcp4_announces_handled counter\n",
        "tcp4_announces_handled 9\n",
        "# HELP tcp4_scrapes_handled Total number of TCP (HTTP tracker) scrape requests from IPv4 peers.\n",
        "# TYPE tcp4_scrapes_handled counter\n",
        "tcp4_scrapes_handled 10\n",
        "# HELP tcp6_connections_handled Total number of TCP (HTTP tracker) connections from IPv6 peers.\n",
        "# TYPE tcp6_connections_handled counter\n",
        "tcp6_connections_handled 11\n",
        "# HELP tcp6_announces_handled Total number of TCP (HTTP tracker) announce requests from IPv6 peers.\n",
        "# TYPE tcp6_announces_handled counter\n",
        "tcp6_announces_handled 12\n",
        "# HELP tcp6_scrapes_handled Total number of TCP (HTTP tracker) scrape requests from IPv6 peers.\n",
        "# TYPE tcp6_scrapes_handled counter\n",
        "tcp6_scrapes_handled 13\n",
        "# HELP udp_requests_discarded Total number of UDP (UDP tracker) requests discarded before processing (e.g. client source port is 0).\n",
        "# TYPE udp_requests_discarded counter\n",
        "udp_requests_discarded 14\n",
        "# HELP udp_requests_aborted Total number of UDP (UDP tracker) requests aborted.\n",
        "# TYPE udp_requests_aborted counter\n",
        "udp_requests_aborted 15\n",
        "# HELP udp_requests_banned Total number of UDP (UDP tracker) requests banned.\n",
        "# TYPE udp_requests_banned counter\n",
        "udp_requests_banned 16\n",
        "# HELP udp_banned_ips_total Number of IPs whose UDP connection ID errors exceed the limit (banned when connection ID validation is strict, would-be banned when it is disabled).\n",
        "# TYPE udp_banned_ips_total gauge\n",
        "udp_banned_ips_total 17\n",
        "# HELP udp_avg_connect_processing_time_ns Average rounded time spent processing UDP connect requests, in nanoseconds.\n",
        "# TYPE udp_avg_connect_processing_time_ns gauge\n",
        "udp_avg_connect_processing_time_ns 18\n",
        "# HELP udp_avg_announce_processing_time_ns Average rounded time spent processing UDP announce requests, in nanoseconds.\n",
        "# TYPE udp_avg_announce_processing_time_ns gauge\n",
        "udp_avg_announce_processing_time_ns 19\n",
        "# HELP udp_avg_scrape_processing_time_ns Average rounded time spent processing UDP scrape requests, in nanoseconds.\n",
        "# TYPE udp_avg_scrape_processing_time_ns gauge\n",
        "udp_avg_scrape_processing_time_ns 20\n",
        "# HELP udp4_requests Total number of UDP (UDP tracker) requests from IPv4 peers.\n",
        "# TYPE udp4_requests counter\n",
        "udp4_requests 21\n",
        "# HELP udp4_connections_handled Total number of UDP (UDP tracker) connections from IPv4 peers.\n",
        "# TYPE udp4_connections_handled counter\n",
        "udp4_connections_handled 22\n",
        "# HELP udp4_announces_handled Total number of UDP (UDP tracker) announce requests from IPv4 peers.\n",
        "# TYPE udp4_announces_handled counter\n",
        "udp4_announces_handled 23\n",
        "# HELP udp4_scrapes_handled Total number of UDP (UDP tracker) scrape requests from IPv4 peers.\n",
        "# TYPE udp4_scrapes_handled counter\n",
        "udp4_scrapes_handled 24\n",
        "# HELP udp4_responses Total number of UDP (UDP tracker) responses to IPv4 peers.\n",
        "# TYPE udp4_responses counter\n",
        "udp4_responses 25\n",
        "# HELP udp4_errors_handled Total number of UDP (UDP tracker) errors handled for IPv4 peers.\n",
        "# TYPE udp4_errors_handled counter\n",
        "udp4_errors_handled 26\n",
        "# HELP udp6_requests Total number of UDP (UDP tracker) requests from IPv6 peers.\n",
        "# TYPE udp6_requests counter\n",
        "udp6_requests 27\n",
        "# HELP udp6_connections_handled Total number of UDP (UDP tracker) connection requests from IPv6 peers.\n",
        "# TYPE udp6_connections_handled counter\n",
        "udp6_connections_handled 28\n",
        "# HELP udp6_announces_handled Total number of UDP (UDP tracker) announce requests from IPv6 peers.\n",
        "# TYPE udp6_announces_handled counter\n",
        "udp6_announces_handled 29\n",
        "# HELP udp6_scrapes_handled Total number of UDP (UDP tracker) scrape requests from IPv6 peers.\n",
        "# TYPE udp6_scrapes_handled counter\n",
        "udp6_scrapes_handled 30\n",
        "# HELP udp6_responses Total number of UDP (UDP tracker) responses to IPv6 peers.\n",
        "# TYPE udp6_responses counter\n",
        "udp6_responses 31\n",
        "# HELP udp6_errors_handled Total number of UDP (UDP tracker) errors handled for IPv6 peers.\n",
        "# TYPE udp6_errors_handled counter\n",
        "udp6_errors_handled 32\n"
    );

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

    #[tokio::test]
    async fn it_should_answer_the_statistics_as_prometheus_text_with_help_and_type_for_each_statistic() {
        let response = metrics_response(&a_stats());

        assert_eq!(observed(response).await, Observed::new(StatusCode::OK, TEXT, STATS_TEXT));
    }

    #[tokio::test]
    async fn it_should_precede_every_sample_with_one_help_and_one_type_line_for_the_same_name() {
        let body = observed(metrics_response(&a_stats())).await.body;

        assert!(body.ends_with('\n'), "{body}");

        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(lines.len(), 32 * 3, "{body}");

        for family in lines.chunks(3) {
            let sample_name = family[2].split(' ').next().unwrap();

            assert!(family[0].starts_with(&format!("# HELP {sample_name} ")), "{family:?}");
            assert!(
                family[1] == format!("# TYPE {sample_name} gauge") || family[1] == format!("# TYPE {sample_name} counter"),
                "{family:?}"
            );
        }
    }

    #[tokio::test]
    async fn it_should_print_the_persistence_flag_as_zero_when_persisted_completed_statistics_are_disabled() {
        let response = metrics_response(&Stats {
            completed_persisted_enabled: false,
            ..a_stats()
        });

        let body = observed(response).await.body;

        assert!(body.contains("\ncompleted_persisted_enabled 0\n"), "{body}");
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
