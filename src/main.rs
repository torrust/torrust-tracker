use fern;
use log::{info};
use std::process::exit;
use torrust_tracker::{webserver, Configuration, TorrentTracker, UDPServer, HttpTrackerConfig, UdpTrackerConfig, HttpApiConfig};
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

    setup_logging(&config);

    // the singleton torrent tracker that gets passed to the HTTP and UDP server
    let tracker = Arc::new(TorrentTracker::new(config.clone()));

    // start torrent cleanup job (periodically removes old peers)
    let _torrent_cleanup_job = start_torrent_cleanup_job(config.clone(), tracker.clone()).unwrap();

    // start HTTP API server
    if let Some(http_api_config) = &config.http_api {
        let _api_server = start_api_server(&http_api_config, tracker.clone());
    };

    // check which tracker to run, UDP (Default) or HTTP
    let _tracker_server = if let Some(http_config) = &config.http_tracker {
        if http_config.enabled {
            start_http_tracker_server(http_config, tracker.clone())
        } else {
            start_udp_tracker_server(&config.udp_tracker, tracker.clone()).await
        }
    } else {
        start_udp_tracker_server(&config.udp_tracker, tracker.clone()).await
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
        let server = webserver::build_server(tracker);
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
    let udp_server = UDPServer::new(tracker).await.unwrap_or_else(|e| {
        panic!("Could not start UDP server: {}", e);
    });

    info!("Starting UDP tracker server..");
    tokio::spawn(async move {
        if let Err(e) = udp_server.accept_packets().await {
            panic!("Could not start UDP server: {}", e);
        }
    })
}

fn setup_logging(cfg: &Configuration) {
    let log_level = match &cfg.log_level {
        None => log::LevelFilter::Info,
        Some(level) => {
            match level.as_str() {
                "off" => log::LevelFilter::Off,
                "trace" => log::LevelFilter::Trace,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                "error" => log::LevelFilter::Error,
                _ => {
                    eprintln!("T3: unknown log level encountered '{}'", level.as_str());
                    exit(-1);
                }
            }
        }
    };

    if let Err(err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
    {
        eprintln!("T3: failed to initialize logging. {}", err);
        std::process::exit(-1);
    }
    info!("logging initialized.");
}
