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

#[cfg(test)]
mod tests {
    // The drain timeout of a listener has no observable seam in a unit test: it matters only when a
    // handler is slower than the timeout, and the real handlers of these listeners are fast. So the
    // sites that must pass the configured value are checked at the source level: each file passes
    // `app_container.application_config.shutdown.event_listener_drain()` as the argument of every
    // listener it starts, and none passes the constant of the `events` package any more. What the
    // container holds is guarded by the container and accessor tests.
    #[test]
    fn it_should_not_leave_a_listener_site_on_the_default_drain_timeout() {
        // Arrange
        let listeners_started_by_each_site = [
            ("http_tracker_core.rs", include_str!("http_tracker_core.rs"), 1),
            ("torrent_repository.rs", include_str!("torrent_repository.rs"), 1),
            ("tracker_core.rs", include_str!("tracker_core.rs"), 2),
            ("udp_tracker_core.rs", include_str!("udp_tracker_core.rs"), 1),
            ("udp_tracker_server.rs", include_str!("udp_tracker_server.rs"), 2),
        ];

        // Act
        let sites_not_on_the_configured_value: Vec<&str> = listeners_started_by_each_site
            .iter()
            .filter(|(_, source, listeners)| {
                source.contains("DEFAULT_DRAIN_TIMEOUT")
                    || source
                        .matches("app_container.application_config.shutdown.event_listener_drain(),")
                        .count()
                        != *listeners
            })
            .map(|(name, _, _)| *name)
            .collect();

        // Assert
        assert!(
            sites_not_on_the_configured_value.is_empty(),
            "these listener sites do not pass the configured drain timeout to every listener: {sites_not_on_the_configured_value:?}"
        );
    }
}
