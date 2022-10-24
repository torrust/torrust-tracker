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
    pub stats: Arc<RwLock<TrackerStatistics>>,
}

impl StatsTracker {
    pub fn new_active_instance() -> (Self, Box<dyn TrackerStatisticsEventSender>) {
        let mut stats_tracker = Self {
            stats: Arc::new(RwLock::new(TrackerStatistics::new())),
        };

        let stats_event_sender = stats_tracker.run_worker();

        (stats_tracker, stats_event_sender)
    }

    pub fn new_inactive_instance() -> Self {
        Self {
            stats: Arc::new(RwLock::new(TrackerStatistics::new())),
        }
    }

    pub fn new_instance(active: bool) -> Self {
        if !active {
            return Self::new_inactive_instance();
        }

        let (stats_tracker, _stats_event_sender) = Self::new_active_instance();

        stats_tracker
    }

    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(TrackerStatistics::new())),
        }
    }

    pub fn run_worker(&mut self) -> Box<dyn TrackerStatisticsEventSender> {
        let (tx, rx) = mpsc::channel::<TrackerStatisticsEvent>(CHANNEL_BUFFER_SIZE);

        let stats = self.stats.clone();

        tokio::spawn(async move { event_listener(rx, stats).await });

        Box::new(StatsEventSender { sender: tx })
    }
}

async fn event_listener(mut rx: Receiver<TrackerStatisticsEvent>, stats: Arc<RwLock<TrackerStatistics>>) {
    while let Some(event) = rx.recv().await {
        let mut stats_lock = stats.write().await;

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

#[async_trait]
pub trait TrackerStatisticsRepository: Sync + Send {
    async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStatistics>;
}

#[async_trait]
impl TrackerStatisticsRepository for StatsTracker {
    async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStatistics> {
        self.stats.read().await
    }
}

pub trait TrackerStatsService: TrackerStatisticsRepository {}

impl TrackerStatsService for StatsTracker {}
