use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
#[cfg(test)]
use mockall::{automock, predicate::*};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, RwLock, RwLockReadGuard};

const CHANNEL_BUFFER_SIZE: usize = 65_535;

#[derive(Debug, PartialEq)]
pub enum TrackerStatisticsEvent {
    Tcp4Announce,
    Tcp4Scrape,
    Tcp6Announce,
    Tcp6Scrape,
    Udp4Connect,
    Udp4Announce,
    Udp4Scrape,
    Udp6Connect,
    Udp6Announce,
    Udp6Scrape,
}

#[derive(Debug)]
pub struct TrackerStatistics {
    pub tcp4_connections_handled: u64,
    pub tcp4_announces_handled: u64,
    pub tcp4_scrapes_handled: u64,
    pub tcp6_connections_handled: u64,
    pub tcp6_announces_handled: u64,
    pub tcp6_scrapes_handled: u64,
    pub udp4_connections_handled: u64,
    pub udp4_announces_handled: u64,
    pub udp4_scrapes_handled: u64,
    pub udp6_connections_handled: u64,
    pub udp6_announces_handled: u64,
    pub udp6_scrapes_handled: u64,
}

impl Default for TrackerStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackerStatistics {
    pub fn new() -> Self {
        Self {
            tcp4_connections_handled: 0,
            tcp4_announces_handled: 0,
            tcp4_scrapes_handled: 0,
            tcp6_connections_handled: 0,
            tcp6_announces_handled: 0,
            tcp6_scrapes_handled: 0,
            udp4_connections_handled: 0,
            udp4_announces_handled: 0,
            udp4_scrapes_handled: 0,
            udp6_connections_handled: 0,
            udp6_announces_handled: 0,
            udp6_scrapes_handled: 0,
        }
    }
}

pub struct StatsTracker {
    pub stats_repository: StatsRepository,
}

impl Default for StatsTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsTracker {
    pub fn new() -> Self {
        Self {
            stats_repository: StatsRepository::new(),
        }
    }

    pub fn new_active_instance() -> (Box<dyn TrackerStatisticsEventSender>, StatsRepository) {
        let mut stats_tracker = Self::new();

        let stats_event_sender = stats_tracker.run_event_listener();

        (stats_event_sender, stats_tracker.stats_repository)
    }

    pub fn run_event_listener(&mut self) -> Box<dyn TrackerStatisticsEventSender> {
        let (sender, receiver) = mpsc::channel::<TrackerStatisticsEvent>(CHANNEL_BUFFER_SIZE);

        let stats_repository = self.stats_repository.clone();

        tokio::spawn(async move { event_listener(receiver, stats_repository).await });

        Box::new(StatsEventSender { sender })
    }
}

async fn event_listener(mut receiver: Receiver<TrackerStatisticsEvent>, stats_repository: StatsRepository) {
    while let Some(event) = receiver.recv().await {
        event_handler(event, &stats_repository).await;
    }
}

async fn event_handler(event: TrackerStatisticsEvent, stats_repository: &StatsRepository) {
    match event {
        // TCP4
        TrackerStatisticsEvent::Tcp4Announce => {
            stats_repository.increase_tcp4_announces().await;
            stats_repository.increase_tcp4_connections().await;
        }
        TrackerStatisticsEvent::Tcp4Scrape => {
            stats_repository.increase_tcp4_scrapes().await;
            stats_repository.increase_tcp4_connections().await;
        }

        // TCP6
        TrackerStatisticsEvent::Tcp6Announce => {
            stats_repository.increase_tcp6_announces().await;
            stats_repository.increase_tcp6_connections().await;
        }
        TrackerStatisticsEvent::Tcp6Scrape => {
            stats_repository.increase_tcp6_scrapes().await;
            stats_repository.increase_tcp6_connections().await;
        }

        // UDP4
        TrackerStatisticsEvent::Udp4Connect => {
            stats_repository.increase_udp4_connections().await;
        }
        TrackerStatisticsEvent::Udp4Announce => {
            stats_repository.increase_udp4_announces().await;
        }
        TrackerStatisticsEvent::Udp4Scrape => {
            stats_repository.increase_udp4_scrapes().await;
        }

        // UDP6
        TrackerStatisticsEvent::Udp6Connect => {
            stats_repository.increase_udp6_connections().await;
        }
        TrackerStatisticsEvent::Udp6Announce => {
            stats_repository.increase_udp6_announces().await;
        }
        TrackerStatisticsEvent::Udp6Scrape => {
            stats_repository.increase_udp6_scrapes().await;
        }
    }

    debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TrackerStatisticsEventSender: Sync + Send {
    async fn send_event(&self, event: TrackerStatisticsEvent) -> Option<Result<(), SendError<TrackerStatisticsEvent>>>;
}

pub struct StatsEventSender {
    sender: Sender<TrackerStatisticsEvent>,
}

#[async_trait]
impl TrackerStatisticsEventSender for StatsEventSender {
    async fn send_event(&self, event: TrackerStatisticsEvent) -> Option<Result<(), SendError<TrackerStatisticsEvent>>> {
        Some(self.sender.send(event).await)
    }
}

#[derive(Clone)]
pub struct StatsRepository {
    pub stats: Arc<RwLock<TrackerStatistics>>,
}

impl Default for StatsRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsRepository {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(TrackerStatistics::new())),
        }
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStatistics> {
        self.stats.read().await
    }

    pub async fn increase_tcp4_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp4_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_tcp4_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp4_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_tcp4_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp4_scrapes_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_tcp6_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp6_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_tcp6_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp6_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_tcp6_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.tcp6_scrapes_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp4_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp4_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp4_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp4_scrapes_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_connections(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_connections_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_announces(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_announces_handled += 1;
        drop(stats_lock);
    }

    pub async fn increase_udp6_scrapes(&self) {
        let mut stats_lock = self.stats.write().await;
        stats_lock.udp6_scrapes_handled += 1;
        drop(stats_lock);
    }
}

#[cfg(test)]
mod tests {

    mod stats_tracker {
        use crate::statistics::{StatsTracker, TrackerStatistics, TrackerStatisticsEvent};

        #[tokio::test]
        async fn should_contain_the_tracker_statistics() {
            let stats_tracker = StatsTracker::new();

            let stats = stats_tracker.stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_announces_handled, TrackerStatistics::new().tcp4_announces_handled);
        }

        #[tokio::test]
        async fn should_create_an_event_sender_to_send_statistical_events() {
            let mut stats_tracker = StatsTracker::new();

            let event_sender = stats_tracker.run_event_listener();

            let result = event_sender.send_event(TrackerStatisticsEvent::Udp4Connect).await;

            assert!(result.is_some());
        }
    }

    mod event_handler {
        use crate::statistics::{event_handler, StatsRepository, TrackerStatisticsEvent};

        #[tokio::test]
        async fn should_increase_the_tcp4_announces_counter_when_it_receives_a_tcp4_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_connections_counter_when_it_receives_a_tcp4_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_scrapes_counter_when_it_receives_a_tcp4_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_connections_counter_when_it_receives_a_tcp4_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_announces_counter_when_it_receives_a_tcp6_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_connections_counter_when_it_receives_a_tcp6_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_scrapes_counter_when_it_receives_a_tcp6_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_connections_counter_when_it_receives_a_tcp6_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Tcp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_connections_counter_when_it_receives_a_udp4_connect_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp4Connect, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_announces_counter_when_it_receives_a_udp4_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_scrapes_counter_when_it_receives_a_udp4_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_connections_counter_when_it_receives_a_udp6_connect_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp6Connect, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_announces_counter_when_it_receives_a_udp6_announce_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_scrapes_counter_when_it_receives_a_udp6_scrape_event() {
            let stats_repository = StatsRepository::new();

            event_handler(TrackerStatisticsEvent::Udp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_scrapes_handled, 1);
        }
    }
}
