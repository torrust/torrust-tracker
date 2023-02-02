use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker::config::{ephemeral_configuration, Configuration};
use torrust_tracker::jobs::udp_tracker;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};

pub fn start_udp_tracker(configuration: &Arc<Configuration>) -> Server {
    let mut udp_server = Server::new();
    udp_server.start(configuration);
    udp_server
}

pub fn tracker_configuration() -> Arc<Configuration> {
    Arc::new(ephemeral_configuration())
}
pub struct Server {
    pub started: AtomicBool,
    pub job: Option<JoinHandle<()>>,
    pub bind_address: Option<SocketAddr>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            started: AtomicBool::new(false),
            job: None,
            bind_address: None,
        }
    }

    pub fn start(&mut self, configuration: &Arc<Configuration>) {
        if !self.started.load(Ordering::Relaxed) {
            // Set the time of Torrust app starting
            lazy_static::initialize(&static_time::TIME_AT_APP_START);

            // Initialize the Ephemeral Instance Random Seed
            lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

            // Initialize stats tracker
            let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

            // Initialize Torrust tracker
            let tracker = match tracker::Tracker::new(&configuration.clone(), Some(stats_event_sender), stats_repository) {
                Ok(tracker) => Arc::new(tracker),
                Err(error) => {
                    panic!("{}", error)
                }
            };

            // Initialize logging
            logging::setup(configuration);

            let udp_tracker_config = &configuration.udp_trackers[0];

            // Start the UDP tracker job
            self.job = Some(udp_tracker::start_job(udp_tracker_config, tracker));

            self.bind_address = Some(udp_tracker_config.bind_address.parse().unwrap());

            self.started.store(true, Ordering::Relaxed);
        }
    }
}
