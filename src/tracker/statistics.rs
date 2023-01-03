use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
#[cfg(test)]
use mockall::{automock, predicate::str};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::{mpsc, RwLock, RwLockReadGuard};

const CHANNEL_BUFFER_SIZE: usize = 65_535;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
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

#[derive(Debug, PartialEq, Default)]
pub struct Metrics {
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

pub struct Keeper {
    pub repository: Repo,
}

impl Default for Keeper {
    fn default() -> Self {
        Self::new()
    }
}

impl Keeper {
    #[must_use]
    pub fn new() -> Self {
        Self { repository: Repo::new() }
    }

    #[must_use]
    pub fn new_active_instance() -> (Box<dyn EventSender>, Repo) {
        let mut stats_tracker = Self::new();

        let stats_event_sender = stats_tracker.run_event_listener();

        (stats_event_sender, stats_tracker.repository)
    }

    pub fn run_event_listener(&mut self) -> Box<dyn EventSender> {
        let (sender, receiver) = mpsc::channel::<Event>(CHANNEL_BUFFER_SIZE);

        let stats_repository = self.repository.clone();

        tokio::spawn(async move { event_listener(receiver, stats_repository).await });

        Box::new(Sender { sender })
    }
}

async fn event_listener(mut receiver: mpsc::Receiver<Event>, stats_repository: Repo) {
    while let Some(event) = receiver.recv().await {
        event_handler(event, &stats_repository).await;
    }
}

async fn event_handler(event: Event, stats_repository: &Repo) {
    match event {
        // TCP4
        Event::Tcp4Announce => {
            stats_repository.increase_tcp4_announces().await;
            stats_repository.increase_tcp4_connections().await;
        }
        Event::Tcp4Scrape => {
            stats_repository.increase_tcp4_scrapes().await;
            stats_repository.increase_tcp4_connections().await;
        }

        // TCP6
        Event::Tcp6Announce => {
            stats_repository.increase_tcp6_announces().await;
            stats_repository.increase_tcp6_connections().await;
        }
        Event::Tcp6Scrape => {
            stats_repository.increase_tcp6_scrapes().await;
            stats_repository.increase_tcp6_connections().await;
        }

        // UDP4
        Event::Udp4Connect => {
            stats_repository.increase_udp4_connections().await;
        }
        Event::Udp4Announce => {
            stats_repository.increase_udp4_announces().await;
        }
        Event::Udp4Scrape => {
            stats_repository.increase_udp4_scrapes().await;
        }

        // UDP6
        Event::Udp6Connect => {
            stats_repository.increase_udp6_connections().await;
        }
        Event::Udp6Announce => {
            stats_repository.increase_udp6_announces().await;
        }
        Event::Udp6Scrape => {
            stats_repository.increase_udp6_scrapes().await;
        }
    }

    debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait EventSender: Sync + Send {
    async fn send_event(&self, event: Event) -> Option<Result<(), SendError<Event>>>;
}

pub struct Sender {
    sender: mpsc::Sender<Event>,
}

#[async_trait]
impl EventSender for Sender {
    async fn send_event(&self, event: Event) -> Option<Result<(), SendError<Event>>> {
        Some(self.sender.send(event).await)
    }
}

#[derive(Clone)]
pub struct Repo {
    pub stats: Arc<RwLock<Metrics>>,
}

impl Default for Repo {
    fn default() -> Self {
        Self::new()
    }
}

impl Repo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(Metrics::default())),
        }
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, Metrics> {
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
        use crate::tracker::statistics::{Event, Keeper, Metrics};

        #[tokio::test]
        async fn should_contain_the_tracker_statistics() {
            let stats_tracker = Keeper::new();

            let stats = stats_tracker.repository.get_stats().await;

            assert_eq!(stats.tcp4_announces_handled, Metrics::default().tcp4_announces_handled);
        }

        #[tokio::test]
        async fn should_create_an_event_sender_to_send_statistical_events() {
            let mut stats_tracker = Keeper::new();

            let event_sender = stats_tracker.run_event_listener();

            let result = event_sender.send_event(Event::Udp4Connect).await;

            assert!(result.is_some());
        }
    }

    mod event_handler {
        use crate::tracker::statistics::{event_handler, Event, Repo};

        #[tokio::test]
        async fn should_increase_the_tcp4_announces_counter_when_it_receives_a_tcp4_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_connections_counter_when_it_receives_a_tcp4_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_scrapes_counter_when_it_receives_a_tcp4_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp4_connections_counter_when_it_receives_a_tcp4_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_announces_counter_when_it_receives_a_tcp6_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_connections_counter_when_it_receives_a_tcp6_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_scrapes_counter_when_it_receives_a_tcp6_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_tcp6_connections_counter_when_it_receives_a_tcp6_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Tcp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_connections_counter_when_it_receives_a_udp4_connect_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp4Connect, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_announces_counter_when_it_receives_a_udp4_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp4Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp4_scrapes_counter_when_it_receives_a_udp4_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp4Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp4_scrapes_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_connections_counter_when_it_receives_a_udp6_connect_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp6Connect, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_connections_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_announces_counter_when_it_receives_a_udp6_announce_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp6Announce, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_announces_handled, 1);
        }

        #[tokio::test]
        async fn should_increase_the_udp6_scrapes_counter_when_it_receives_a_udp6_scrape_event() {
            let stats_repository = Repo::new();

            event_handler(Event::Udp6Scrape, &stats_repository).await;

            let stats = stats_repository.get_stats().await;

            assert_eq!(stats.udp6_scrapes_handled, 1);
        }
    }
}
