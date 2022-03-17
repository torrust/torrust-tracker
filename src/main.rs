use std::net::SocketAddr;
use std::sync::Arc;
use log::info;
use tokio::task::JoinHandle;
use torrust_tracker::{Configuration, http_api_server, HttpApiConfig, HttpTrackerConfig, logging, TorrentTracker, UdpServer, UdpTrackerConfig};
use torrust_tracker::torrust_http_tracker::server::HttpServer;

#[tokio::main]
async fn main() {
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
    }

    // start torrent cleanup job (periodically removes old peers)
    let _torrent_cleanup_job = start_torrent_cleanup_job(config.clone(), tracker.clone()).unwrap();

    // start HTTP API server
    if config.http_api.enabled {
        let _api_server = start_api_server(&config.http_api, tracker.clone());
    }

    let (tx, rx) = tokio::sync::watch::channel(false);
    let mut udp_server_handles = Vec::new();

    // start the udp blocks
    for udp_tracker in &config.udp_trackers {
        // used to send kill signal to thread

        if udp_tracker.enabled {
            udp_server_handles.push(
                start_udp_tracker_server(&udp_tracker, tracker.clone(), rx.clone()).await
            )
        }
    }

    // start the http blocks
    for http_tracker in &config.http_trackers {
        let _ = start_http_tracker_server(&http_tracker, tracker.clone(), true);
        let _ = start_http_tracker_server(&http_tracker, tracker.clone(), false);
    }

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
                let _ = tracker.save_torrents().await;
                info!("Torrents saved");
            }
        }
    }
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

fn start_api_server(config: &HttpApiConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    info!("Starting HTTP API server on: {}", config.bind_address);
    let bind_addr = config.bind_address.parse::<std::net::SocketAddr>().unwrap();

    tokio::spawn(async move {
        let server = http_api_server::build_server(tracker);
        let _ = server.bind(bind_addr).await;
    })
}

fn start_http_tracker_server(config: &HttpTrackerConfig, tracker: Arc<TorrentTracker>, ssl: bool) -> JoinHandle<()> {
    let http_tracker = HttpServer::new(tracker);
    let enabled = config.enabled;
    let bind_addr = config.bind_address.parse::<SocketAddr>().unwrap();
    let ssl_enabled = config.ssl_enabled;
    let ssl_bind_addr = config.ssl_bind_address.parse::<SocketAddr>().unwrap();
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    tokio::spawn(async move {
        // run with tls if ssl_enabled and cert and key path are set
        if ssl && ssl_enabled && ssl_cert_path.is_some() && ssl_key_path.is_some() {
            info!("Starting HTTPS server on: {} (TLS)", ssl_bind_addr);
            http_tracker.start_tls(ssl_bind_addr, ssl_cert_path.as_ref().unwrap(), ssl_key_path.as_ref().unwrap()).await;
        }
        if !ssl && enabled {
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
