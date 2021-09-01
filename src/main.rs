use clap;
use fern;
use log::{info, trace};

use std::process::exit;
use torrust_tracker::{webserver, Configuration, udp_server, TorrentTracker};
use torrust_tracker::database::SqliteDatabase;
use std::sync::Arc;
use torrust_tracker::key_manager::KeyManager;

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

    let cfg = match matches.value_of("config") {
        Some(cfg_path) => {
            match Configuration::load_file(cfg_path) {
                Ok(v) => Arc::new(v),
                Err(e) => {
                    eprintln!("udpt: failed to open configuration: {}", e);
                    return;
                }
            }
        },
        None => {
            eprintln!("No TOML supplied. Loading default configuration.");
            Arc::new(Configuration::default().await)
        }
    };

    setup_logging(&cfg);

    let sqlite_database = SqliteDatabase::new().await.unwrap();

    match sqlite_database.create_database() {
        Ok(_) => {
            eprintln!("Whitelist table exists in database.");
        }
        Err(_) => {
            eprintln!("Could not create database table. Exiting..");
            exit(-1);
        }
    }

    let arc_sqlite_database = Arc::new(sqlite_database);

    let key_manager = KeyManager::new(cfg.get_secret().to_string());
    let arc_key_manager = Arc::new(key_manager);

    let torrent_tracker = TorrentTracker::new(cfg.clone(), arc_sqlite_database, arc_key_manager);
    let arc_torrent_tracker = Arc::new(torrent_tracker);


    // periodically call tracker.cleanup_torrents()
    {
        let weak_tracker = std::sync::Arc::downgrade(&arc_torrent_tracker);
        let interval = cfg.get_cleanup_interval().unwrap_or(600);

        tokio::spawn(async move {
            let interval = std::time::Duration::from_secs(interval);
            let mut interval = tokio::time::interval(interval);
            interval.tick().await; // first tick is immediate...
            loop {
                interval.tick().await;
                if let Some(tracker) = weak_tracker.upgrade() {
                    tracker.cleanup_torrents().await;
                } else {
                    break;
                }
            }
        });
    }

    // start http server
    {
        if cfg.get_http_config().is_some() {
            let https_tracker = arc_torrent_tracker.clone();
            let http_cfg = cfg.clone();

            info!("Starting HTTP server");
            tokio::spawn(async move {
                let http_cfg = http_cfg.get_http_config().unwrap();
                let bind_addr = http_cfg.get_address();
                let tokens = http_cfg.get_access_tokens();

                let server = webserver::build_server(https_tracker, tokens.clone());
                server.bind(bind_addr.parse::<std::net::SocketAddr>().unwrap()).await;
            });
        }
    }

    // start udp server
    {
        let udp_server = udp_server::UDPServer::new(cfg.clone(), arc_torrent_tracker.clone())
            .await
            .expect("failed to bind udp socket");

        trace!("Waiting for UDP packets");
        let _udp_server = tokio::spawn(async move {
            if let Err(err) = udp_server.accept_packets().await {
                eprintln!("error: {}", err);
            }
        });
    }

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        //_ = udp_server => { warn!("udp server exited.") },
        _ = ctrl_c => { info!("CTRL-C, exiting...") },
    }

    info!("goodbye.");
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
