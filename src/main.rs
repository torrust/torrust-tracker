extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod server;
mod tracker;
mod stackvec;

fn main() {
    let tracker = std::sync::Arc::new(tracker::TorrentTracker::new());

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
