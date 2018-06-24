extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate hyper;
extern crate futures;

mod server;
mod tracker;
mod stackvec;
mod webserver;

fn main() {
    let tracker = std::sync::Arc::new(tracker::TorrentTracker::new());

    let addr = "0.0.0.0:1212";
    let s = std::sync::Arc::new(server::UDPTracker::new(addr, tracker.clone()).unwrap());

    use std::str::FromStr;
    let sa = std::net::SocketAddrV4::from_str("0.0.0.0:1213").unwrap();
    webserver::start_server(std::net::SocketAddr::from(sa), tracker.clone(), "myt0k3n");

    loop {
        match s.accept_packet() {
            Err(e) => {
                println!("error: {}", e);
            },
            Ok(_) => {},
        }
    }
}
