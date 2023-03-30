//! Tracker statistics API context.
//!
//! The tracker collects statistics about the number of torrents, seeders,
//! leechers, completed downloads, and the number of requests handled.
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
//! **Resource**
//!
//! Refer to the API [`Stats`](crate::servers::apis::v1::context::stats::resources::Stats)
//! resource for more information about the response attributes.
pub mod handlers;
pub mod resources;
pub mod responses;
pub mod routes;
