extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod server;
mod tracker;

fn main() {
    let mut tracker = tracker::TorrentTracker::new();

    let addr = "0.0.0.0:1212";
    let mut s = server::UDPTracker::new(addr, &mut tracker).unwrap();
    loop {
        match s.accept_packet() {
            Err(e) => {
                println!("error: {}", e);
            },
            Ok(_) => {},
        }
    }
}
