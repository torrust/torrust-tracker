//! Tracker statistics API context.
//!
//! This context serves two endpoints with different purposes (see the ADR "Define the stats and metrics endpoints and require
//! HELP and TYPE in Prometheus output"):
//!
//! - `GET /stats` returns the aggregated, unlabelled global statistics as a fixed, flat resource. It is **not deprecated**;
//!   only the `completed` field is.
//! - `GET /metrics` returns the extendable labelled metrics of all subsystems. New metrics and labels belong there.
//!
//! The tracker collects statistics about the number of torrents, seeders,
//! leechers, completed downloads, and the number of requests handled. The
//! legacy `completed` field is deprecated; use `completed_in_session` and
//! `completed_persisted` with `completed_persisted_enabled` instead.
//!
//! # Endpoints
//!
//! - [Get tracker statistics](#get-tracker-statistics)
//!
//! # Get tracker statistics
//!
//! `GET /stats`
//!
//! Returns the tracker statistics.
//!
//! **Example request**
//!
//! ```bash
//! curl "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "torrents": 0,
//!     "seeders": 0,
//!     "completed": 0,
//!     "completed_in_session": 0,
//!     "completed_persisted": 0,
//!     "completed_persisted_enabled": false,
//!     "leechers": 0,
//!     "tcp4_connections_handled": 0,
//!     "tcp4_announces_handled": 0,
//!     "tcp4_scrapes_handled": 0,
//!     "tcp6_connections_handled": 0,
//!     "tcp6_announces_handled": 0,
//!     "tcp6_scrapes_handled": 0,
//!     "udp4_connections_handled": 0,
//!     "udp4_announces_handled": 0,
//!     "udp4_scrapes_handled": 0,
//!     "udp6_connections_handled": 0,
//!     "udp6_announces_handled": 0,
//!     "udp6_scrapes_handled": 0
//!   }
//! ```
//!
//! **Prometheus text**
//!
//! `GET /stats?format=prometheus` returns the same statistics as one `name value` line per statistic, in the order of the JSON
//! fields, that a Prometheus server can scrape:
//!
//! ```bash
//! curl "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken&format=prometheus"
//! ```
//!
//! ```text
//! torrents 0
//! seeders 0
//! completed 0
//! completed_in_session 0
//! completed_persisted 0
//! completed_persisted_enabled 0
//! leechers 0
//! ...
//! udp6_errors_handled 0
//! ```
//!
//! Every value is a number: the boolean `completed_persisted_enabled` is printed as `1` (enabled) or `0` (disabled), and the
//! body ends with a line feed. The text has no `# HELP` or `# TYPE` lines and no labels. That is a known gap, not a
//! design choice: the ADR requires every Prometheus text of the tracker to carry `# HELP` and `# TYPE`, and adding them here
//! is a tracked follow-up. The full Prometheus exposition format with metric types, descriptions and labels is
//! `GET /metrics?format=prometheus`.
//!
//! **Resource**
//!
//! Refer to the API [`Stats`](torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::Stats)
//! resource for more information about the response attributes.
pub mod handlers;
pub mod responses;
pub mod routes;
