---
doc-type: file-test-plan
issue: 2876
package: torrust-tracker-primitives
target-file: packages/primitives/src/peer.rs
status: done
last-updated-utc: "2026-11-16 16:09"
---

# `peer::fixture::qbittorrent_peer_id` Test Plan (T2)

The realistic-id helper decided in Q1 (in `peer::fixture`) and Q4 (`u32` input).

## The Helper

```rust
pub const fn qbittorrent_peer_id(number: u32) -> Id
```

It returns the Azureus-style prefix `-qB4650-` (qBittorrent 4.6.5, the id #2824's tests use)
followed by `number` as 12 zero-padded decimal digits: `qbittorrent_peer_id(42)` is
`-qB4650-000000000042`. `u32::MAX` needs 10 digits, so every input fits.

- It returns `Id`, like `Id::new` did, so the callers keep passing `&id` to
  `PeerBuilder::with_peer_id(&PeerId)` through `Deref`.
- It is a `const fn`, so the fixture's `const fn` builders (`seeder`, `leecher`) can use it for
  their own ids (Q6, T6).

## Behaviors and Tests

All three are unit tests in the `peer::test::qbittorrent_peer_id` module of `peer.rs`; the helper
decides each behavior itself.

| Behavior | Test |
| --- | --- |
| Exact format: prefix, then 12 zero-padded decimal digits | `it_should_write_the_number_as_twelve_zero_padded_decimal_digits_after_the_qbittorrent_prefix` (0, 42, `u32::MAX`) |
| The client registry knows every id as qBittorrent | `it_should_make_ids_that_the_client_registry_knows_as_qbittorrent` (quickcheck, any `u32`) |
| Distinct numbers give distinct ids | `it_should_make_distinct_ids_from_distinct_numbers` (quickcheck, any two different `u32`) |

## Red, Green and Mutations

Stable Rust toolchain, cargo 1.99.1, `cargo test -q -p torrust-tracker-primitives --lib qbittorrent_peer_id`.

- **Red 1:** tests written first, no helper: `error[E0432]: unresolved import
  crate::peer::fixture::qbittorrent_peer_id`.
- **Red 2:** a stub that returned `Id::new(number)` (today's ids): the format test failed
  (`peer id made from 0: "\0\0..."`) and the client test failed (`[quickcheck] TEST FAILED.
  Arguments: (0)`); 1 passed, 2 failed. The distinctness test passed: `Id::new` is distinct too, so
  its guard is shown by mutation M3.
- **Green:** 3 passed. The whole package: 111 passed (108 before).

Each mutation was applied to the working tree, checked to compile (no `error[` line; the only
`error:` line is cargo's "test failed"), and the file was restored with `cmp` against a saved copy.

| Mutation | Breaks | Failed tests |
| --- | --- | --- |
| M1: digits written from byte 8 onwards, least significant first (`42` gives `-qB4650-240000000000`) | the format only; still distinct and still qBittorrent | format |
| M2: prefix `-ZZ4650-` | the client name (and so the exact bytes) | client, format |
| M3: digit loop never runs (every number gives `-qB4650-000000000000`) | distinctness (and the bytes of 42 and `u32::MAX`) | distinct, format |

A first run of M1 did not compile (it left `PEER_ID_BYTES_LEN` unused, which `-D warnings`
rejects); it was rewritten to keep the constant in use and re-run, with the result above.

## Prose-First AAA Review

Format test:

- Arrange: three numbers that differ in how they are written: zero (no digit to write, all
  padding), an ordinary number, and the widest `u32` (ten digits); each with the exact id expected,
  written out by hand.
- Act: build the id from the number.
- Assert: the id is exactly the expected bytes; on failure the message shows the number and the id
  made.

The table did not say why these three numbers; a short comment on each case now does, and
`// Arrange`, `// Act`, `// Assert` mark the sections. The expected ids are literals, not derived
from the helper.

Property tests: each is one expression with the Act and the expected result in view (`Some("qBittorrent")`;
`!=` on the bytes, with equal inputs discarded). Prose added nothing, so none was kept.

Smells checked: no complex Arrange, no hidden Act, no production-derived expectation. The format
test asserts in a loop, but every assertion checks the same behavior and its message names the case.

## Maintainer Design Review

Approved by the maintainer in chat on 2026-11-16 ("Approve, continue"): the name
`qbittorrent_peer_id`, the signature `const fn(u32) -> Id` and the `-qB4650-` + 12-digit format,
before the callers move to it (T3).

## Caller Migration (T3)

`torrent-repository-benchmarking` moved from `peer::Id::new` to the helper (Q4 (b)):

- `tests/common/torrent_peer_builder.rs`: `a_completed_peer` and `a_started_peer` take a `u32` and
  build the id with `qbittorrent_peer_id`. A new `pub const OTHER_PEER: u32 = 1000` names the peer
  that no scenario makes (the scenarios number their peers from 0 to 407).
- `tests/entry/mod.rs`: the three `-1` arguments are `OTHER_PEER`; the scrape-limit loop builds its
  75 ids with `qbittorrent_peer_id(peer_number)`.
- `tests/repository/mod.rs`: the `-1` argument is `OTHER_PEER`; the two fixtures iterate over
  `0..408_u32` and convert to `i32` only for `InfoHash::from(&i32)` and `write_i32`, so the info
  hashes are the same as before.

`grep -rn 'peer::Id::new' --include=*.rs packages src tests` finds no caller left.
`cargo test -q -p torrust-tracker-torrent-repository-benchmarking` (stable Rust toolchain, cargo
1.99.1): 17 and 1,468 passed, the same tests as before the change (no test added or removed), and
`cargo clippy -p torrust-tracker-torrent-repository-benchmarking --all-targets --all-features -- -D warnings`
is clean.

Sentinel check: with `OTHER_PEER` changed to `1` (a number the scenarios make), 10 cases failed,
all of `entry::it_should_remove_inactive_peers_beyond_cutoff` for the `started` and `three`
scenarios (the ones that make peer 1) over the five entry types; 1,458 passed. The constant was
restored to `1000`.

## `Id::new` Removal (T4)

`Id::new` is removed (Q3 (a)). The doc comment of `peer::Id` now has two doctests, following the
`write-unit-test` rule for compile-time guarantees:

- a companion that compiles: `Id::from(PeerId(*b"-qB4650-000000000042"))`, whose client name is
  `Some("qBittorrent")`;
- `compile_fail,E0599`: `Id::new(1)`.

`cargo test -q -p torrust-tracker-primitives` (stable Rust toolchain, cargo 1.99.1): 111 unit tests
and the doctests pass; `cargo +nightly test -q -p torrust-tracker-primitives --doc` (nightly Rust
toolchain, which checks the error code) passes too.

Mutation: an `Id::new(number: i32)` put back into `impl Id`. With both `cargo test --doc` and
`cargo +nightly test --doc`, `peer::Id (line 359)` failed with "Test compiled successfully, but
it's marked `compile_fail`"; 3 passed, 1 failed. The file was restored (`cmp` against a saved copy).

Removing a public item is a breaking change for the published `torrust-tracker-primitives` 3.0.0.
The crate already owes a major release (4.0.0, for #2454) in the #2583 plan; this removal adds to it.
