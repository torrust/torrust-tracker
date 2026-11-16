---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md
last-updated-utc: 2026-11-16 15:59
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16 15:59
- Artifact under test: `develop` at `c8b1477ad` (the merge of #2878), before any change of this
  issue (T1 baseline).
- Operating system / environment: Linux 7.0.0-43-generic; stable Rust toolchain, rustc 1.99.1,
  cargo 1.99.1.
- Prerequisites and setup performed: none beyond a checkout of `develop`.

## Verification Processes

### V1 - Production builds need no test support (M1, baseline)

- Goal: show that no library or binary target of the workspace uses `Id::new`, `peer::fixture`
  (`PeerBuilder`) or the `Default` impls of `Peer` and `Id`.
- Initial state: clean checkout of `develop` at `c8b1477ad`.
- Status: `IN_PROGRESS` (the baseline is done; the final run on the finished code is pending, T9)

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

## Failures and Follow-up

None so far.
