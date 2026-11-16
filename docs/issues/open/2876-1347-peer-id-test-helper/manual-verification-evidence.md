---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md
last-updated-utc: 2026-11-16 16:29
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16 15:59
- Artifact under test: `develop` at `c8b1477ad` (the merge of #2878) for the baselines (V0, V1), and
  branch `2876-peer-id-test-helper` for V1's final run, V2 and V3, as each section says.
- Operating system / environment: Linux 7.0.0-43-generic; stable Rust toolchain, rustc 1.99.1,
  cargo 1.99.1.
- Prerequisites and setup performed: none beyond a checkout of `develop`.

## Verification Processes

### V1 - Production builds need no test support (M1, baseline)

- Goal: show that no library or binary target of the workspace uses `Id::new`, `peer::fixture`
  (`PeerBuilder`) or the `Default` impls of `Peer` and `Id`.
- Initial state: clean checkout of `develop` at `c8b1477ad`.
- Status: `DONE` (baseline above; final run on the finished code below)

#### Steps Performed

The steps ran in this order in one shell, each after the previous one finished:

1. 15:59:03.559 - 15:59:03.737 UTC: throwaway gate in `packages/primitives/src/peer.rs`:
   `#[cfg(test)]` added above `impl Id {` (the block that holds `new`, line 367) and above
   `pub mod fixture {`. `git diff -U0` showed exactly the two added `#[cfg(test)]` lines.
2. 15:59:03.757 - 15:59:04.680 UTC: `cargo check --workspace --lib --bins` (stable Rust toolchain,
   cargo 1.99.1).
3. 15:59:04.691 UTC: `git checkout -- packages/primitives/src/peer.rs`, then
   `git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0 after the restore.

A first run of the same three steps a minute earlier (without the timestamps) also exited 0; it did
the full compile, so the timed run above finished quickly from the build cache.

#### Observed Result

```text
exit=0 warnings=0
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.55s
```

#### Conclusion

Met (baseline). With `Id::new` and the fixture module (which holds the `Default` impls) compiled
only under `cfg(test)`, every library and binary target of the workspace still compiles, with no
warnings. Code outside this workspace that depends on the published 3.0.0 is not covered.

#### Final Run (T9)

On the finished branch (head commit with subject `docs(adrs): [#2876] amend the test-support ADR
for shared fixtures such as peer::fixture`), with no throwaway change: `Id::new` no longer exists
and `peer::fixture` is behind `testing`, so a plain check is the proof. Run at 16:27 UTC, in one
shell, after `linter all`:

```text
M1 cargo check --workspace --lib --bins exit=0 warnings=0
primitives no-features check exit=0
```

(`cargo check -p torrust-tracker-primitives`, no features, is the second line.) Met: no library or
binary target of the workspace needs `Id::new`, the fixture or its `Default` impls.

### V0 - Reproduction: today's test ids name no client (M3, baseline)

- Goal: show that the ids the fixture and `Id::new` make today are not ids a real client sends.
- Initial state: `develop` at `c8b1477ad`.
- Status: `DONE` (baseline half of M3; the comparison with the new helper's ids follows in V3)

#### Steps Performed

1. Created a temporary, untracked integration test `packages/primitives/tests/scratch_2876.rs`
   (the package has no `tests/` folder; it was created for this run) that prints
   `to_hex_string()` and `get_client_name()` for `Id::default()`, the ids of
   `PeerBuilder::seeder()` and `PeerBuilder::leecher()`, `Id::new(1)`, `Id::new(-1)`, and, as the
   control, `-qB4650-abcdefghijkl` (the id #2824's tests use).
2. `cargo test -q -p torrust-tracker-primitives --test scratch_2876 -- --nocapture` (stable Rust
   toolchain, cargo 1.99.1), exit 0.
3. `rm -r packages/primitives/tests`; `git status -s` was empty afterwards.

#### Observed Result

```text
Id::default()            hex=0x2d71423030303030303030303030303030303030 client=None
PeerBuilder::seeder()    hex=0x2d71423030303030303030303030303030303031 client=None
PeerBuilder::leecher()   hex=0x2d71423030303030303030303030303030303032 client=None
Id::new(1)               hex=0x0000000001000000000000000000000000000000 client=None
Id::new(-1)              hex=0x00000000ffffffffffffffffffffffffffffffff client=None
-qB4650-abcdefghijkl     hex=0x2d7142343635302d6162636465666768696a6b6c client=Some("qBittorrent")
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

The finding is reproduced: the three fixture ids (`-qB000...`) and the `Id::new` ids give
`get_client_name() == None`; the Azureus-style control id gives `Some("qBittorrent")`.

### V2 - The release tracker carries no `testing` feature of `primitives` (M2)

- Goal: show that, with `peer::fixture` behind the new `testing` feature (T5), the shipped
  `torrust-tracker` binary's dependency tree does not enable it.
- Initial state: branch `2876-peer-id-test-helper` with the T5 change (feature, self
  dev-dependency, eight consumer `[dev-dependencies]` entries), uncommitted at the time of the run.
- Status: `DONE`

#### Steps Performed

1. `cargo run -q -p shipped-features-check -- --package torrust-tracker` (stable Rust toolchain,
   cargo 1.99.1).
2. `cargo tree -p torrust-tracker -e normal,build,features -i torrust-tracker-primitives`, then
   `grep -c 'feature "testing"'` on its output.

#### Observed Result

```text
shipped-features exit=0
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
tree exit=0
0
```

The tree's first lines show only the `default` feature:

```text
torrust-tracker-primitives v3.1.0 (.../packages/primitives)
└── torrust-tracker-primitives feature "default"
    ├── torrust-tracker v3.0.0-develop (...)
```

#### Conclusion

Met. The check exits 0 and the tree has no `testing` feature of `primitives`.

Repeated on the finished branch in T9 (16:27 UTC): `M2 exit=0` with the same `checked` message, and
`grep -c 'feature "testing"'` on the tree gave `0`.

### V3 - Realistic ids (M3)

- Goal: show that the fixture's ids and the helper's ids name a client, unlike the ids of V0.
- Initial state: branch `2876-peer-id-test-helper` with the T6 change (the fixture ids built with
  `qbittorrent_peer_id`), uncommitted at the time of the run.
- Status: `DONE`

#### Steps Performed

1. Created a temporary, untracked integration test `packages/primitives/tests/scratch_2876.rs` that
   prints the bytes and `get_client_name()` of `Id::default()`, `Peer::default()`, the ids of
   `PeerBuilder::seeder()` and `PeerBuilder::leecher()`, and `qbittorrent_peer_id` for 1, 1000 and
   `u32::MAX`.
2. `cargo test -q -p torrust-tracker-primitives --test scratch_2876 -- --nocapture` (stable Rust
   toolchain, cargo 1.99.1), exit 0. The self dev-dependency turns `testing` on.
3. `rm -r packages/primitives/tests`; `git status -s` then listed only the T6 change to
   `packages/primitives/src/peer.rs`.

#### Observed Result

```text
Id::default()                  bytes=-qB4650-000000000000 client=Some("qBittorrent")
Peer::default()                bytes=-qB4650-000000000000 client=Some("qBittorrent")
PeerBuilder::seeder()          bytes=-qB4650-000000000001 client=Some("qBittorrent")
PeerBuilder::leecher()         bytes=-qB4650-000000000002 client=Some("qBittorrent")
qbittorrent_peer_id(1)         bytes=-qB4650-000000000001 client=Some("qBittorrent")
qbittorrent_peer_id(1000)      bytes=-qB4650-000000001000 client=Some("qBittorrent")
qbittorrent_peer_id(u32::MAX)  bytes=-qB4650-004294967295 client=Some("qBittorrent")
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

Met. Every id names qBittorrent; in V0 the old fixture ids and the `Id::new` ids gave `None`.

## Failures and Follow-up

None so far.
