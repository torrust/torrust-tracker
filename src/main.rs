use std::net::SocketAddr;
use std::sync::Arc;
use crossbeam_channel::bounded;
use log::{info};
use tokio::task::JoinHandle;
use torrust_tracker::{Configuration, http_api_server, HttpApiConfig, HttpTrackerConfig, logging, TorrentTracker, UdpServer, UdpTrackerConfig};
use torrust_tracker::torrust_http_tracker::server::HttpServer;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub struct DataStream {
    action: u8,
    data: Vec<()>
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // Loading configuration
    let config = match Configuration::load_from_file() {
        Ok(config) => config,
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Start the thread where data is being exchanged for usaga
    let (sender, receiver): (crossbeam_channel::Sender<DataStream>, crossbeam_channel::Receiver<DataStream>) = bounded(1);
    let _torrents_memory_handler = start_torrents_memory_handler(&config, sender.clone(), receiver.clone());

    // torrust config
    let config = match Configuration::load_from_file() {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // the singleton torrent tracker that gets passed to the HTTP and UDP server
    let tracker = Arc::new(TorrentTracker::new(config.clone()).unwrap_or_else(|e| {
        panic!("{}", e)
    }));

    logging::setup_logging(&config);

    // load persistent torrents if enabled
    if config.persistence {
        info!("Loading persistent torrents into memory...");
        if tracker.load_torrents().await.is_err() {
            panic!("Could not load persistent torrents.")
        };
        info!("Persistent torrents loaded.");

        let _torrent_periodic_job = start_torrent_periodic_job(config.clone(), tracker.clone()).unwrap();
    }

    // start torrent cleanup job (periodically removes old peers)
    let _torrent_cleanup_job = start_torrent_cleanup_job(config.clone(), tracker.clone()).unwrap();

    // start HTTP API server
    if config.http_api.enabled {
        let _api_server = start_api_server(&config.http_api, tracker.clone());
    }

    // used to send graceful shutdown signal to udp listeners
    let (tx, rx) = tokio::sync::watch::channel(false);
    let mut udp_server_handles = Vec::new();

    // start the udp blocks
    for udp_tracker in &config.udp_trackers {
        if !udp_tracker.enabled { continue; }

        if tracker.is_private() {
            panic!("Could not start UDP tracker on: {} while in {:?}. UDP is not safe for private trackers!", udp_tracker.bind_address, config.mode);
        }

        udp_server_handles.push(
            start_udp_tracker_server(&udp_tracker, tracker.clone(), rx.clone()).await
        )
    }

    // start the http blocks
    for http_tracker in &config.http_trackers {
        if !http_tracker.enabled { continue; }

        // SSL requires a cert and a key
        if http_tracker.ssl_enabled && !http_tracker.verify_ssl_cert_and_key_set() {
            panic!("Could not start HTTP tracker on: {}, missing SSL Cert or Key!", http_tracker.bind_address);
        }

        let _ = start_http_tracker_server(&http_tracker, tracker.clone());
    }

    // start a thread to post statistics
    let _ = start_statistics_job(config.clone(), tracker.clone()).unwrap();

    // handle the signals here
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Torrust shutting down..");

            // send kill signal
            let _ = tx.send(true);

            // await for all udp servers to shutdown
            futures::future::join_all(udp_server_handles).await;

            // Save torrents if enabled
            if config.persistence {
                info!("Saving torrents into SQL from memory...");
                let _ = tracker.periodic_saving().await;
                info!("Torrents saved");
            }

            // Closing down channel
            sender.clone().send(DataStream{
                action: ACTION_CLOSE_CHANNEL,
                data: Vec::new()
            });
        }
    }
}


const ACTION_CLOSE_CHANNEL: u8 = 0;
const ACTION_READ_TORRENTS: u8 = 1;
const ACTION_WRITE_TORRENTS: u8 = 2;
const ACTION_UPDATE_TORRENTS: u8 = 3;
const ACTION_READ_PEERS: u8 = 4;
const ACTION_WRITE_PEERS: u8 = 5;
const ACTION_UPDATE_PEERS: u8 = 6;
fn start_torrents_memory_handler(config: &Configuration, sender: crossbeam_channel::Sender<DataStream>, receiver: crossbeam_channel::Receiver<DataStream>) -> Option<JoinHandle<()>> {
    // This is our main memory handler, everything will be received, handled and send back.
    return Some(tokio::spawn(async move {
        loop {
            // Wait for incoming data.
            let data: DataStream = receiver.recv().unwrap();

            // Lets check what action is given.
            match data.action {
                ACTION_CLOSE_CHANNEL => {
                    info!("Ending the memory handler thread...");
                    sender.send(DataStream{
                        action: ACTION_CLOSE_CHANNEL,
                        data: Vec::new()
                    });
                    break;
                }
                _ => {}
            }
        }
    }));
}

fn start_torrent_periodic_job(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.persistence_interval.unwrap_or(900);

    return Some(tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await; // first tick is immediate...
        // periodically call tracker.cleanup_torrents()
        loop {
            interval.tick().await;
            if let Some(tracker) = weak_tracker.upgrade() {
                info!("Executing periodic saving...");
                tracker.periodic_saving().await;
                info!("Periodic saving done.");
            } else {
                break;
            }
        }
    }));
}

fn start_torrent_cleanup_job(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.cleanup_interval.unwrap_or(600);

    return Some(tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await; // first tick is immediate...
        // periodically call tracker.cleanup_torrents()
        loop {
            interval.tick().await;
            if let Some(tracker) = weak_tracker.upgrade() {
                tracker.cleanup_torrents().await;
            } else {
                break;
            }
        }
    }));
}

fn start_statistics_job(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.log_interval.unwrap_or(60);

    return Some(tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(interval);
        let mut interval = tokio::time::interval(interval);
        interval.tick().await; // first tick is immediate...
        // periodically call tracker.cleanup_torrents()
        loop {
            interval.tick().await;
            if let Some(tracker) = weak_tracker.upgrade() {
                tracker.post_log().await;
            } else {
                break;
            }
        }
    }));
}

fn start_api_server(config: &HttpApiConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    info!("Starting HTTP API server on: {}", config.bind_address);
    let bind_addr = config.bind_address.parse::<std::net::SocketAddr>().unwrap();

    tokio::spawn(async move {
        let server = http_api_server::build_server(tracker);
        let _ = server.bind(bind_addr).await;
    })
}

fn start_http_tracker_server(config: &HttpTrackerConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    let http_tracker = HttpServer::new(tracker);
    let bind_addr = config.bind_address.parse::<SocketAddr>().unwrap();
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    tokio::spawn(async move {
        // run with tls if ssl_enabled and cert and key path are set
        if ssl_enabled {
            info!("Starting HTTPS server on: {} (TLS)", bind_addr);
            http_tracker.start_tls(bind_addr, ssl_cert_path.as_ref().unwrap(), ssl_key_path.as_ref().unwrap()).await;
        } else {
            info!("Starting HTTP server on: {}", bind_addr);
            http_tracker.start(bind_addr).await;
        }
    })
}

async fn start_udp_tracker_server(config: &UdpTrackerConfig, tracker: Arc<TorrentTracker>, rx: tokio::sync::watch::Receiver<bool>) -> JoinHandle<()> {
    let udp_server = UdpServer::new(tracker, &config.bind_address).await.unwrap_or_else(|e| {
        panic!("Could not start UDP server: {}", e);
    });

    info!("Starting UDP server on: {}", config.bind_address);
    tokio::spawn(async move {
        udp_server.start(rx).await;
    })
}
