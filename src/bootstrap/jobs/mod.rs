//! Application jobs launchers.
//!
//! The main application setup has only two main stages:
//!
//! 1. Setup the domain layer: the core tracker.
//! 2. Launch all the application services as concurrent jobs.
//!
//! This modules contains all the functions needed to start those jobs.
pub mod activity_metrics_updater;
pub mod health_check_api;
pub mod http_tracker;
pub mod http_tracker_core;
pub mod manager;
pub mod torrent_cleanup;
pub mod torrent_repository;
pub mod tracker_apis;
pub mod tracker_core;
pub mod udp_tracker;
pub mod udp_tracker_core;
pub mod udp_tracker_server;
