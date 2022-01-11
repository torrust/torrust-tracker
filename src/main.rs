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
    let config = match Configuration::load_from_file() {
        Ok(config) => Arc::new(config),
        Err(error) => {
            eprintln!("{}", error);
            exit(-1);
        }
    };

    setup_logging(&config);

    let sqlite_database = match SqliteDatabase::new(config.get_db_path()).await {
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
    if config.get_http_api_config().is_some() {
        info!("Starting API server..");
        return Some(tokio::spawn(async move {
            let http_cfg = config.get_http_api_config().unwrap();
            let bind_addr = http_cfg.get_address();
            let tokens = http_cfg.get_access_tokens();

            let server = webserver::build_server(tracker, tokens.clone());
            server.bind(bind_addr.parse::<std::net::SocketAddr>().unwrap()).await;
        }))
    }

    None
}

fn start_http_tracker_server(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Option<JoinHandle<()>> {
    if config.get_http_tracker_config().is_some() {
        let http_tracker = Arc::new(HttpServer::new(config.clone(), tracker));

        return Some(tokio::spawn(async move {
            let http_tracker_config = config.get_http_tracker_config().unwrap();
            let bind_addr = http_tracker_config.get_address().parse::<std::net::SocketAddrV4>().unwrap();
            println!("{}", bind_addr);

            // run with tls if ssl_enabled and cert and key path are set
            if http_tracker_config.is_ssl_enabled() {
                info!("Starting HTTP tracker server in TLS mode..");
                warp::serve(HttpServer::routes(http_tracker))
                    .tls()
                    .cert_path(&http_tracker_config.ssl_cert_path.as_ref().unwrap())
                    .key_path(&http_tracker_config.ssl_key_path.as_ref().unwrap())
                    .run(bind_addr).await;
            } else {
                info!("Starting HTTP tracker server..");
                warp::serve(HttpServer::routes(http_tracker))
                    .run(bind_addr).await;
            }

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
