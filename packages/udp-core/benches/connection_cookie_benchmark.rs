//! Benchmarks for issuing and validating UDP connection IDs (cookies).
//!
//! `make` runs for every connect request and `check` for every announce and
//! scrape request, so these functions sit on the UDP tracker's hot path.

use std::hint::black_box;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Range;
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use torrust_tracker_udp_core::connection_cookie::{check, gen_remote_fingerprint, make};
use torrust_tracker_udp_core::crypto::cookie_cipher::CookieCipher;

const ISSUE_AT: f64 = 1_000_000_000_f64;

fn client_fingerprint() -> u64 {
    gen_remote_fingerprint(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 7)), 6881))
}

fn valid_range() -> Range<f64> {
    (ISSUE_AT - 120.0)..(ISSUE_AT + 120.0)
}

fn bench_connection_cookie(c: &mut Criterion) {
    let cipher = CookieCipher::random();
    let fingerprint = client_fingerprint();
    let cookie = make(&cipher, fingerprint, ISSUE_AT).expect("the issue time should be a normal value");

    let mut group = c.benchmark_group("udp_tracker/connection_cookie");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));

    group.bench_function("make", |b| {
        b.iter(|| make(&cipher, black_box(fingerprint), black_box(ISSUE_AT)));
    });

    group.bench_function("check", |b| {
        b.iter(|| check(&cipher, black_box(&cookie), black_box(fingerprint), valid_range()));
    });

    group.finish();
}

criterion_group!(benches, bench_connection_cookie);
criterion_main!(benches);
