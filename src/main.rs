#![forbid(unsafe_code)]

extern crate clap;
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
    let parser = clap::App::new("udpt")
        .about("High performance, lightweight, udp based torrent tracker.")
        .author("Naim A. <naim94a@gmail.com>")
        .arg(clap::Arg::with_name("config").takes_value(true).short("-c").help("Configuration file to load.").required(true));

    let matches = parser.get_matches();
    let cfg_path = matches.value_of("config").unwrap();

    let cfg = match Configuration::load_file(cfg_path) {
        Ok(v) => std::sync::Arc::new(v),
        Err(e) => {
            eprintln!("failed to open configuration: {}", e);
            return;
        }
    };

    let tracker = std::sync::Arc::new(tracker::TorrentTracker::new(cfg.get_mode().clone()));

    // start http server:
    if cfg.get_http_config().is_some() {
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
