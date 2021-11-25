use std::net::{SocketAddrV4};
use clap;
use fern;
use log::{info, warn};

use std::process::exit;
use torrust_tracker::{webserver, Configuration, TorrentTracker, UDPServer};
use torrust_tracker::database::SqliteDatabase;
use std::sync::Arc;
use tokio::task::JoinHandle;
use torrust_tracker::http_server::HttpServer;

#[tokio::main]
async fn main() {
    let parser = clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::with_name("config")
                .takes_value(true)
                .short("-c")
                .help("Configuration file to load.")
        );

    let matches = parser.get_matches();

    let config = match matches.value_of("config") {
        Some(cfg_path) => {
            match Configuration::load_file(cfg_path) {
                Ok(v) => Arc::new(v),
                Err(e) => {
                    info!("udpt: failed to open configuration: {}", e);
                    return;
                }
            }
        },
        None => {
            info!("No TOML supplied. Loading default configuration.");
            Arc::new(Configuration::default().await)
        }
    };

    setup_logging(&config);

    let sqlite_database = match SqliteDatabase::new().await {
        Some(sqlite_database) => {
            info!("Verified database tables.");
            Arc::new(sqlite_database)
        }
        None => {
            eprintln!("Exiting..");
            exit(-1);
        }
    };

    // the singleton torrent tracker that gets passed to the HTTP and UDP server
    let tracker = Arc::new(TorrentTracker::new(config.clone(), sqlite_database.clone()));

    // start torrent cleanup job (periodically removes old peers)
    let _torrent_cleanup_job = start_torrent_cleanup_job(config.clone(), tracker.clone()).unwrap();

    // start HTTP API server
    let _api_server = start_api_server(config.clone(), tracker.clone());

    // start HTTP Tracker server
    let _http_server = start_http_tracker_server(config.clone(), tracker.clone());

    // start UDP Tracker server
    let udp_server = start_udp_tracker_server(config.clone(), tracker.clone()).await.unwrap();

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        _ = udp_server => { warn!("UDP Tracker server exited.") },
        _ = ctrl_c => { info!("T3 shutting down..") }
    }
}

fn start_torrent_cleanup_job(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    let interval = config.get_cleanup_interval().unwrap_or(600);

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

fn start_api_server(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    if config.get_http_config().is_some() {
        info!("Starting API server..");
        return Some(tokio::spawn(async move {
            let http_cfg = config.get_http_config().unwrap();
            let bind_addr = http_cfg.get_address();
            let tokens = http_cfg.get_access_tokens();

            let server = webserver::build_server(tracker, tokens.clone());
            server.bind(bind_addr.parse::<std::net::SocketAddr>().unwrap()).await;
        }))
    }

    None
}

fn start_http_tracker_server(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    if config.get_http_config().is_some() {
        let http_tracker = Arc::new(HttpServer::new(config, tracker));

        info!("Starting HTTP tracker server..");
        return Some(tokio::spawn(async move {
            warp::serve(HttpServer::routes(http_tracker))
                .tls()
                .cert_path("ssl/cert.pem")
                .key_path("ssl/key.rsa")
                .run(SocketAddrV4::new("0.0.0.0".parse().unwrap(), 7878)).await;
        }))
    }

    None
}

async fn start_udp_tracker_server(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    let udp_server = UDPServer::new(config, tracker)
        .await
        .expect("failed to bind udp socket");

    info!("Starting UDP tracker server..");
    Some(tokio::spawn(async move {
        if let Err(err) = udp_server.accept_packets().await {
            eprintln!("error: {}", err);
        }
    }))
}

fn setup_logging(cfg: &Configuration) {
    let log_level = match cfg.get_log_level() {
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
                    eprintln!("udpt: unknown log level encountered '{}'", level.as_str());
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
        eprintln!("udpt: failed to initialize logging. {}", err);
        std::process::exit(-1);
    }
    info!("logging initialized.");
}
