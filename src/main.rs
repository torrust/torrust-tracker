#![forbid(unsafe_code)]

use clap;
use log::{trace, warn, info, debug, error};
use fern;
use num_cpus;
use lazy_static::lazy_static;

mod config;
mod server;
mod stackvec;
mod tracker;
mod webserver;

use config::Configuration;
use std::process::exit;

lazy_static!{
    static ref term_mutex: std::sync::Arc<std::sync::atomic::AtomicBool> = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
}

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

fn signal_termination() {
    term_mutex.store(true, std::sync::atomic::Ordering::Relaxed);
}

fn main() {
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

    let mut threads = Vec::new();

    let tracker = std::sync::Arc::new(tracker_obj);

    let http_server = if cfg.get_http_config().is_some() {
        let http_tracker_ref = tracker.clone();
        let cfg_ref = cfg.clone();

        Some(webserver::WebServer::new(http_tracker_ref, cfg_ref))
    } else {
        None
    };

    let udp_server = std::sync::Arc::new(server::UDPTracker::new(cfg.clone(), tracker.clone()).unwrap());

    trace!("Waiting for UDP packets");
    let logical_cpus = num_cpus::get();
    for i in 0..logical_cpus {
        debug!("starting thread {}/{}", i + 1, logical_cpus);
        let server_handle = udp_server.clone();
        let thread_term_ref = term_mutex.clone();
        threads.push(std::thread::spawn(move || loop {
            match server_handle.accept_packet() {
                Err(e) => {
                    if thread_term_ref.load(std::sync::atomic::Ordering::Relaxed) == true {
                        debug!("Thread terminating...");
                        break;
                    }
                    match e.kind() {
                        std::io::ErrorKind::TimedOut => {},
                        std::io::ErrorKind::WouldBlock => {},
                        _ => {
                            error!("Failed to process packet. {}", e);
                        }
                    }
                }
                Ok(_) => {}
            }
        }));
    }

    match cfg.get_db_path() {
        Some(db_path) => {
            let db_p = db_path.clone();
            let tracker_clone = tracker.clone();
            let cleanup_interval = match *cfg.get_cleanup_interval() {
                Some(v) => v,
                None => 10 * 60,
            };

            let thread_term_mutex = term_mutex.clone();
            threads.push(std::thread::spawn(move || {
                let timeout = std::time::Duration::new(cleanup_interval, 0);

                let timeout_start = std::time::Instant::now();
                let mut timeout_remaining = timeout;
                loop {
                    std::thread::park_timeout(std::time::Duration::new(cleanup_interval, 0));

                    if thread_term_mutex.load(std::sync::atomic::Ordering::Relaxed) {
                        debug!("Maintenance thread terminating.");
                        break;
                    }

                    let elapsed = std::time::Instant::now() - timeout_start;
                    if elapsed < timeout_remaining {
                        timeout_remaining = timeout - elapsed;
                        continue;
                    }
                    else {
                        timeout_remaining = timeout;
                    }

                    debug!("periodically saving database.");
                    tracker_clone.periodic_task(db_p.as_str());
                    debug!("database saved.");
                }
            }));
        },
        None => {}
    }

    loop {
        if term_mutex.load(std::sync::atomic::Ordering::Relaxed) {
            // termination signaled. start cleanup.
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    match http_server {
        Some(v) => v.shutdown(),
        None => {},
    };

    while !threads.is_empty() {
        if let Some(thread) = threads.pop() {
            thread.thread().unpark();
            let _ = thread.join();
        }
    }

    if let Some(db_path) = cfg.get_db_path() {
        info!("running final cleanup & saving database...");
        tracker.periodic_task(db_path.as_str());
    }
    info!("goodbye.");
}
