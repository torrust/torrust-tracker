//! Which components consume from and publish on each event bus.
//!
//! Bootstrap is the only place that knows both sides of every event bus: it
//! creates the services that publish and the listeners that consume. The
//! registry turns those facts into the stop order that
//! [`JobManager::spawn_with`](crate::bootstrap::jobs::manager::JobManager::spawn_with)
//! enforces, so that a producer completes before the listeners of the buses it
//! publishes on are asked to stop (loss window (b) of issue #2410). The job
//! manager knows nothing about events.
//!
//! Register every consumer of a bus before any producer of it. That is also
//! the start order a broadcast channel needs: an event reaches only the
//! receivers that already existed when it was sent.
//!
//! ADR: `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`
//!
//! issue: #2416

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::bootstrap::jobs::manager::ComponentId;

/// An event bus: one broadcast channel for one family of events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bus {
    /// Swarm coordination events.
    Swarm,
    /// HTTP tracker core events.
    HttpCore,
    /// UDP tracker core events.
    UdpCore,
    /// UDP tracker server events.
    UdpServer,
}

impl fmt::Display for Bus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Swarm => "swarm",
            Self::HttpCore => "http-core",
            Self::UdpCore => "udp-core",
            Self::UdpServer => "udp-server",
        })
    }
}

/// Errors returned by [`EventFlows`].
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(
        "A consumer of the {bus} event bus was registered after a producer of that bus. Register every listener of a bus before starting any component that publishes on it, so that no event is published before its listener subscribes and the stop order covers the listener."
    )]
    ConsumerAfterProducer { bus: Bus },
}

/// The registry of the consumers of each event bus, and of the buses that already have a producer.
#[derive(Debug, Default)]
pub struct EventFlows {
    consumers: BTreeMap<Bus, BTreeSet<ComponentId>>,
    buses_with_producers: BTreeSet<Bus>,
}

impl EventFlows {
    /// Registers a listener that consumes from `bus`.
    ///
    /// Register only listeners that started: a disabled listener is not a
    /// consumer and nothing waits for it.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConsumerAfterProducer`] when a producer of `bus` was
    /// already declared through [`Self::consumers_of`]: that producer would not
    /// stop before this consumer.
    pub fn add_consumer(&mut self, bus: Bus, consumer: ComponentId) -> Result<(), Error> {
        if self.buses_with_producers.contains(&bus) {
            return Err(Error::ConsumerAfterProducer { bus });
        }

        self.consumers.entry(bus).or_default().insert(consumer);

        Ok(())
    }

    /// Declares a producer that publishes on `buses` and returns the consumers
    /// it must stop before: every consumer of any of those buses, once.
    pub fn consumers_of(&mut self, buses: &[Bus]) -> Vec<ComponentId> {
        self.buses_with_producers.extend(buses.iter().copied());

        let consumers: BTreeSet<ComponentId> = buses
            .iter()
            .filter_map(|bus| self.consumers.get(bus))
            .flatten()
            .copied()
            .collect();

        consumers.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bus, Error, EventFlows};
    use crate::bootstrap::jobs::manager::{ComponentId, JobManager, Reservation};

    /// Reserves `count` components and returns their ids, in reservation order.
    fn component_ids(count: usize) -> Vec<ComponentId> {
        let mut manager = JobManager::new();
        let reservations: Vec<_> = (0..count)
            .map(|index| manager.reserve(format!("component-{index}")))
            .collect();
        reservations.iter().map(Reservation::id).collect()
    }

    #[test]
    fn it_should_reject_a_consumer_registered_after_a_producer_of_its_bus() {
        // Arrange
        // A producer that publishes on the swarm bus was already declared, while that bus had no consumer.
        let ids = component_ids(1);
        let mut flows = EventFlows::default();
        flows.consumers_of(&[Bus::Swarm]);

        // Act
        let result = flows.add_consumer(Bus::Swarm, ids[0]);

        // Assert
        assert_eq!(result, Err(Error::ConsumerAfterProducer { bus: Bus::Swarm }));
    }

    #[test]
    fn it_should_accept_a_consumer_of_a_bus_that_has_no_producer_yet() {
        // Arrange
        // A producer of the swarm bus was declared, and the consumer listens to another bus.
        let ids = component_ids(1);
        let mut flows = EventFlows::default();
        flows.consumers_of(&[Bus::Swarm]);

        // Act
        let result = flows.add_consumer(Bus::UdpServer, ids[0]);

        // Assert
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn it_should_return_the_consumers_of_every_bus_a_producer_publishes_on_without_duplicates() {
        // Arrange
        // Component 0 consumes the swarm and http-core buses, component 1 only swarm, component 2 only udp-core.
        let ids = component_ids(3);
        let mut flows = EventFlows::default();
        flows.add_consumer(Bus::Swarm, ids[0]).expect("no producer was declared yet");
        flows
            .add_consumer(Bus::HttpCore, ids[0])
            .expect("no producer was declared yet");
        flows.add_consumer(Bus::Swarm, ids[1]).expect("no producer was declared yet");
        flows
            .add_consumer(Bus::UdpCore, ids[2])
            .expect("no producer was declared yet");

        // Act
        let consumers = flows.consumers_of(&[Bus::HttpCore, Bus::Swarm]);

        // Assert
        assert_eq!(consumers, vec![ids[0], ids[1]]);
    }

    #[test]
    fn it_should_not_return_listeners_that_were_never_registered() {
        // Arrange
        // A disabled listener is never registered, so the swarm bus has no consumer.
        let mut flows = EventFlows::default();

        // Act
        let consumers = flows.consumers_of(&[Bus::Swarm]);

        // Assert
        assert_eq!(consumers, Vec::new());
    }
}
