#![forbid(unsafe_code)]

extern crate bincode;
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate binascii;
extern crate toml;
#[macro_use]
extern crate log;
extern crate bzip2;
extern crate fern;
extern crate num_cpus;
extern crate serde_json;

mod config;
mod server;
mod stackvec;
mod tracker;
mod webserver;

use config::Configuration;
use std::process::exit;

fn setup_logging(cfg: &Configuration) {
    let log_level = match cfg.get_log_level() {
        None => log::LevelFilter::Info,
        Some(level) => match level.as_str() {
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
        },
    };

    if let Err(err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}]\t{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
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

fn main() {
    let parser = clap::App::new("udpt")
        .about("High performance, lightweight, udp based torrent tracker.")
        .author("Naim A. <naim94a@gmail.com>")
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
            }
            else {
                let mut input_file = match std::fs::File::open(file_path) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("failed to open \"{}\". error: {}", path.as_str(), err);
                        panic!("error opening file. check logs.");
                    }
                };
                match tracker::TorrentTracker::load_database(cfg.get_mode().clone(), &mut input_file) {
                    Ok(v) => v,
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

    // start http server:
    if cfg.get_http_config().is_some() {
        let http_tracker_ref = tracker.clone();
        let cfg_ref = cfg.clone();
        std::thread::spawn(move || {
            webserver::WebServer::new(http_tracker_ref, cfg_ref);
        });
    }

    let udp_server = std::sync::Arc::new(server::UDPTracker::new(cfg.clone(), tracker.clone()).unwrap());

    trace!("Waiting for UDP packets");
    let logical_cpus = num_cpus::get();
    let mut threads = Vec::with_capacity(logical_cpus);
    for i in 0..logical_cpus {
        debug!("starting thread {}/{}", i + 1, logical_cpus);
        let server_handle = udp_server.clone();
        threads.push(std::thread::spawn(move || loop {
            match server_handle.accept_packet() {
                Err(e) => {
                    error!("Failed to process packet. {}", e);
                }
                Ok(_) => {}
            }
        }));
    }

    match cfg.get_db_path() {
        Some(db_path) => {
            let db_p = db_path.clone();
            let tracker_clone = tracker.clone();

            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::new(120, 0));
                    debug!("periodically saving database.");
                    tracker_clone.periodic_task(db_p.as_str());
                    debug!("database saved.");
                }
            });
        },
        None => {}
    }

    while !threads.is_empty() {
        if let Some(thread) = threads.pop() {
            let _ = thread.join();
        }
    }
}
