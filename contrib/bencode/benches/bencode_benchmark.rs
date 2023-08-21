use bencode::{BDecodeOpt, BencodeRef};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const B_NESTED_LISTS: &[u8; 100] =
    b"lllllllllllllllllllllllllllllllllllllllllllllllllleeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"; // cspell:disable-line
const MULTI_KB_BENCODE: &[u8; 30004] = include_bytes!("multi_kb.bencode");

fn bench_nested_lists(bencode: &[u8]) {
    BencodeRef::decode(bencode, BDecodeOpt::new(50, true, true)).unwrap();
}

fn bench_multi_kb_bencode(bencode: &[u8]) {
    BencodeRef::decode(bencode, BDecodeOpt::default()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bencode nested lists", |b| {
        b.iter(|| bench_nested_lists(black_box(B_NESTED_LISTS)));
    });

    c.bench_function("bencode multi kb", |b| {
        b.iter(|| bench_multi_kb_bencode(black_box(MULTI_KB_BENCODE)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
