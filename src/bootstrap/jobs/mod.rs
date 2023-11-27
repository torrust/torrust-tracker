//! Application jobs launchers.
//!
//! The main application setup has only two main stages:
//!
//! 1. Setup the domain layer: the core tracker.
//! 2. Launch all the application services as concurrent jobs.
//!
//! This modules contains all the functions needed to start those jobs.
pub mod health_check_api;
pub mod http_tracker;
pub mod torrent_cleanup;
pub mod tracker_apis;
pub mod udp_tracker;
