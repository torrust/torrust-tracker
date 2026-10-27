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

use std::time::Duration;

use torrust_tracker_configuration::v3_0_0::Configuration;

/// The time each event listener keeps admitting queued events after it is asked to stop
/// (`application.shutdown.event_listener_drain_in_secs`).
///
/// Every event listener site of the composition root passes this value, so that one setting
/// serves all of them.
#[must_use]
pub const fn event_listener_drain_timeout(config: &Configuration) -> Duration {
    config.application.shutdown.event_listener_drain()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::shutdown::ShutdownBudgetInSecs;

    use super::event_listener_drain_timeout;

    #[test]
    fn it_should_use_the_configured_event_listener_drain_budget_and_not_the_default() {
        // Arrange
        let mut configuration = Configuration::default();
        configuration.application.shutdown.event_listener_drain_in_secs =
            ShutdownBudgetInSecs::new(7).expect("7 s is within the supported range");

        // Act
        let drain_timeout = event_listener_drain_timeout(&configuration);

        // Assert
        assert_eq!(
            drain_timeout,
            Duration::from_secs(7),
            "the configured listener drain budget, not the default of 5 s"
        );
    }

    // The drain timeout of a listener has no observable seam in a unit test: it matters only when a
    // handler is slower than the timeout, and the real handlers of these listeners are fast. So the
    // sites that must pass the configured value are checked at the source level: each file passes
    // `event_listener_drain_timeout(config)` as the argument of every listener it starts, and none
    // passes the constant of the `events` package any more.
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
                    || source.matches("event_listener_drain_timeout(config),").count() != *listeners
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
