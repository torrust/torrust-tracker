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
