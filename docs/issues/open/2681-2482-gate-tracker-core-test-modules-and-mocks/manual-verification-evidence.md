---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
last-updated-utc: "2026-10-28 16:42"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). Every output below is the real output of the command shown; paths under the repository are replaced by `<repo>`, and output that only lists unrelated dependencies is reduced to the lines that matter.

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain (`cargo 1.99.1`), nightly for `cargo +nightly fmt`. No container build is needed for this issue.
- Baseline tree: `develop` at the merge commit of PR #2682 (`docs(issues): [#2681] specify gating the tracker-core test modules and mocks`), before any change of this issue.
- Test counts come from the `test result:` lines of the cargo runs; the raw logs were kept under `/tmp` during the work and are not part of the repository. Each cell lists the counts of the test binaries in the order cargo ran them: unit tests, the `tests/` binary of `tracker-core`, the doc tests, and the second `running N tests` block that follows the doc tests (it is there before and after the change). `+3i` means 3 ignored. Benches run in test mode under `--all-targets` and print no `test result:` line, so they are not counted.

## V0 - Facts re-verified before editing (task T1)

`develop` moved between the spec's audit (`883cc834c`) and the baseline. The diff over `packages`, `deny.toml` and `Cargo.toml` touched `tracker-core/src/announce_handler.rs`, `tracker-core/tests/integration.rs`, `http-core/src/services/announce.rs` and other packages, none of which declares a helper module, a `Mock*` type or the `mockall` dependency. Re-running the search for `MockDatabase` (which no longer exists) and the four `Mock*` types over `packages`, `src`, `tests` and `contrib` found the same files as the spec's table: `MockTorrentMetricsStore` in `tracker-core` (`scrape_handler.rs`, `statistics/event/handler.rs`), `http-core` (`services/announce.rs`, `services/scrape.rs`) and `udp-core` (`services/announce.rs`, `services/scrape.rs`); `MockAuthKeyStore` in `tracker-core/src/authentication/handler.rs` only; `MockWhitelistStore` and `MockSchemaMigrator` nowhere except their declarations and re-exports. The three `pub mod` declarations were still at `lib.rs:131`, `lib.rs:132` and `whitelist/mod.rs:23`.

Dependency audit against rule A.2 of the ADR: `mockall` was the only normal dependency of `tracker-core` that only test code uses. The other normal dependencies (`sqlx`, `tokio`, `serde`, `chrono`, `rand`, `derive_more`, `thiserror` and the workspace crates) are used by non-test code, so none moves. `secrecy`, `testcontainers` and `url` were already dev-dependencies.

## V1 - The default build has no test dependency (M1)

Before (a detached worktree at the baseline commit):

```text
$ cargo tree -p torrust-tracker-core -e normal --depth 1
torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
├── mockall v0.15.0
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── torrust-tracker-events v0.1.0 (<repo>/packages/events)
├── torrust-tracker-primitives v3.0.0 (<repo>/packages/primitives)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
└── tracing v0.1.44
```

After (this branch), with and without the feature:

```text
$ cargo tree -p torrust-tracker-core -e normal --depth 1
torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── torrust-tracker-events v0.1.0 (<repo>/packages/events)
├── torrust-tracker-primitives v3.0.0 (<repo>/packages/primitives)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
└── tracing v0.1.44
$ cargo tree -p torrust-tracker-core -e normal --depth 1 -F testing
torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
├── mockall v0.15.0
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── torrust-tracker-events v0.1.0 (<repo>/packages/events)
├── torrust-tracker-primitives v3.0.0 (<repo>/packages/primitives)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
└── tracing v0.1.44
```

`--depth 1` lists direct dependencies only: `mockall` is a direct normal dependency before the change and is absent from the default tree after it; V3 checks the full normal graph of the consumers. The other direct dependencies (`async-trait`, `chrono`, `derive_more`, `rand`, `serde`, `serde_json`, `sqlx`, `thiserror`, `tokio`, `tokio-util`, `torrust-clock`, `torrust-info-hash`, `torrust-located-error`, `torrust-metrics`) are unchanged and omitted above.

Builds, with and without the feature:

```text
$ cargo build -p torrust-tracker-core
    Finished `dev` profile [optimized + debuginfo] target(s) in 4.15s
$ cargo build -p torrust-tracker-core --features testing
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.82s
$ cargo build -p torrust-tracker-http-core
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.02s
$ cargo build -p torrust-tracker-udp-core
    Finished `dev` profile [optimized + debuginfo] target(s) in 6.46s
```

## V2 - Each package's tests run with a plain command (M2)

Counts per command, on the baseline tree and after the change. `--all-targets` does not run doc tests, so the doc-test counts come from the plain and `--doc` runs (finding F2 of PR #2682).

| Command | Before | After | Result |
| --- | --- | --- | --- |
| `cargo test -p torrust-tracker-core` | 155 / 12 / 16 / 2 | 155 / 12 / 16 / 2 | equal |
| `cargo test -p torrust-tracker-core --all-features` | 157 / 12 / 16 / 2 | 157 / 12 / 16 / 2 | equal |
| `cargo test -p torrust-tracker-core --all-targets --all-features` | 157 / 12 | 157 / 12 | equal |
| `cargo test -p torrust-tracker-core --doc --all-features` | 16 / 2 | 16 / 2 | equal |
| `cargo test -p torrust-tracker-http-core` | 191 / 1 / 2 | 191 / 1 / 2 | equal |
| `cargo test -p torrust-tracker-http-core --all-features` | 191 / 1 / 2 | 191 / 1 / 2 | equal |
| `cargo test -p torrust-tracker-http-core --all-targets --all-features` | 191 | 191 | equal |
| `cargo test -p torrust-tracker-http-core --doc --all-features` | 1 / 2 | 1 / 2 | equal |
| `cargo test -p torrust-tracker-udp-core` | 99 / 3+3i / 4 | 99 / 3+3i / 4 | equal |
| `cargo test -p torrust-tracker-udp-core --all-features` | 99 / 3+3i / 4 | 99 / 3+3i / 4 | equal |
| `cargo test -p torrust-tracker-udp-core --all-targets --all-features` | 99 | 99 | equal |
| `cargo test -p torrust-tracker-udp-core --doc --all-features` | 3+3i / 4 | 3+3i / 4 | equal |

All 12 commands exit 0 with no `FAILED` and no `error` in the logs. `cargo test -p torrust-tracker-core` without `--all-features` builds and runs the unit tests (155), the `tests/integration.rs` binary (12) and the doc tests with the `testing` feature on through the path-only self dev-dependency: no target is skipped and no `required-features` is used. With `--all-features` the unit tests are 157 because `db-compatibility-tests` adds two, before and after.

## V3 - A consumer still gets the mock, and only in its tests (M3)

Red first. With the mock gated and the feature and self dev-dependency added, but before the consumers' `[dev-dependencies]` entries, both consumers fail to compile their unit tests:

```text
$ cargo test -p torrust-tracker-http-core
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error: could not compile `torrust-tracker-http-core` (lib test) due to 2 previous errors
$ cargo test -p torrust-tracker-udp-core
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error: could not compile `torrust-tracker-udp-core` (lib test) due to 2 previous errors
```

Green after adding `torrust-tracker-core = { version = "0.1.0", path = "../tracker-core", features = ["testing"] }` to the `[dev-dependencies]` of `http-core` and `udp-core`: 191 and 99 unit tests pass (V2).

The normal graph of each consumer, before the change (detached worktree at the baseline commit) and after it:

```text
BEFORE
$ cargo tree -p torrust-tracker-http-core -e normal -i mockall
mockall v0.15.0
└── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
    └── torrust-tracker-http-core v0.1.0 (<repo>/packages/http-core)
$ cargo tree -p torrust-tracker-udp-core -e normal -i mockall
mockall v0.15.0
└── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
    └── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core)

AFTER
$ cargo tree -p torrust-tracker-http-core -e normal -i mockall
warning: nothing to print.
$ cargo tree -p torrust-tracker-udp-core -e normal -i mockall
warning: nothing to print.
```

With dev-dependencies included, `mockall` is reached through `torrust-tracker-core` and through each consumer's own dev-dependency, which is what the tests need:

```text
$ cargo tree -p torrust-tracker-http-core -e normal,dev -i mockall --depth 2
mockall v0.15.0
└── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
    └── torrust-tracker-http-core v0.1.0 (<repo>/packages/http-core)
    [dev-dependencies]
    └── torrust-tracker-http-core v0.1.0 (<repo>/packages/http-core)
[dev-dependencies]
└── torrust-tracker-http-core v0.1.0 (<repo>/packages/http-core)
```

## V4 - The shipped binary cannot get the feature (M4)

This is the first row where the `Shipped Features` job has a real `testing` feature to check. The same command that the CI job runs, on the real tree, then with two deliberate breaks (each reverted with `git checkout`):

```text
### green: real tree
$ cargo run -q -p shipped-features-check -- --package torrust-tracker
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
exit=0
### red 1: features = ["testing"] on the root [dependencies] entry of torrust-tracker-core
{"kind":"validation_error","message":"workspace package `torrust-tracker-core` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-core","feature":"testing","exit_code":1}
exit=1
### red 2: testing in the default chain of tracker-core (default = [ "testing" ])
{"kind":"validation_error","message":"workspace package `torrust-tracker-core` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-core","feature":"testing","exit_code":1}
exit=1
### green again after restoring
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
exit=0
```

The shipped-profile binary itself, built by `cargo build -p torrust-tracker --bin torrust-tracker` (no `--tests`, no `--all-features`), contains no trace of the mock or of `mockall`. A positive control shows the check can see them: the `rlib` of `tracker-core` built with `--features testing` has thousands of matching strings, and the default one has none.

```text
$ strings target/debug/torrust-tracker | grep -c MockTorrentMetricsStore
0
$ strings target/debug/torrust-tracker | grep -ci mockall
0
$ strings <rlib built with --features testing> | grep -c __mock_MockTorrentMetricsStore ; ... | grep -c mockall
3402 ; 1087
$ strings <rlib of the default build> | grep -c __mock_MockTorrentMetricsStore ; ... | grep -c mockall
0 ; 0
```

The native-tracker tests pass in both targets (`tests/common/native_tracker`, through the `cli-configuration` test target), with the test build's own binary and with the shipped-profile binary through `TORRUST_TRACKER_BINARY`:

```text
$ cargo test -p torrust-tracker --test cli-configuration
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
$ TORRUST_TRACKER_BINARY=<repo>/target/debug/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
```

## V5 - `cargo deny`, `cargo machete` and packaging (AC5)

The self dev-dependency makes `torrust-tracker-core` its own wrapper in the `deny.toml` ban list. Without the entry, `cargo deny check bans` fails; with it, it passes:

```text
$ cargo deny check bans   # with the committed deny.toml
bans ok
$ cargo deny check bans   # with "torrust-tracker-core" removed from its own wrappers list
error[banned]: crate 'torrust-tracker-core = 0.1.0' is explicitly banned
bans FAILED
$ cargo deny check bans   # restored
bans ok
$ cargo machete --with-metadata
cargo-machete didn't find any unused dependencies in this directory. Good job!
$ cargo +stable package --workspace --no-verify --allow-dirty
exit=0, 33 packages packaged
```

The packaged `torrust-tracker-core-0.1.0.crate` manifest keeps `testing = ["dep:mockall"]`, lists `mockall` as an optional normal dependency and as a dev-dependency, and has no self dev-dependency (`cargo package` strips the path-only entry):

```text
$ tar -xzOf target/package/torrust-tracker-core-0.1.0.crate torrust-tracker-core-0.1.0/Cargo.toml | grep -n "testing\|mockall"
42:testing = ["dep:mockall"]
68:[dependencies.mockall]
136:[dev-dependencies.mockall]
```

## V6 - The feature is visible only when asked (M6)

```text
$ cargo doc -p torrust-tracker-core --no-deps       # default features
default: Mock files: 0; peer_tests/test_helpers entries: 0
$ cargo doc -p torrust-tracker-core --no-deps --features testing
testing: Mock files: .../databases/traits/torrent_metrics/struct.MockTorrentMetricsStore.html (and its two __mock_ items)
peer_tests/test_helpers entries: 0
```

`cargo metadata` shows the feature and both `mockall` entries:

```text
features {'db-compatibility-tests': [], 'default': [], 'testing': ['dep:mockall']}
mockall None optional= True req= ^0 features= []
mockall dev optional= False req= ^0 features= []
torrust-tracker-core dev optional= False req= * features= ['testing']
```

## V7 - The coupling report reports the edges as designed (M5)

`cargo run -p workspace-coupling -- /tmp/<report>.md` (an output path outside the tracked reports), read for the `torrust-tracker-core` entries of the three packages:

```text
torrust-tracker-http-core [('torrust-tracker-core', 'normal'), ('torrust-tracker-core', 'dev')]
torrust-tracker-udp-core  [('torrust-tracker-core', 'normal'), ('torrust-tracker-core', 'dev')]
torrust-tracker-core      no `torrust-tracker-core` entry (the self edge is skipped);
                          other entries: configuration, events, primitives, swarm-coordination-registry [normal], test-helpers [dev]
```

The report lists only workspace packages, so `mockall` is absent by design; V6 shows it in `cargo metadata`.
