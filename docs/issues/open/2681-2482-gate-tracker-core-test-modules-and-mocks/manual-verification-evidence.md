---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
last-updated-utc: "2026-10-29 11:53"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). Every output below is the real output of the command shown; paths under the repository are replaced by `<repo>`, and output that only lists unrelated dependencies is reduced to the lines that matter.

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain (`cargo 1.99.1`), nightly for `cargo +nightly fmt`. No container build is needed for this issue.
- Date and time (UTC): V0 to V8 were produced on 2026-10-28 while the implementation was made: the baseline runs before the first implementation commit, the rest until the last one (the implementation commits were created from 16:27:49 to 16:50:50 UTC). Their steps were not stamped one by one and the per-step times cannot be recovered, so none is given for them. [V9](#v9---stamped-re-run-of-the-verification-sequence) is a fresh run of the same sequence with a UTC start and end stamp on every step (2026-10-29 11:14 to 11:15 UTC); it is the run that carries the order of the dependent steps.
- Artifact under test: the post-change tree is the branch head whose subject is `docs(issues): [#2681] record the closing-keyword lesson of the spec PR` when V9 ran, whose code (`packages`, `deny.toml`, `Cargo.toml`, `Cargo.lock`) is identical to the commit whose subject is `docs(tracker-core): [#2681] explain the testing mock, the self wrapper and the pub test modules` (V9 checks it with `git diff --quiet`). The baseline is the merge of PR #2682 (`docs(issues): [#2681] specify gating the tracker-core test modules and mocks`). For V0 to V8 the post-change tree was the working tree of this branch at the time, before the last code-affecting commit existed in some steps; V9 repeats them on the final code.
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

(Provenance: the commands below were not stamped; V9 repeats the whole sequence with UTC start and end stamps.) The native-tracker tests pass in both targets (`tests/common/native_tracker`, through the `cli-configuration` test target), with the test build's own binary and with the preserved shipped-profile binary through `TORRUST_TRACKER_BINARY`. The first version of this scenario pointed the override at `target/debug/torrust-tracker`, which the plain test run rebuilds, so it did not prove that the inspected binary was the one tested (finding F4 of PR #2688). It was redone with a copy made right after the shipped-profile build, whose checksum is compared before and after:

```text
$ cargo build -p torrust-tracker --bin torrust-tracker
$ cp target/debug/torrust-tracker /tmp/<dir>/torrust-tracker      # the preserved shipped-profile binary
$ sha256sum target/debug/torrust-tracker /tmp/<dir>/torrust-tracker
1653421cdeec9d69  target/debug/torrust-tracker                     # first 16 hex digits
1653421cdeec9d69  /tmp/<dir>/torrust-tracker
$ strings /tmp/<dir>/torrust-tracker | grep -c MockTorrentMetricsStore ; ... | grep -ci mockall
0 ; 0
$ cargo test -p torrust-tracker --test cli-configuration
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
$ sha256sum target/debug/torrust-tracker /tmp/<dir>/torrust-tracker
2f9d185fca66a2e5  target/debug/torrust-tracker                     # rebuilt by the test run: it differs
1653421cdeec9d69  /tmp/<dir>/torrust-tracker                       # the preserved copy is unchanged
$ TORRUST_TRACKER_BINARY=/tmp/<dir>/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
$ sha256sum /tmp/<dir>/torrust-tracker
1653421cdeec9d69
```

The test-profile binary that the plain run rebuilt also has no `MockTorrentMetricsStore` and no `mockall` string (measured with `strings`; the `shipped-features-check` in V4 covers the dependency tree).

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

## V8 - Pre-push checks and the independent pre-PR review

`./contrib/dev-tools/git/hooks/pre-push.sh` exits 0 on the implementation head (nightly format, nightly check, nightly docs for binaries and libraries, workspace packaging, all tests):

```text
[Step 1/6] Checking format with nightly toolchain ... PASS (1s)
[Step 2/6] Checking workspace with nightly toolchain ... PASS (0s)
[Step 3/6] Building documentation with nightly toolchain ... PASS (5s)
[Step 4/6] Building library documentation with nightly toolchain ... PASS (3s)
[Step 5/6] Checking that every workspace crate can be packaged ... PASS (39s)
[Step 6/6] Running all tests ... PASS (38s)
SUCCESS: All pre-push checks passed! (1m 26s)
```

An independent Task Reviewer reviewed the branch read-only before the pull request and returned PASS WITH FINDINGS (no blocker). It re-ran the builds, the three packages' plain tests, clippy, `cargo tree`, `shipped-features-check`, `cargo deny check bans`, `cargo machete` and `cargo doc` and reproduced AC1 to AC3 and AC6; it did not repeat the destructive red runs of V4 and V5 (its instruction was read-only). Findings and dispositions:

| # | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| 1 | Minor | Rule A.6 asks for the "outside semver" statement in the module documentation too, not only in the `Cargo.toml` comment | Fixed: a doc comment on the `MockTorrentMetricsStore` re-export, in `docs(tracker-core): [#2681] explain the testing mock, the self wrapper and the pub test modules` |
| 2 | Minor | The spec's AC1 changed "five" to "four" `Mock*` types, but the Progress Log said no deviation | Fixed by a new Progress Log entry (the log is append-only); T1 and the Risks section keep "five names" as the grep pattern, which included the non-existent `MockDatabase` |
| 3 | Minor | The Progress Log said "five signed commits"; there were six at that point | Corrected by the same new entry |
| 4 | Nit | `883cc834c` is cited in the docs | No change: it is a commit on `develop`, which no rewrite of this branch changes (the commit-id check of the `open-pull-request` skill passes) |
| 5 | Nit | The evidence has no raw logs | No change: the transcripts are the real output reduced to the relevant lines, as in earlier rows |
| 6 | Nit | The `deny.toml` self-wrapper is unexplained | Fixed: a comment in the same commit as finding 1 |
| 7 | Nit | `pub mod tests` is explained only in the inventory | Fixed: a comment above each `pub mod tests`, in the same commit |

## V9 - Stamped re-run of the verification sequence

Added after finding F7 of da2ce7's review of PR #2688, which asked for run provenance (start and end UTC stamps of the dependent steps, and the artifact identified by its Conventional Commit subject) instead of inventing times for V0 to V8. This is a fresh run on 2026-10-29, not a re-reading of the old one: a script ran every step in order against the tested head named in the Environment section (V1, V3, V4, V5, V6 and the V2 matrix, with the baseline as a detached worktree at the merge of PR #2682), temporarily editing a manifest for each red step and restoring it with `git checkout`, and printing `date -u` before and after each step. The script that ran is retained and printed in full under "Script" below (only the absolute path of the worktree is replaced by `<repo>`); each step label in the transcript is the `run "<label>"` call of that script, and the command behind it is on the same line. The comparisons behind the restore steps are `git diff --quiet -- Cargo.toml Cargo.lock packages deny.toml` (V3 and V4) and `git diff --quiet -- deny.toml` (V5), both printing `clean-exit=$?`, and the final line of the run is `git status --short -- Cargo.toml Cargo.lock packages deny.toml | wc -l`. Paths are replaced by `<repo>`, `<baseline>` and `<preserved-copy-dir>`, and output is filtered to the lines that matter (the exit code of every step is the real one).

Three reading notes:

- The order of the steps is the order of this transcript. The host's wall clock showed three sub-second backward steps between two consecutive stamps (11:14:35.784 to 11:14:35.467, 11:14:35.467 to 11:14:35.213 and 11:14:45.596 to 11:14:45.161), so a stamp is a measured time and not a monotonic clock. An earlier run of the same script, discarded for the same reason, produced the same results.
- The non-zero exits are the intended red steps: 101 for the two consumers without their dev-dependency (V3), 1 for the two `Shipped Features` breaks (V4) and 2 for `cargo deny` without the self wrapper (V5). Every restore step prints `clean-exit=0`, and the run ends with 0 changed manifest files.
- The shipped-profile binary was already built at this head, so the build step only reports it up to date; the checksum, `strings` and test steps are the real checks.

V2 counts per command, from this run (the unit, `tests/`, doc and second doc-test counts in cargo's order; `+3i` means 3 ignored). The before and after sides ran in this run, one after the other:

| Command | Before | After | Result |
| --- | --- | --- | --- |
| `cargo test -p torrust-tracker-core` | 155 / 12 / 16 / 2 (exit 0) | 155 / 12 / 16 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-core --all-features` | 157 / 12 / 16 / 2 (exit 0) | 157 / 12 / 16 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-core --all-targets --all-features` | 157 / 12 (exit 0) | 157 / 12 (exit 0) | equal |
| `cargo test -p torrust-tracker-core --doc --all-features` | 16 / 2 (exit 0) | 16 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-http-core` | 191 / 1 / 2 (exit 0) | 191 / 1 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-http-core --all-features` | 191 / 1 / 2 (exit 0) | 191 / 1 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-http-core --all-targets --all-features` | 191 (exit 0) | 191 (exit 0) | equal |
| `cargo test -p torrust-tracker-http-core --doc --all-features` | 1 / 2 (exit 0) | 1 / 2 (exit 0) | equal |
| `cargo test -p torrust-tracker-udp-core` | 99 / 3+3i / 4 (exit 0) | 99 / 3+3i / 4 (exit 0) | equal |
| `cargo test -p torrust-tracker-udp-core --all-features` | 99 / 3+3i / 4 (exit 0) | 99 / 3+3i / 4 (exit 0) | equal |
| `cargo test -p torrust-tracker-udp-core --all-targets --all-features` | 99 (exit 0) | 99 (exit 0) | equal |
| `cargo test -p torrust-tracker-udp-core --doc --all-features` | 3+3i / 4 (exit 0) | 3+3i / 4 (exit 0) | equal |

Transcript:

```text
RUN START 2026-10-29 11:14:26.459 UTC
tested head subject: docs(issues): [#2681] record the closing-keyword lesson of the spec PR
code commit subject: docs(tracker-core): [#2681] explain the testing mock, the self wrapper and the pub test modules
### no code change between the code commit and the tested head
start 2026-10-29 11:14:26.470 UTC
diff-exit=0
end 2026-10-29 11:14:26.478 UTC exit=0

baseline worktree subject: Merge torrust/torrust-tracker#2682: docs(issues): [#2681] specify gating the tracker-core test modules and mocks

### V1 before: direct normal dependencies of tracker-core
start 2026-10-29 11:14:26.606 UTC
torrust-tracker-core v0.1.0 (<baseline>/packages/tracker-core)
├── mockall v0.15.0
end 2026-10-29 11:14:26.727 UTC exit=0

### V1 after: direct normal dependencies of tracker-core
start 2026-10-29 11:14:26.730 UTC
torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
end 2026-10-29 11:14:26.849 UTC exit=0

### V1 after, with the feature: direct normal dependencies
start 2026-10-29 11:14:26.851 UTC
torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
├── mockall v0.15.0
end 2026-10-29 11:14:26.971 UTC exit=0

### V1 builds without and with the feature
start 2026-10-29 11:14:26.973 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.30s
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.28s
end 2026-10-29 11:14:29.630 UTC exit=0

### V3 before: normal graph of torrust-tracker-http-core (-i mockall)
start 2026-10-29 11:14:29.633 UTC
mockall v0.15.0
└── torrust-tracker-core v0.1.0 (<baseline>/packages/tracker-core)
    └── torrust-tracker-http-core v0.1.0 (<baseline>/packages/http-core)
end 2026-10-29 11:14:29.743 UTC exit=0

### V3 after: normal graph of torrust-tracker-http-core (-i mockall)
start 2026-10-29 11:14:29.746 UTC
warning: nothing to print.
To find dependencies that require specific target platforms, try to use option `--target all` first, and then narrow your search scope accordingly.
end 2026-10-29 11:14:29.858 UTC exit=0

### V3 before: normal graph of torrust-tracker-udp-core (-i mockall)
start 2026-10-29 11:14:29.860 UTC
mockall v0.15.0
└── torrust-tracker-core v0.1.0 (<baseline>/packages/tracker-core)
    └── torrust-tracker-udp-core v0.1.0 (<baseline>/packages/udp-core)
end 2026-10-29 11:14:29.973 UTC exit=0

### V3 after: normal graph of torrust-tracker-udp-core (-i mockall)
start 2026-10-29 11:14:29.975 UTC
warning: nothing to print.
To find dependencies that require specific target platforms, try to use option `--target all` first, and then narrow your search scope accordingly.
end 2026-10-29 11:14:30.866 UTC exit=0

### V3 red: torrust-tracker-http-core without its dev-dependency entry
start 2026-10-29 11:14:30.905 UTC
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error: could not compile `torrust-tracker-http-core` (lib test) due to 2 previous errors
end 2026-10-29 11:14:31.909 UTC exit=101

### V3 red: torrust-tracker-udp-core without its dev-dependency entry
start 2026-10-29 11:14:31.912 UTC
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error[E0432]: unresolved import `torrust_tracker_core::databases::MockTorrentMetricsStore`
error: could not compile `torrust-tracker-udp-core` (lib test) due to 2 previous errors
end 2026-10-29 11:14:33.571 UTC exit=101

### V3 restore: no change in manifests
start 2026-10-29 11:14:33.578 UTC
clean-exit=0
end 2026-10-29 11:14:33.585 UTC exit=0

### V3 green: torrust-tracker-http-core with its dev-dependency entry
start 2026-10-29 11:14:33.588 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:14:34.152 UTC exit=0

### V3 green: torrust-tracker-udp-core with its dev-dependency entry
start 2026-10-29 11:14:34.154 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:14:34.792 UTC exit=0

### V4 green: real tree
start 2026-10-29 11:14:34.794 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-10-29 11:14:35.784 UTC exit=0

### V4 red 1: features testing on the root [dependencies] entry
start 2026-10-29 11:14:35.467 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-core` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-core","feature":"testing","exit_code":1}
end 2026-10-29 11:14:35.213 UTC exit=1

### V4 red 2: testing in the default chain of tracker-core
start 2026-10-29 11:14:35.220 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-core` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-core","feature":"testing","exit_code":1}
end 2026-10-29 11:14:35.431 UTC exit=1

### V4 restore: no change in manifests
start 2026-10-29 11:14:35.438 UTC
clean-exit=0
end 2026-10-29 11:14:35.447 UTC exit=0

### V4 green again after the restore
start 2026-10-29 11:14:35.449 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-10-29 11:14:35.653 UTC exit=0

### V4 build the shipped-profile binary
start 2026-10-29 11:14:35.655 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 5.06s
end 2026-10-29 11:14:40.759 UTC exit=0

### V4 checksum of the build and of the preserved copy (first 16 hex digits)
start 2026-10-29 11:14:40.849 UTC
09eb4a76f8c7f158  target/debug/torrust-tracker
09eb4a76f8c7f158  <preserved-copy-dir>/torrust-tracker
end 2026-10-29 11:14:41.389 UTC exit=0

### V4 mock strings in the preserved copy
start 2026-10-29 11:14:41.416 UTC
MockTorrentMetricsStore=0 mockall=0
end 2026-10-29 11:14:42.384 UTC exit=0

### V4 plain cli-configuration tests
start 2026-10-29 11:14:42.386 UTC
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-10-29 11:14:43.626 UTC exit=0

### V4 checksums after the plain run
start 2026-10-29 11:14:43.629 UTC
2f9d185fca66a2e5  target/debug/torrust-tracker
09eb4a76f8c7f158  <preserved-copy-dir>/torrust-tracker
end 2026-10-29 11:14:43.813 UTC exit=0

### V4 cli-configuration tests with TORRUST_TRACKER_BINARY set to the preserved copy
start 2026-10-29 11:14:43.815 UTC
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-10-29 11:14:45.573 UTC exit=0

### V4 checksum of the preserved copy after the override run
start 2026-10-29 11:14:45.596 UTC
09eb4a76f8c7f158
end 2026-10-29 11:14:45.161 UTC exit=0

### V5 cargo deny check bans (committed deny.toml)
start 2026-10-29 11:14:45.163 UTC
bans ok
end 2026-10-29 11:14:45.738 UTC exit=0

### V5 red: cargo deny check bans without the self wrapper
start 2026-10-29 11:14:45.773 UTC
error[banned]: crate 'torrust-tracker-core = 0.1.0' is explicitly banned
bans FAILED
end 2026-10-29 11:14:46.367 UTC exit=2

### V5 restore: deny.toml has no change
start 2026-10-29 11:14:46.370 UTC
clean-exit=0
end 2026-10-29 11:14:46.381 UTC exit=0

### V5 cargo deny check bans after the restore
start 2026-10-29 11:14:46.384 UTC
bans ok
end 2026-10-29 11:14:46.970 UTC exit=0

### V5 cargo machete --with-metadata
start 2026-10-29 11:14:46.973 UTC
cargo-machete didn't find any unused dependencies in this directory. Good job!
end 2026-10-29 11:14:50.840 UTC exit=0

### V6 cargo doc default
start 2026-10-29 11:14:50.842 UTC
Mock-files=0 helper-entries=0
end 2026-10-29 11:14:51.467 UTC exit=0

### V6 cargo doc with the feature
start 2026-10-29 11:14:51.470 UTC
Mock-files=3 helper-entries=0
end 2026-10-29 11:14:52.229 UTC exit=0

### V2 cargo test -p torrust-tracker-core (plain, before)
start 2026-10-29 11:14:52.231 UTC
test result: ok. 155 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:14:54.535 UTC exit=0

### V2 cargo test -p torrust-tracker-core --all-features (before)
start 2026-10-29 11:14:54.537 UTC
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:14:56.680 UTC exit=0

### V2 cargo test -p torrust-tracker-core --all-targets --all-features (before)
start 2026-10-29 11:14:56.682 UTC
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
end 2026-10-29 11:14:57.245 UTC exit=0

### V2 cargo test -p torrust-tracker-core --doc --all-features (before)
start 2026-10-29 11:14:57.247 UTC
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
end 2026-10-29 11:14:57.960 UTC exit=0

### V2 cargo test -p torrust-tracker-core (plain, after)
start 2026-10-29 11:14:57.963 UTC
test result: ok. 155 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:14:59.848 UTC exit=0

### V2 cargo test -p torrust-tracker-core --all-features (after)
start 2026-10-29 11:14:59.850 UTC
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:01.951 UTC exit=0

### V2 cargo test -p torrust-tracker-core --all-targets --all-features (after)
start 2026-10-29 11:15:01.954 UTC
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
end 2026-10-29 11:15:02.522 UTC exit=0

### V2 cargo test -p torrust-tracker-core --doc --all-features (after)
start 2026-10-29 11:15:02.525 UTC
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
end 2026-10-29 11:15:03.205 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core (plain, before)
start 2026-10-29 11:15:03.208 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
end 2026-10-29 11:15:05.116 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --all-features (before)
start 2026-10-29 11:15:05.118 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:05.683 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --all-targets --all-features (before)
start 2026-10-29 11:15:05.686 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
end 2026-10-29 11:15:06.291 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --doc --all-features (before)
start 2026-10-29 11:15:06.294 UTC
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:06.807 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core (plain, after)
start 2026-10-29 11:15:06.809 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
end 2026-10-29 11:15:07.850 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --all-features (after)
start 2026-10-29 11:15:07.852 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
end 2026-10-29 11:15:08.414 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --all-targets --all-features (after)
start 2026-10-29 11:15:08.417 UTC
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
end 2026-10-29 11:15:09.254 UTC exit=0

### V2 cargo test -p torrust-tracker-http-core --doc --all-features (after)
start 2026-10-29 11:15:09.278 UTC
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:09.522 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core (plain, before)
start 2026-10-29 11:15:09.525 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:11.369 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --all-features (before)
start 2026-10-29 11:15:11.371 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:11.988 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --all-targets --all-features (before)
start 2026-10-29 11:15:11.990 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:12.547 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --doc --all-features (before)
start 2026-10-29 11:15:12.550 UTC
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:13.121 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core (plain, after)
start 2026-10-29 11:15:13.124 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:14.170 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --all-features (after)
start 2026-10-29 11:15:14.173 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:14.794 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --all-targets --all-features (after)
start 2026-10-29 11:15:14.796 UTC
test result: ok. 99 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-10-29 11:15:15.483 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-core --doc --all-features (after)
start 2026-10-29 11:15:15.485 UTC
test result: ok. 3 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
end 2026-10-29 11:15:16.480 UTC exit=0

RUN END 2026-10-29 11:15:16.767 UTC
final manifests status: 0 changed files
```

## Script

The script of V9, as it ran (the same file ran twice; the transcript above is the second run, the first having been discarded as the reading notes say). `run LABEL FILTER COMMAND...` prints the `date -u` start stamp, runs the command, prints the lines of its output that match the filter, and prints the end stamp and the command's exit code. The restore steps compare the working tree with `HEAD` through `git diff --quiet` on the named paths after a `git checkout -q --` of the files the red step edited; `$CODE` is the commit whose subject is `docs(tracker-core): [#2681] explain the testing mock, the self wrapper and the pub test modules`.

```bash
#!/usr/bin/env bash
set -u
R=<repo>
B=/tmp/row3/base
export CARGO_TARGET_DIR=$R/target
cd "$R" || exit 1

ts() { date -u '+%Y-%m-%d %H:%M:%S.%3N'; }

# run LABEL FILTER-REGEX COMMAND...: prints start/end stamps, the filtered output and the exit code
run() {
  local label="$1" pat="$2"
  shift 2
  echo "### $label"
  echo "start $(ts) UTC"
  local out rc
  out=$("$@" 2>&1)
  rc=$?
  echo "$out" | grep -E "$pat"
  echo "end $(ts) UTC exit=$rc"
  echo
}

echo "RUN START $(ts) UTC"
CODE=$(git log --format=%H -1 --grep='explain the testing mock, the self wrapper')
echo "tested head subject: $(git log -1 --format=%s)"
echo "code commit subject: $(git log -1 --format=%s "$CODE")"
run "no code change between the code commit and the tested head" '.' bash -c "git diff --quiet $CODE HEAD -- packages deny.toml Cargo.toml Cargo.lock; echo diff-exit=\$?"

git worktree add -q --detach "$B" 0edeb136c
echo "baseline worktree subject: $(git -C "$B" log -1 --format=%s)"
echo

# V1
run "V1 before: direct normal dependencies of tracker-core" 'mockall|torrust-tracker-core v' bash -c "cd $B && cargo tree -p torrust-tracker-core -e normal --depth 1"
run "V1 after: direct normal dependencies of tracker-core" 'mockall|torrust-tracker-core v' cargo tree -p torrust-tracker-core -e normal --depth 1
run "V1 after, with the feature: direct normal dependencies" 'mockall|torrust-tracker-core v' cargo tree -p torrust-tracker-core -e normal --depth 1 -F testing
run "V1 builds without and with the feature" 'Finished|error' bash -c "cargo build -p torrust-tracker-core && cargo build -p torrust-tracker-core --features testing"

# V3 normal graph of the consumers
for p in torrust-tracker-http-core torrust-tracker-udp-core; do
  run "V3 before: normal graph of $p (-i mockall)" '.' bash -c "cd $B && cargo tree -p $p -e normal -i mockall"
  run "V3 after: normal graph of $p (-i mockall)" '.' cargo tree -p "$p" -e normal -i mockall
done

# V3 red: consumers without their dev-dependency entry
sed -i '/^torrust-tracker-core = .*features = \[ "testing" \]/d' packages/http-core/Cargo.toml packages/udp-core/Cargo.toml
for p in torrust-tracker-http-core torrust-tracker-udp-core; do
  run "V3 red: $p without its dev-dependency entry" '^error' cargo test -p "$p" --no-run
done
git checkout -q -- packages/http-core/Cargo.toml packages/udp-core/Cargo.toml Cargo.lock
run "V3 restore: no change in manifests" '.' bash -c "git diff --quiet -- Cargo.toml Cargo.lock packages deny.toml; echo clean-exit=\$?"
for p in torrust-tracker-http-core torrust-tracker-udp-core; do
  run "V3 green: $p with its dev-dependency entry" '^test result' cargo test -p "$p"
done

# V4 shipped features red and restore
run "V4 green: real tree" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
sed -i 's#^torrust-tracker-core = { version = "0.1.0", path = "packages/tracker-core" }#torrust-tracker-core = { version = "0.1.0", path = "packages/tracker-core", features = ["testing"] }#' Cargo.toml
run "V4 red 1: features testing on the root [dependencies] entry" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
git checkout -q -- Cargo.toml Cargo.lock
sed -i 's#^default = \[  \]#default = [ "testing" ]#' packages/tracker-core/Cargo.toml
run "V4 red 2: testing in the default chain of tracker-core" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
git checkout -q -- packages/tracker-core/Cargo.toml Cargo.lock
run "V4 restore: no change in manifests" '.' bash -c "git diff --quiet -- Cargo.toml Cargo.lock packages deny.toml; echo clean-exit=\$?"
run "V4 green again after the restore" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker

# V4 preserved shipped-profile binary
run "V4 build the shipped-profile binary" 'Finished|error' cargo build -p torrust-tracker --bin torrust-tracker
mkdir -p /tmp/row3/shipped2 && cp target/debug/torrust-tracker /tmp/row3/shipped2/torrust-tracker
run "V4 checksum of the build and of the preserved copy (first 16 hex digits)" '.' bash -c "sha256sum target/debug/torrust-tracker /tmp/row3/shipped2/torrust-tracker | cut -c1-16,65-"
run "V4 mock strings in the preserved copy" '.' bash -c "echo MockTorrentMetricsStore=\$(strings /tmp/row3/shipped2/torrust-tracker | grep -c MockTorrentMetricsStore) mockall=\$(strings /tmp/row3/shipped2/torrust-tracker | grep -ci mockall)"
run "V4 plain cli-configuration tests" '^test result|^error' cargo test -p torrust-tracker --test cli-configuration
run "V4 checksums after the plain run" '.' bash -c "sha256sum target/debug/torrust-tracker /tmp/row3/shipped2/torrust-tracker | cut -c1-16,65-"
run "V4 cli-configuration tests with TORRUST_TRACKER_BINARY set to the preserved copy" '^test result|^error' env TORRUST_TRACKER_BINARY=/tmp/row3/shipped2/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
run "V4 checksum of the preserved copy after the override run" '.' bash -c "sha256sum /tmp/row3/shipped2/torrust-tracker | cut -c1-16"

# V5
run "V5 cargo deny check bans (committed deny.toml)" 'bans (ok|FAILED)' cargo deny check bans
cp deny.toml /tmp/row3/deny.keep2
python3 - <<'PY'
lines = open("deny.toml").read().split("\n")
out = []
inside = False
for l in lines:
    if 'crate = "torrust-tracker-core", wrappers' in l:
        inside = True
    if inside and l.strip() == '"torrust-tracker-core",':
        continue
    if inside and l.strip() == "] },":
        inside = False
    out.append(l)
open("deny.toml", "w").write("\n".join(out))
PY
run "V5 red: cargo deny check bans without the self wrapper" '^error\[|bans (ok|FAILED)' cargo deny check bans
cp /tmp/row3/deny.keep2 deny.toml
run "V5 restore: deny.toml has no change" '.' bash -c "git diff --quiet -- deny.toml; echo clean-exit=\$?"
run "V5 cargo deny check bans after the restore" 'bans (ok|FAILED)' cargo deny check bans
run "V5 cargo machete --with-metadata" 'didn.t find|found' cargo machete --with-metadata

# V6 docs
run "V6 cargo doc default" '.' bash -c "cargo doc -p torrust-tracker-core --no-deps >/dev/null 2>&1; D=target/doc/torrust_tracker_core; echo Mock-files=\$(find \$D -name '*Mock*' | wc -l) helper-entries=\$(grep -rl 'peer_tests\|test_helpers' \$D --include=*.html | wc -l)"
run "V6 cargo doc with the feature" '.' bash -c "cargo doc -p torrust-tracker-core --no-deps --features testing >/dev/null 2>&1; D=target/doc/torrust_tracker_core; echo Mock-files=\$(find \$D -name '*Mock*' | wc -l) helper-entries=\$(grep -rl 'peer_tests\|test_helpers' \$D --include=*.html | wc -l)"

# V2 matrix, before then after
mat() {
  local where="$1" pkg="$2" label="$3"
  shift 3
  run "V2 cargo test -p $pkg $label" '^test result|^error' bash -c "cd $where && cargo test -p $pkg $*"
}
for p in torrust-tracker-core torrust-tracker-http-core torrust-tracker-udp-core; do
  for w in "$B:before" "$R:after"; do
    dir=${w%%:*}
    tag=${w##*:}
    mat "$dir" "$p" "(plain, $tag)" ""
    mat "$dir" "$p" "--all-features ($tag)" "--all-features"
    mat "$dir" "$p" "--all-targets --all-features ($tag)" "--all-targets --all-features"
    mat "$dir" "$p" "--doc --all-features ($tag)" "--doc --all-features"
  done
done

git worktree remove --force "$B"
echo "RUN END $(ts) UTC"
echo "final manifests status: $(git status --short -- Cargo.toml Cargo.lock packages deny.toml | wc -l) changed files"
```
