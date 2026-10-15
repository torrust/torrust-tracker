//! UDP tracker core statistics.
//!
//! Tracker metrics are collected using a Publisher-Subscribe pattern.
//!
//! [`UdpTrackerCoreServices::initialize_from`](crate::container::UdpTrackerCoreServices::initialize_from)
//! builds the two structs involved:
//!
//! - An event [`Sender`](crate::event::sender::Sender), taken from the services' event bus
//! - A statistics [`Repository`](repository::Repository)
//!
//! The statistics repository is responsible for storing the metrics in memory.
//! The statistics event sender allows sending events related to metrics.
//! There is an event listener that is receiving all the events and processing them with an event handler.
//! Then, the event handler updates the metrics depending on the received event.
//!
//! For example, if you send the event [`Event::UdpConnect`](crate::event::Event::UdpConnect):
//!
//! ```rust,ignore
//! let result = event_sender.send(Event::UdpConnect { connection }).await;
//! ```
//!
//! Eventually the requests-received counter is increased for that request kind and server socket
//! family, so [`Metrics::udp4_connections_handled`] reports one more connection on IPv4 server
//! sockets.
pub mod event;
pub mod metrics;
pub mod repository;

use metrics::Metrics;
use torrust_metrics::metric::description::MetricDescription;
use torrust_metrics::metric_name;
use torrust_metrics::unit::Unit;

const UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL: &str = "udp_tracker_core_requests_received_total";

#[must_use]
pub fn describe_metrics() -> Metrics {
    let mut metrics = Metrics::default();

    metrics.metric_collection.describe_counter(
        &metric_name!(UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL),
        Some(Unit::Count),
        Some(MetricDescription::new("Total number of UDP requests received")),
    );

    metrics
}

#[cfg(test)]
mod tests {
    use torrust_metrics::metric_name;

    use super::describe_metrics;

    #[test]
    fn it_should_declare_the_requests_received_counter_under_its_exported_name() {
        // Act
        let metrics = describe_metrics();

        // Assert
        assert!(
            metrics
                .metric_collection
                .contains_counter(&metric_name!("udp_tracker_core_requests_received_total")),
            "the counter should be exported as `udp_tracker_core_requests_received_total`"
        );
    }
}
