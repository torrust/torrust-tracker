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
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to open configuration: {}", e);
            return;
        }
    };


    let tracker = std::sync::Arc::new(tracker::TorrentTracker::new());

    // start http server:
    let mut access_tokens = std::collections::HashMap::new();
    access_tokens.insert(String::from("MySpecialToken"), String::from("username"));

    let http_tracker_ref = tracker.clone();
    std::thread::spawn(move || {
        webserver::WebServer::new(http_tracker_ref);
    });

    let addr = "0.0.0.0:1212";
    let s = std::sync::Arc::new(server::UDPTracker::new(addr, tracker.clone()).unwrap());

    loop {
        match s.accept_packet() {
            Err(e) => {
                println!("error: {}", e);
            },
            Ok(_) => {},
        }
    }
}
