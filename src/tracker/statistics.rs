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
    pub fn new_active_instance() -> (Box<dyn TrackerStatisticsEventSender>, StatsRepository) {
        let mut stats_tracker = Self::new();

        let stats_event_sender = stats_tracker.run_event_listener();

        (stats_event_sender, stats_tracker.stats_repository)
    }

    pub fn new() -> Self {
        Self {
            stats_repository: StatsRepository::new(),
        }
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
        let mut stats_lock = stats_repository.stats.write().await;

        match event {
            TrackerStatisticsEvent::Tcp4Announce => {
                stats_lock.tcp4_announces_handled += 1;
                stats_lock.tcp4_connections_handled += 1;
            }
            TrackerStatisticsEvent::Tcp4Scrape => {
                stats_lock.tcp4_scrapes_handled += 1;
                stats_lock.tcp4_connections_handled += 1;
            }
            TrackerStatisticsEvent::Tcp6Announce => {
                stats_lock.tcp6_announces_handled += 1;
                stats_lock.tcp6_connections_handled += 1;
            }
            TrackerStatisticsEvent::Tcp6Scrape => {
                stats_lock.tcp6_scrapes_handled += 1;
                stats_lock.tcp6_connections_handled += 1;
            }
            TrackerStatisticsEvent::Udp4Connect => {
                stats_lock.udp4_connections_handled += 1;
            }
            TrackerStatisticsEvent::Udp4Announce => {
                stats_lock.udp4_announces_handled += 1;
            }
            TrackerStatisticsEvent::Udp4Scrape => {
                stats_lock.udp4_scrapes_handled += 1;
            }
            TrackerStatisticsEvent::Udp6Connect => {
                stats_lock.udp6_connections_handled += 1;
            }
            TrackerStatisticsEvent::Udp6Announce => {
                stats_lock.udp6_announces_handled += 1;
            }
            TrackerStatisticsEvent::Udp6Scrape => {
                stats_lock.udp6_scrapes_handled += 1;
            }
        }

        debug!("stats: {:?}", stats_lock);

        drop(stats_lock);
    }
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
}
