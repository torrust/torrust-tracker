extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate binascii;

mod server;
mod tracker;
mod stackvec;
mod webserver;

fn main() {
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
