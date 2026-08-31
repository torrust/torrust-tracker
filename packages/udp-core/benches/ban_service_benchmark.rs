use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use torrust_tracker_udp_core::services::banning::BanService;

const COUNTER_LIMIT: u32 = 10;
const CARDINALITIES: [usize; 3] = [10, 1_000, 10_000];
const REPEATED_REQUESTS: usize = 10_000;

#[derive(Clone, Copy)]
enum AddressFamily {
    Ipv4,
    Ipv6,
}

impl AddressFamily {
    fn name(self) -> &'static str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
        }
    }
}

fn addresses(address_family: AddressFamily, cardinality: usize) -> Vec<IpAddr> {
    (0..cardinality)
        .map(|index| match address_family {
            AddressFamily::Ipv4 => {
                let third_octet = u8::try_from(index / 256).expect("benchmark IPv4 cardinality must fit in two octets");
                let fourth_octet = u8::try_from(index % 256).expect("IPv4 octet must fit in u8");

                IpAddr::V4(Ipv4Addr::new(198, 51, third_octet, fourth_octet))
            }
            AddressFamily::Ipv6 => {
                let suffix = u16::try_from(index).expect("benchmark IPv6 cardinality must fit in the address suffix");

                IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, suffix))
            }
        })
        .collect()
}

fn populate_ban_service(addresses: &[IpAddr]) -> BanService {
    let mut ban_service = BanService::new(COUNTER_LIMIT);

    for ip in addresses {
        ban_service.increase_counter(ip);
    }

    ban_service
}

fn bench_increase_counter(c: &mut Criterion) {
    let mut group = c.benchmark_group("udp_ban_service/increase_counter");

    for address_family in [AddressFamily::Ipv4, AddressFamily::Ipv6] {
        let addresses = addresses(address_family, CARDINALITIES[2]);
        let repeated_ip = addresses[0];

        group.bench_with_input(
            BenchmarkId::new("repeated", address_family.name()),
            &repeated_ip,
            |bench, ip| {
                bench.iter_batched(
                    || BanService::new(COUNTER_LIMIT),
                    |mut ban_service| {
                        for _ in 0..REPEATED_REQUESTS {
                            ban_service.increase_counter(black_box(ip));
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        group.bench_with_input(
            BenchmarkId::new("distinct", address_family.name()),
            &addresses,
            |bench, addresses| {
                bench.iter_batched(
                    || BanService::new(COUNTER_LIMIT),
                    |mut ban_service| {
                        for ip in addresses {
                            ban_service.increase_counter(black_box(ip));
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn bench_is_banned(c: &mut Criterion) {
    let mut group = c.benchmark_group("udp_ban_service/is_banned");

    for address_family in [AddressFamily::Ipv4, AddressFamily::Ipv6] {
        for cardinality in CARDINALITIES {
            let addresses = addresses(address_family, cardinality);
            let ip = addresses[0];

            for (scenario, counter_increments) in [
                ("below_threshold", COUNTER_LIMIT - 1),
                ("at_threshold", COUNTER_LIMIT),
                ("above_threshold", COUNTER_LIMIT + 1),
            ] {
                let mut current_service = populate_ban_service(&addresses);

                for _ in 1..counter_increments {
                    current_service.increase_counter(&ip);
                }

                group.bench_with_input(
                    BenchmarkId::new(format!("{}/{scenario}", address_family.name()), cardinality),
                    &(&current_service, ip),
                    |bench, (ban_service, ip)| bench.iter(|| black_box(ban_service.is_banned(black_box(ip)))),
                );
            }
        }
    }

    group.finish();
}

criterion_group!(benches, bench_increase_counter, bench_is_banned);
criterion_main!(benches);
