use log::{info};
use torrust_tracker::{http_api_server, Configuration, TorrentTracker, UdpServer, HttpTrackerConfig, UdpTrackerConfig, HttpApiConfig, logging, TrackerServer};
use std::sync::Arc;
use tokio::task::JoinHandle;
use torrust_tracker::http_server::HttpServer;

#[tokio::main]
async fn main() {
    let config = match Configuration::load_from_file() {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
    };

    logging::setup_logging(&config);

    // the singleton torrent tracker that gets passed to the HTTP and UDP server
    let tracker = Arc::new(TorrentTracker::new(config.clone()));

    // start torrent cleanup job (periodically removes old peers)
    let _torrent_cleanup_job = start_torrent_cleanup_job(config.clone(), tracker.clone()).unwrap();

    // start HTTP API server
    if config.http_api.enabled {
        let _api_server = start_api_server(&config.http_api, tracker.clone());
    }

    // check which tracker to run, UDP (Default) or HTTP
    let _tracker_server = match config.get_tracker_server() {
        TrackerServer::UDP => {
            start_udp_tracker_server(&config.udp_tracker, tracker.clone()).await
        }
        TrackerServer::HTTP => {
            start_http_tracker_server(&config.http_tracker, tracker.clone())
        }
    };

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::select! {
        _ = _tracker_server => { panic!("Tracker server exited.") },
        _ = ctrl_c => { info!("Torrust shutting down..") }
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
    }))
}

fn start_api_server(config: &HttpApiConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    info!("Starting HTTP API server on: {}", config.bind_address);
    let bind_addr = config.bind_address.parse::<std::net::SocketAddr>().unwrap();

    tokio::spawn(async move {
        let server = http_api_server::build_server(tracker);
        server.bind(bind_addr).await;
    })
}

fn start_http_tracker_server(config: &HttpTrackerConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    info!("Starting HTTP server on: {}", config.bind_address);
    let http_tracker = Arc::new(HttpServer::new(tracker));
    let bind_addr = config.bind_address.parse::<std::net::SocketAddrV4>().unwrap();
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    tokio::spawn(async move {
        // run with tls if ssl_enabled and cert and key path are set
        if ssl_enabled {
            info!("SSL enabled.");
            warp::serve(HttpServer::routes(http_tracker))
                .tls()
                .cert_path(ssl_cert_path.as_ref().unwrap())
                .key_path(ssl_key_path.as_ref().unwrap())
                .run(bind_addr).await;
        } else {
            warp::serve(HttpServer::routes(http_tracker))
                .run(bind_addr).await;
        }
    })
}

async fn start_udp_tracker_server(config: &UdpTrackerConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    info!("Starting UDP server on: {}", config.bind_address);
    let udp_server = UdpServer::new(tracker).await.unwrap_or_else(|e| {
        panic!("Could not start UDP server: {}", e);
    });

    info!("Starting UDP tracker server..");
    tokio::spawn(async move {
        if let Err(e) = udp_server.accept_packets().await {
            panic!("Could not start UDP server: {}", e);
        }
    })
}
