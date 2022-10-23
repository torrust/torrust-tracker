use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use criterion::{criterion_group, criterion_main, Criterion};
use torrust_tracker::udp::connection_cookie::{
    ConnectionCookie, EncryptedConnectionCookie, HashedConnectionCookie, WitnessConnectionCookie,
};

pub fn benchmark_hashed_connection_cookie(bench: &mut Criterion) {
    use HashedConnectionCookie as BenchConnectionCookie;

    let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let cookie = BenchConnectionCookie::make_connection_cookie(&remote_address);

    bench.bench_function("Make Hashed Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::make_connection_cookie(&remote_address);
        })
    });

    bench.bench_function("Check Hashed Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        })
    });
}

pub fn benchmark_witness_connection_cookie(bench: &mut Criterion) {
    use WitnessConnectionCookie as BenchConnectionCookie;

    let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let cookie = BenchConnectionCookie::make_connection_cookie(&remote_address);

    bench.bench_function("Make Witness Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::make_connection_cookie(&remote_address);
        })
    });

    bench.bench_function("Check Witness Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        })
    });
}

pub fn benchmark_encrypted_connection_cookie(bench: &mut Criterion) {
    use EncryptedConnectionCookie as BenchConnectionCookie;

    let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let cookie = BenchConnectionCookie::make_connection_cookie(&remote_address);

    bench.bench_function("Make Encrypted Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::make_connection_cookie(&remote_address);
        })
    });

    bench.bench_function("Check Encrypted Cookie", |b| {
        b.iter(|| {
            let _ = BenchConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        })
    });
}

criterion_group!(
    benches,
    benchmark_hashed_connection_cookie,
    benchmark_witness_connection_cookie,
    benchmark_encrypted_connection_cookie,
);
criterion_main!(benches);
