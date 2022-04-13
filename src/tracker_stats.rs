use std::sync::Arc;
use tokio::sync::{mpsc, RwLock, RwLockReadGuard};
use tokio::sync::mpsc::{Sender};
use tokio::sync::mpsc::error::SendError;

const CHANNEL_BUFFER_SIZE: usize = 65_535;

#[derive(Debug)]
pub enum TrackerStatsEvent {
    Tcp4Announce,
    Tcp4Scrape,
    Tcp6Announce,
    Tcp6Scrape,
    Udp4Connect,
    Udp4Announce,
    Udp4Scrape,
    Udp6Connect,
    Udp6Announce,
    Udp6Scrape
}

#[derive(Debug)]
pub struct TrackerStats {
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

impl TrackerStats {
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
    channel_sender: Option<Sender<TrackerStatsEvent>>,
    pub stats: Arc<RwLock<TrackerStats>>
}

impl StatsTracker {
    pub fn new() -> Self {
        Self {
            channel_sender: None,
            stats: Arc::new(RwLock::new(TrackerStats::new()))
        }
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStats> {
        self.stats.read().await
    }

    pub async fn send_event(&self, event: TrackerStatsEvent) -> Option<Result<(), SendError<TrackerStatsEvent>>> {
        if let Some(tx) = &self.channel_sender {
            Some(tx.send(event).await)
        } else {
            None
        }
    }

    pub fn run_worker(&mut self) {
        let (tx, mut rx) = mpsc::channel::<TrackerStatsEvent>(CHANNEL_BUFFER_SIZE);

        // set send channel on stats_tracker
        self.channel_sender = Some(tx);

        let stats = self.stats.clone();

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                let mut stats_lock = stats.write().await;

                match event {
                    TrackerStatsEvent::Tcp4Announce => {
                        stats_lock.tcp4_announces_handled += 1;
                        stats_lock.tcp4_connections_handled += 1;
                    }
                    TrackerStatsEvent::Tcp4Scrape => {
                        stats_lock.tcp4_scrapes_handled += 1;
                        stats_lock.tcp4_connections_handled += 1;
                    }
                    TrackerStatsEvent::Tcp6Announce => {
                        stats_lock.tcp6_announces_handled += 1;
                        stats_lock.tcp6_connections_handled += 1;
                    }
                    TrackerStatsEvent::Tcp6Scrape => {
                        stats_lock.tcp6_scrapes_handled += 1;
                        stats_lock.tcp6_connections_handled += 1;
                    }
                    TrackerStatsEvent::Udp4Connect => { stats_lock.udp4_connections_handled += 1; }
                    TrackerStatsEvent::Udp4Announce => { stats_lock.udp4_announces_handled += 1; }
                    TrackerStatsEvent::Udp4Scrape => { stats_lock.udp4_scrapes_handled += 1; }
                    TrackerStatsEvent::Udp6Connect => { stats_lock.udp6_connections_handled += 1; }
                    TrackerStatsEvent::Udp6Announce => { stats_lock.udp6_announces_handled += 1; }
                    TrackerStatsEvent::Udp6Scrape => { stats_lock.udp6_scrapes_handled += 1; }
                }

                drop(stats_lock);
            }
        });
    }
}
