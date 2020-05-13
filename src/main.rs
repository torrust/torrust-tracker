use clap;
use fern;
use log::{error, info, trace, warn};

mod config;
mod server;
mod stackvec;
mod tracker;
mod utils;
mod webserver;

use config::Configuration;
use std::process::exit;

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
                .required(true),
        );

    let matches = parser.get_matches();
    let cfg_path = matches.value_of("config").unwrap();

    let cfg = match Configuration::load_file(cfg_path) {
        Ok(v) => std::sync::Arc::new(v),
        Err(e) => {
            eprintln!("udpt: failed to open configuration: {}", e);
            return;
        }
    };

    setup_logging(&cfg);

    let tracker_obj = match cfg.get_db_path() {
        Some(path) => {
            let file_path = std::path::Path::new(path);
            if !file_path.exists() {
                warn!("database file \"{}\" doesn't exist.", path);
                tracker::TorrentTracker::new(cfg.get_mode().clone())
            } else {
                let mut input_file = match tokio::fs::File::open(file_path).await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("failed to open \"{}\". error: {}", path.as_str(), err);
                        panic!("error opening file. check logs.");
                    }
                };
                match tracker::TorrentTracker::load_database(cfg.get_mode().clone(), &mut input_file).await {
                    Ok(v) => {
                        info!("database loaded.");
                        v
                    }
                    Err(err) => {
                        error!("failed to load database. error: {}", err);
                        panic!("failed to load database. check logs.");
                    }
                }
            }
        }
        None => tracker::TorrentTracker::new(cfg.get_mode().clone()),
    };

    let tracker = std::sync::Arc::new(tracker_obj);

    if cfg.get_http_config().is_some() {
        let https_tracker = tracker.clone();
        let http_cfg = cfg.clone();

        info!("Starting http server");
        tokio::spawn(async move {
            let http_cfg = http_cfg.get_http_config().unwrap();
            let bind_addr = http_cfg.get_address();
            let tokens = http_cfg.get_access_tokens();

            let server = webserver::build_server(https_tracker, tokens.clone());
            server.bind(bind_addr.parse::<std::net::SocketAddr>().unwrap()).await;
        });
    }

    let udp_server = server::UDPTracker::new(cfg.clone(), tracker.clone())
        .await
        .expect("failed to bind udp socket");

    trace!("Waiting for UDP packets");
    let udp_server = tokio::spawn(async move {
        if let Err(err) = udp_server.accept_packets().await {
            eprintln!("error: {}", err);
        }
    });

    let weak_tracker = std::sync::Arc::downgrade(&tracker);
    if let Some(db_path) = cfg.get_db_path() {
        let db_path = db_path.clone();
        let interval = cfg.get_cleanup_interval().unwrap_or(600);

        tokio::spawn(async move {
            let interval = std::time::Duration::from_secs(interval);
            let mut interval = tokio::time::interval(interval);
            interval.tick().await; // first tick is immediate...
            loop {
                interval.tick().await;
                if let Some(tracker) = weak_tracker.upgrade() {
                    tracker.periodic_task(&db_path).await;
                } else {
                    break;
                }
            }
        });
    }

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        _ = udp_server => { warn!("udp server exited.") },
        _ = ctrl_c => { info!("CTRL-C, exiting...") },
    }

    if let Some(path) = cfg.get_db_path() {
        info!("saving database...");
        tracker.periodic_task(path).await;
    }

    info!("goodbye.");
}
