---
doc-type: file-test-plan
issue: 2876
package: torrust-tracker-primitives
target-file: packages/primitives/src/peer.rs
status: done
last-updated-utc: "2026-11-16 17:30"
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
`!=` on the bytes, with equal inputs discarded). Prose added nothing, so none was kept, and the
`// Arrange`, `// Act`, `// Assert` markers were left out as redundant: a one-expression property has
no separate sections to mark. Only the format test, whose loop has three sections, carries them.

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

## `testing` Feature (T5)

Q2 and Q5: `peer::fixture` (with `PeerBuilder`, `qbittorrent_peer_id` and the `Default` impls of
`Peer` and `Id`) is compiled only with the new `testing` feature of `primitives`.

- `primitives/Cargo.toml`: `testing = [  ]`, not in `default`, with the rule A.6 comment; the
  path-only self dev-dependency (rule A.3). The module's doc comment says it is test support outside
  semver.
- `cargo check -p torrust-tracker-primitives` (no features) exits 0.
- Consumers. `cargo test --workspace --no-run` passed without any consumer entry, because the self
  dev-dependency unifies the feature on for every member (the ADR's V4 note). Building each of the
  34 members on its own (`cargo test -q -p <member> --no-run`) found the eight that need it, with
  `E0432` (`peer::fixture` unresolved) or `E0599` (`Peer::default` missing): `torrust-tracker`
  (`src/app.rs`), `axum-http-server`, `axum-rest-api-server`, `http-core` (`src/event.rs`),
  `swarm-coordination-registry`, `torrent-repository-benchmarking`, `udp-core` and `udp-server`.
  That is the six packages of the inventory plus the two that Q5 names. Each got
  `torrust-tracker-primitives = { version = "3.0.0", path = ..., features = [ "testing" ] }` under
  `[dev-dependencies]`, and all eight then built on their own. No `[dependencies]` entry enables
  `testing`.
- `cargo machete --with-metadata` first reported the self dev-dependency as unused (no code in
  `primitives` names the crate by its package name: it has no `tests/` folder) and warned that the
  ignored `quickcheck` is used (T2's test imports `quickcheck::TestResult`). The ignore list now
  names the self dev-dependency, with the reason, instead of `quickcheck`; machete is clean.
- `cargo package -p torrust-tracker-primitives --no-verify --allow-dirty` exits 0, and the packaged
  `Cargo.toml` has `testing = []` and no self dev-dependency (ADR criterion 5).
- `cargo test -q --workspace` (stable Rust toolchain, cargo 1.99.1): 4,087 passed, 0 failed.
- M2 (`manual-verification-evidence.md`, V2): the shipped-features check exits 0.

## Realistic Fixture Ids (T6)

Q6: the fixture's three ids are built with the helper: `Peer::default()` and `Id::default()` use
`qbittorrent_peer_id(0)`, `PeerBuilder::seeder()` uses 1 and `PeerBuilder::leecher()` uses 2 (the
same numbering as the old `-qB0000000000000000N` ids). The `const fn` builders take the bytes with
`qbittorrent_peer_id(n).data`.

Assertions on the old bytes, listed before the change: the spec expected some in the
`axum-http-server` and `axum-rest-api-server` tests. A search for the old literals and their hex
(`grep -rn -E 'qB0000000000000000[012]|2d7142303030...'`) finds 30 files outside `peer.rs`, but those
are ids written by hand, which Q6 leaves out of scope. To find the assertions that read bytes the
fixture makes, the three ids were changed first and `cargo test -q --workspace --no-fail-fast` was
run (stable Rust toolchain, cargo 1.99.1): 4,087 passed, 0 failed, the same count as after T5. So no
assertion depends on the fixture's id bytes; the tests that read them compare against values taken
from the same fixture. Nothing else needed to change.

M3 (`manual-verification-evidence.md`, V3): every fixture id now names qBittorrent.

## Fixture Tests (T7)

14 unit tests in `peer.rs`, for the fixture lines the baseline left uncovered (the `PeerBuilder`
setters and the two `Default` impls):

- `peer_fixture_defaults` (2): the exact default `Peer` (every field written out by hand) and the
  default `Id` (`-qB4650-000000000000`, named qBittorrent).
- `peer_builder` (12): `PeerBuilder::default()` gives `Peer::default()`; each of the ten setters
  (`with_peer_id`, `with_peer_addr`, `with_peer_address`, `updated_on`, `last_updated_on`,
  `with_bytes_left_to_download`, `with_no_bytes_left_to_download`, `with_event`,
  `with_event_started`, `with_event_completed`) changes its own field and nothing else; `into`
  gives the same peer as `build`.

`cargo test -q -p torrust-tracker-primitives --lib` (stable Rust toolchain, cargo 1.99.1): 125
passed (111 before). Clippy (`--all-targets --all-features -- -D warnings`) is clean. Coverage:
`coverage-evidence.md`, "After T7".

Mutations (applied, checked to compile, restored with `cmp`; `cargo test -q -p
torrust-tracker-primitives --lib peer_builder`):

| Mutation | Failed test |
| --- | --- |
| MB1: `with_event_started` sets `Completed` | `it_should_set_the_started_event_for_a_peer_that_had_completed` |
| MB2: `with_no_bytes_left_to_download` sets 1 | `it_should_set_no_bytes_left_to_download_for_a_peer_that_had_bytes_left` |
| MB3: `with_peer_addr` ignores its argument and sets the default address | `it_should_set_the_peer_address_from_a_reference` |
| MB4: `last_updated_on` also sets the event to `None` (the "nothing else" clause) | `it_should_set_the_last_update_time_with_last_updated_on` |

A first form of MB3 (dropping the assignment) did not compile, because `mut self` became unused
under `-D warnings`; it was rewritten as above and re-run.

Prose-first AAA review of the setter tests:

- Arrange: a peer whose field differs from the value the setter writes: the default peer, or the
  seeder (event `Completed`) for `with_event_started` and the leecher (10 bytes left) for
  `with_no_bytes_left_to_download`, where the default peer already has the target value.
- Act: one builder call, then `build`.
- Assert: the built peer equals the starting peer with only that field changed
  (`Peer { field, ..start }`).

The code says this without comments. Two tests start from a fixture whose relevant field is not
obvious, so they first assert the starting value, with a message, so that a change to the seeder or
the leecher fails there and not as a false pass. The struct-update expectation takes the other fields
from the fixture used by the Act (the skill's rule against duplicated fixture-derived expectations);
the default `Peer` itself is pinned by a test with every field written out. The duplicated setters
(`with_peer_addr` / `with_peer_address`, `updated_on` / `last_updated_on`, `build` / `into`) are
tested as they are; whether to merge them is not in this issue's scope.

Smells checked: no complex Arrange, no hidden Act, no production-derived expectation. Like the other
short tests in `peer.rs`, the sections are separated by blank lines; the `// Arrange`, `// Act`,
`// Assert` markers were left out as redundant prose for these one- to four-statement bodies (four in the
two tests that first assert their starting fixture; the skill's prose-first step 4), a decision
recorded here after review finding F1 of PR #2888 (count corrected after F3).
