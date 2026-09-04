# Reproduction Attempt: Timing Leak in REST API Token Comparison

Evidence artifact for the parent issue. Records the independent reproduction step required by
`docs/security/vulnerability-remediation.md` (step 2). The result is **negative**: no
position-dependent timing signal was observable in-process on the test platform.

## Question

Does `configured_token.expose_secret() == token` (a `&str == &str` comparison) take
measurably longer when more leading bytes of the candidate match the secret, such that a
remote attacker could recover the token byte by byte?

## Method

A disposable Cargo project under `.tmp/timing-probe/` (git-ignored, deleted afterwards). Two
comparison functions were timed over 2 000 000 iterations per data point:

- **plain `==`** — the current implementation.
- **`subtle::ConstantTimeEq::ct_eq`** — the proposed replacement.

Each was fed a fixed secret and a candidate whose _first differing byte_ was placed at a chosen
offset (the candidate is the secret with one byte XOR-ed at that offset). The final data point
is an exact match. Both operands were passed through `std::hint::black_box` so the compiler
could not constant-fold the comparison.

Environment: x86-64 Linux, glibc, Rust stable (edition 2024), `--release` (`opt-level = 3`),
`subtle = "=2.6.1"` (identical version and checksum to the workspace lockfile).

### Probe source

```rust
use std::hint::black_box;
use std::time::Instant;

use subtle::ConstantTimeEq;

const ITERS: u32 = 2_000_000;
// 32-byte run; the 128-byte run repeated the pattern four times.
const SECRET: &[u8; 32] = b"0123456789abcdef0123456789abcdef";

fn candidate(prefix_ok: usize) -> [u8; 32] {
    let mut c = *SECRET;
    if prefix_ok < 32 {
        c[prefix_ok] ^= 0xff; // first wrong byte at `prefix_ok`
    }
    c
}

fn bench(label: &str, cmp: impl Fn(&[u8], &[u8]) -> bool) {
    println!("{label}");
    for prefix_ok in [0usize, 4, 8, 16, 24, 31, 32] {
        let candidate = candidate(prefix_ok);
        let mut hits = 0u32;
        let start = Instant::now();
        for _ in 0..ITERS {
            if cmp(black_box(&SECRET[..]), black_box(&candidate[..])) {
                hits += 1;
            }
        }
        let ns = start.elapsed().as_nanos() as f64 / f64::from(ITERS);
        println!("  correct-prefix={prefix_ok:>2}  {ns:6.2} ns/op  (matches={hits})");
    }
}

fn main() {
    bench("plain == (current code)", |a, b| a == b);
    bench("subtle ct_eq (proposed)", |a, b| a.ct_eq(b).into());
}
```

## Results

### 32-byte token

| First differing byte at | plain `==` (ns/op) | `subtle::ct_eq` (ns/op) |
| ----------------------- | ------------------ | ----------------------- |
| 0                       | 1.75               | 33.49                   |
| 4                       | 1.90               | 33.13                   |
| 8                       | 1.81               | 31.41                   |
| 16                      | 1.82               | 31.74                   |
| 24                      | 1.81               | 31.42                   |
| 31                      | 1.80               | 31.60                   |
| exact match             | 1.80               | 33.66                   |

### 128-byte token

| First differing byte at | plain `==` (ns/op) | `subtle::ct_eq` (ns/op) |
| ----------------------- | ------------------ | ----------------------- |
| 0                       | 1.74               | 121.72                  |
| 8                       | 1.78               | 121.49                  |
| 32                      | 1.74               | 122.67                  |
| 64                      | 2.25               | 120.84                  |
| 96                      | 1.95               | 120.46                  |
| 127                     | 2.59               | 121.21                  |
| exact match             | 2.09               | 122.27                  |

## Interpretation

- **No reproducible signal.** For plain `==`, the spread across all offsets is ≤ 0.15 ns at
  32 bytes and shows no monotonic trend at 128 bytes (offset 64 is slower than offset 96;
  the exact match is faster than offset 127). This is measurement noise, not a leak. On this
  platform glibc's `memcmp` compares inputs of this size with wide SIMD loads; there is no
  byte loop whose exit point could vary.
- **`subtle` is flat, as advertised**, and roughly 18× (32 B) to 60× (128 B) slower in
  absolute terms — about 30 ns and 120 ns respectively. Irrelevant for an admin endpoint.
- **Remote exploitability** would require distinguishing a sub-nanosecond in-process
  difference (which does not exist here) through tokio scheduling, HTTP parsing, optional
  TLS, and network jitter measured in tens of microseconds. Not credible on this platform.

## Why the negative result does not close the issue

The measurement describes one compiler, one libc, one CPU. Neither the C standard nor
glibc documents `memcmp` as constant-time; musl, other architectures, other glibc versions,
or a future LLVM lowering may behave differently without any change to our source. The
finding is therefore reclassified from "vulnerability" to **theoretical hardening gap**, and
fixed because the guarantee should come from a documented contract (`subtle`), not from an
observed characteristic of the current toolchain. See the parent spec, "Maintainer triage".

## Limitations

- Single machine, single run per data point, wall-clock timing. Adequate to answer "is there
  an obvious position-dependent signal?"; not a statistical proof of absence.
- Did not test musl, aarch64, or debug builds. Those are the reason the hardening is still
  applied.
- No network-level measurement was attempted; with no in-process signal there was nothing to
  amplify.
