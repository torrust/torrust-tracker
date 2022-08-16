use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use aquatic_udp_protocol::ConnectionId;
use criterion::{Criterion, criterion_group, criterion_main};
use torrust_tracker::{udp::connection_id::get_connection_id, protocol::clock::current_timestamp};

fn get_connection_id_old(current_time: u64, port: u16) -> ConnectionId {
    let time_i64 = (current_time / 3600) as i64;

    ConnectionId((time_i64 | port as i64) << 36)
}

pub fn benchmark_generate_id_with_time_and_port(bench: &mut Criterion) {
    let remote_address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 117);
    let current_time = current_timestamp();

    bench.bench_function("generate_id_with_time_and_port", |b| {
        b.iter(|| {
            // Inner closure, the actual test
            let _ = get_connection_id_old(current_time, remote_address.port());
        })
    });
}

pub fn benchmark_generate_id_with_hashed_time_and_ip_and_port_and_salt(bench: &mut Criterion) {
    let remote_address = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 117));
    let current_time = current_timestamp();

    bench.bench_function("generate_id_with_hashed_time_and_ip_and_port_and_salt", |b| {
        b.iter(|| {
            // Inner closure, the actual test
            let _ = get_connection_id(&remote_address, current_time);
        })
    });
}

criterion_group!(benches, benchmark_generate_id_with_time_and_port, benchmark_generate_id_with_hashed_time_and_ip_and_port_and_salt);
criterion_main!(benches);
