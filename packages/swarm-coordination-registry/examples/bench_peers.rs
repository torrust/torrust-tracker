//! Microbenchmark: `Coordinator::peers_excluding` throughput.
//! Usage: cargo run --package torrust-tracker-swarm-coordination-registry --example `bench_peers` --release

use std::hint::black_box;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId};
use torrust_tracker_swarm_coordination_registry::event::sender::Sender;
use torrust_tracker_swarm_coordination_registry::swarm::coordinator::Coordinator;

fn make_peer(ip_last_octet: u8, port: u16, seed: u8) -> Peer {
    let mut id = [seed; 20];
    id[0] = ip_last_octet;
    Peer {
        peer_id: PeerId(id),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip_last_octet)), port).into(),
        updated: DurationSinceUnixEpoch::new(1_669_397_478, 0),
        uploaded: NumberOfBytes::new(0),
        downloaded: NumberOfBytes::new(0),
        left: NumberOfBytes::new(0),
        event: AnnounceEvent::None,
    }
}

// Clippy notes on the casts below:
// - `i % 254 + 1` is safe: `i` iterates over small `usize` values (< 1000).
// - `i % 10000` is safe for u16: all values fit.
// - `elapsed.as_nanos()` -> f64 sacrifices precision beyond 2^52 ns (~52 days) but
//   total run time is ~0.04s, so the mantissa is more than sufficient.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
fn bench_peers_excluding(num_peers: usize, limit: usize, iterations: u64) -> f64 {
    use torrust_info_hash::InfoHash;
    let info_hash = InfoHash::default();
    let sender = Sender::default();
    let mut coordinator = Coordinator::new(&info_hash, 0, sender);

    // Reuse a single runtime for setup (creating one per peer is slow but outside the timed section)
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Populate swarm
    for i in 0..num_peers {
        let peer = make_peer((i % 254) as u8 + 1, 6881 + (i % 10000) as u16, (i % 255) as u8);
        rt.block_on(coordinator.handle_announcement(&peer));
    }

    let requesting_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 254)), 6999).into();

    // Warm up
    for _ in 0..1000 {
        black_box(coordinator.peers_excluding(&requesting_addr, Some(limit)));
    }

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(coordinator.peers_excluding(&requesting_addr, Some(limit)));
    }
    let elapsed = start.elapsed();
    elapsed.as_nanos() as f64 / iterations as f64
}

fn main() {
    let iterations = 100_000;

    println!("=== Baseline: Coordinator::peers_excluding ===");
    println!("iterations={iterations}");

    for num_peers in [10, 74, 100, 500, 1000] {
        let ns = bench_peers_excluding(num_peers, 74, iterations);
        let per_peer = ns / f64::from(u32::try_from(num_peers).expect("num_peers fits in u32"));
        println!("{num_peers:>4} peers: {ns:>10.2} ns/iter  ({per_peer:.2} ns/peer)");
    }

    // Memory estimate
    println!();
    println!("=== Memory per peer ===");
    println!("Peer struct:        {} bytes", std::mem::size_of::<Peer>());
    println!("Arc<Peer>:          {} bytes", std::mem::size_of::<std::sync::Arc<Peer>>());
    println!("SocketAddr:         {} bytes", std::mem::size_of::<SocketAddr>());
    println!("PeerId:             {} bytes", std::mem::size_of::<PeerId>());
    println!(
        "CompactPeer (est):  {} bytes (PeerId + SocketAddr)",
        std::mem::size_of::<PeerId>() + std::mem::size_of::<SocketAddr>()
    );
    println!(
        "Vec<Arc<Peer>>(74): {} bytes",
        std::mem::size_of::<Vec<std::sync::Arc<Peer>>>() + 74 * std::mem::size_of::<std::sync::Arc<Peer>>()
    );
}
