extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate binascii;
extern crate toml;

mod server;
mod tracker;
mod stackvec;
mod webserver;
mod config;
use config::Configuration;

fn main() {
    let cfg = match Configuration::load_file("udpt.toml") {
        Ok(v) => std::sync::Arc::new(v),
        Err(e) => {
            eprintln!("failed to open configuration: {}", e);
            return;
        }
    };

    let tracker = std::sync::Arc::new(tracker::TorrentTracker::new(cfg.get_mode().clone()));

    // start http server:
    if let Some(http_cfg) = cfg.get_http_config() {
        let http_tracker_ref = tracker.clone();
        let cfg_ref = cfg.clone();
        std::thread::spawn(move || {
            webserver::WebServer::new(http_tracker_ref, cfg_ref);
        });
    }

    let s = std::sync::Arc::new(server::UDPTracker::new(cfg, tracker.clone()).unwrap());

    loop {
        match s.accept_packet() {
            Err(e) => {
                println!("error: {}", e);
            },
            Ok(_) => {},
        }
    }
}
