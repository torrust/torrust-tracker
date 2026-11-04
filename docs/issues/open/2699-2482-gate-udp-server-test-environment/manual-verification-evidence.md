---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2699-2482-gate-udp-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-04 12:26"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). Every output below is the real output of the command shown; paths under the repository are replaced by `<repo>`, and output that only lists unrelated dependencies or packaging is reduced as each section says.

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain (`cargo 1.99.1`), `cargo +nightly` (`cargo 1.101.0-nightly`) for the all-targets check. No container build is needed: the `Containerfile` change of #2670 already builds the shipped binary without test features, and V4 checks the shipped-profile binary directly.
- Date and time (UTC): V1 to V6 and the matrix ran as one script on 2026-11-04 from 11:22:32 to 11:27:29 UTC; the examples run (V7) on 2026-11-04 from 11:27:29 to 11:27:35 UTC. An earlier run of the same scripts (10:59 to 11:13 UTC) on the previous version of the new example gave the same counts and exit codes; it is superseded by this one, after the independent pre-PR review asked for changes to that example, and its transcript is not kept. The sub-second part of the stamps is not monotonic in a few steps (an end stamp a few hundred milliseconds before its start stamp, for example in "V5 restore"), so the order of the steps is the order of the script, and only the seconds are meaningful.
- Artifact under test: the branch head whose subject is `refactor(udp-server): [#2699] follow the application's listener policy and loop error handling in the production-style example`, when the script started; the script checks that the code paths (`packages`, `src`, `tests`, `deny.toml`, `Cargo.toml`, `Cargo.lock`) have no change against `HEAD` (`git diff --quiet HEAD -- <paths>`, exit 0).
- Baseline tree: `develop` at the merge of PR #2700 (`docs(issues): [#2699] specify gating the udp-server test environment`), before any change of this issue, built in a detached worktree (`git worktree add --detach`) and removed at the end.
- Test counts come from the `test result:` lines of the cargo runs; the transcript below keeps only those lines. The raw logs were kept under `/tmp` during the work and are not part of the repository. Benches run in test mode under `--all-targets` and print no `test result:` line.

## Results

| Scenario | Result | Where |
| --- | --- | --- |
| M1 The production build has no test-only dependency | The direct normal dependencies of `udp-server` drop from 27 to 26; `torrust-tracker-swarm-coordination-registry` is no longer a direct dependency. It stays reachable transitively through `torrust-tracker-core` and `torrust-tracker-udp-core` (the `-i` tree below). With `-F testing` it is the only added direct dependency. `src/testing/` is absent from the default `cargo doc` output and present with the feature. | V1 |
| M2 Plain package tests | `cargo test -p torrust-tracker-udp-server` runs 233 + 16 + 1 (1 ignored) + 2 tests, equal to the baseline, with and without `--all-features`; `--all-targets --all-features` has one more empty test binary, the new example. | V2 |
| M3 The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` runs 15 + 8 tests, equal to the baseline. Without its dev-dependency feature the same command fails to compile (two `E0433` errors in the integration test of that package), so the feature entry is needed. | V2, V3 |
| M4 Shipped features | green on the real tree; red with `features = ["testing"]` on the root `[dependencies]` entry of `udp-server`; red with `default = [ "testing" ]`; green again after restoring (clean `git diff --quiet`). | V4 |
| M5 Dependents | `axum-rest-api-server` (106 + 58) and `rest-api-runtime-adapter` (3 + 0), which depend on `udp-server` through `[dependencies]`, have identical counts before and after, plain and with `--all-features`. | V2 |
| M6 Examples | Both build under `cargo build --examples` and `cargo test --examples`; both run under plain `cargo run` (they print `Listening on ...` until the timeout kills them); both answer a real UDP connect request and stop on SIGTERM (V7, whose commands are the second script below). The new example has no `use` line of `udp_server::testing` (`grep -c '^use .*udp_server::testing'` prints 0); the existing one has exactly one. | V6, V7 |
| M7 Deny, machete, packaging, nightly, bench | `cargo deny check bans` ok, and failing (`explicitly banned`) without the self wrapper; `cargo machete --with-metadata` finds nothing; `cargo +stable package --workspace --no-verify` exits 0; `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features` exits 0 (in the script it finished in 0.54 s because the tree was up to date; after a `touch` of the new example, on 2026-11-04 at 11:31 UTC, the same command printed `Checking torrust-tracker-udp-server`, finished in 0.46 s and exited 0); `cargo bench -p torrust-tracker-udp-server --no-run` exits 0; no `required-features` anywhere in the workspace manifests. | V5, V6 |
| M8 The shipped binary | The binary built by `cargo build -p torrust-tracker --bin torrust-tracker` has no trace of the UDP test environment (strings count 0 for both markers); the 35 `cli-configuration` tests pass with `TORRUST_TRACKER_BINARY` set to the preserved copy, whose checksum is unchanged by the plain run (the plain run rebuilt `target/debug/torrust-tracker`, whose checksum differs). | V4 |
| Container recipe (spec scenario M8) | With `torrust-cargo-chef` 0.1.78, `cargo chef prepare --external-only` then `cargo chef cook --tests --workspace --all-features` on this tree exits 101 with the CI message; with the fixed build (0.1.79) the skeleton has `testing = []` and `cook` exits 0. The `Containerfile` job with the new pin has not run yet (AC6 open). | V9 |

Dependency audit against rule A.2 of the ADR (done for the spec on 2026-11-03 and checked again by the "V1 before" and "V1 after" lists): `torrust-tracker-swarm-coordination-registry` was the only direct normal dependency of `udp-server` that only `src/testing/environment.rs` uses. Every other direct normal dependency is also used by non-test code, and `mockall`, `rand`, `criterion`, `torrust-tracker-test-helpers` are dev-dependencies already. The new production-style example imports the registry crate; it compiles because the self dev-dependency enables `testing` for the package's own targets, so no explicit dev-dependency on the registry crate was added (T4). `cargo build -p torrust-tracker-udp-server --examples` (V6) succeeded; that build runs with the package's dev-dependencies, so it shows the example compiles under the self dev-dependency, not that it would compile without the feature.

## V1 to V6 and the matrix - transcript

The script below ran once; its filtered output follows. "before" runs `cd <baseline worktree> && cargo ...`, "after" runs in this tree.

```text
RUN START 2026-11-04 11:22:32.452 UTC
tested head subject: refactor(udp-server): [#2699] follow the application's listener policy and loop error handling in the production-style example
### tree is clean for the code paths
start 2026-11-04 11:22:32.456 UTC
diff-exit=0
end 2026-11-04 11:22:32.464 UTC exit=0

baseline worktree subject: Merge torrust/torrust-tracker#2700: docs(issues): [#2699] specify gating the udp-server test environment

### V1 before: direct normal dependencies of udp-server
start 2026-11-04 11:22:32.604 UTC
torrust-tracker-udp-server v0.1.0 (/tmp/row4/base/packages/udp-server)
├── derive_more v2.1.1
├── futures v0.3.34
├── futures-util v0.3.34
├── ringbuf v0.6.0
├── serde v1.0.229
├── socket2 v0.6.5
├── thiserror v2.0.21
├── tokio v1.54.1
├── tokio-util v0.7.20
├── torrust-clock v3.0.0
├── torrust-info-hash v0.2.0
├── torrust-metrics v0.2.0
├── torrust-net-primitives v0.1.0
├── torrust-peer-id v0.1.0
├── torrust-server-lib v0.4.0
├── torrust-tracker-client-lib v0.1.0 (/tmp/row4/base/packages/tracker-client)
├── torrust-tracker-configuration v3.0.0 (/tmp/row4/base/packages/configuration)
├── torrust-tracker-core v0.1.0 (/tmp/row4/base/packages/tracker-core)
├── torrust-tracker-events v0.1.0 (/tmp/row4/base/packages/events)
├── torrust-tracker-primitives v3.0.0 (/tmp/row4/base/packages/primitives)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (/tmp/row4/base/packages/swarm-coordination-registry)
├── torrust-tracker-udp-core v0.1.0 (/tmp/row4/base/packages/udp-core)
├── torrust-tracker-udp-protocol v0.1.0 (/tmp/row4/base/packages/udp-protocol)
├── tracing v0.1.44
├── url v2.5.8
├── uuid v1.26.1
└── zerocopy v0.8.63
end 2026-11-04 11:22:32.728 UTC exit=0

### V1 after: direct normal dependencies of udp-server
start 2026-11-04 11:22:32.730 UTC
torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
├── derive_more v2.1.1
├── futures v0.3.34
├── futures-util v0.3.34
├── ringbuf v0.6.0
├── serde v1.0.229
├── socket2 v0.6.5
├── thiserror v2.0.21
├── tokio v1.54.1
├── tokio-util v0.7.20
├── torrust-clock v3.0.0
├── torrust-info-hash v0.2.0
├── torrust-metrics v0.2.0
├── torrust-net-primitives v0.1.0
├── torrust-peer-id v0.1.0
├── torrust-server-lib v0.4.0
├── torrust-tracker-client-lib v0.1.0 (<repo>/packages/tracker-client)
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
├── torrust-tracker-events v0.1.0 (<repo>/packages/events)
├── torrust-tracker-primitives v3.0.0 (<repo>/packages/primitives)
├── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core)
├── torrust-tracker-udp-protocol v0.1.0 (<repo>/packages/udp-protocol)
├── tracing v0.1.44
├── url v2.5.8
├── uuid v1.26.1
└── zerocopy v0.8.63
end 2026-11-04 11:22:32.857 UTC exit=0

### V1 after, with the feature: direct normal dependencies
start 2026-11-04 11:22:32.859 UTC
torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
end 2026-11-04 11:22:32.991 UTC exit=0

### V1 transitive: where the registry crate stays reachable from udp-server (after, no feature)
start 2026-11-04 11:22:32.993 UTC
torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
├── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
│   ├── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core)
│   │   └── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
│   └── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
└── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core) (*)
end 2026-11-04 11:22:33.132 UTC exit=0

### V1 builds without and with the feature
start 2026-11-04 11:22:33.134 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 3.68s
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.36s
end 2026-11-04 11:22:39.262 UTC exit=0

### V1 the module is not compiled without the feature (rustdoc item count)
start 2026-11-04 11:22:39.264 UTC
testing-dir-default=0
testing-dir-with-feature=1
end 2026-11-04 11:22:41.762 UTC exit=0

### V3 red: health check package without its dev-dependency feature
start 2026-11-04 11:22:41.804 UTC
error[E0433]: cannot find `testing` in `torrust_tracker_udp_server`
error[E0433]: cannot find `testing` in `torrust_tracker_udp_server`
error: could not compile `torrust-tracker-axum-health-check-api-server` (test "integration") due to 2 previous errors
end 2026-11-04 11:22:44.933 UTC exit=101

### V3 restore: no change in manifests
start 2026-11-04 11:22:44.939 UTC
clean-exit=0
end 2026-11-04 11:22:44.947 UTC exit=0

### V3 dev edge of the health check package to udp-server, with its features
start 2026-11-04 11:22:44.949 UTC
torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server) []
├── torrust-tracker-axum-http-server v0.1.0 (<repo>/packages/axum-http-server) []
├── torrust-tracker-axum-rest-api-server v0.1.0 (<repo>/packages/axum-rest-api-server) []
├── torrust-tracker-test-helpers v3.0.0 (<repo>/packages/test-helpers) []
└── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server) [default,testing]
end 2026-11-04 11:22:45.829 UTC exit=0

### V3 green: health check package with its dev-dependency feature
start 2026-11-04 11:22:45.853 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:22:49.406 UTC exit=0

### V4 green: real tree
start 2026-11-04 11:22:49.408 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-11-04 11:22:49.641 UTC exit=0

### V4 red 1: features testing on the root [dependencies] entry of udp-server
start 2026-11-04 11:22:49.655 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-udp-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-udp-server","feature":"testing","exit_code":1}
end 2026-11-04 11:22:49.861 UTC exit=1

### V4 red 2: testing in the default chain of udp-server
start 2026-11-04 11:22:49.870 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-udp-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-udp-server","feature":"testing","exit_code":1}
end 2026-11-04 11:22:50.113 UTC exit=1

### V4 restore: no change in manifests
start 2026-11-04 11:22:50.119 UTC
clean-exit=0
end 2026-11-04 11:22:50.126 UTC exit=0

### V4 green again after the restore
start 2026-11-04 11:22:50.128 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-11-04 11:22:50.348 UTC exit=0

### V4 build the shipped-profile binary
start 2026-11-04 11:22:50.350 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 5.59s
end 2026-11-04 11:22:55.981 UTC exit=0

### V4 checksum of the build and of the preserved copy (first 16 hex digits)
start 2026-11-04 11:22:56.798 UTC
7742439af594b43f  target/debug/torrust-tracker
7742439af594b43f  /tmp/row4/shipped/torrust-tracker
end 2026-11-04 11:22:56.282 UTC exit=0

### V4 test environment strings in the preserved copy
start 2026-11-04 11:22:56.285 UTC
environment=0 udp_server_testing_module=0
end 2026-11-04 11:22:57.644 UTC exit=0

### V4 plain cli-configuration tests
start 2026-11-04 11:22:57.646 UTC
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-11-04 11:23:04.200 UTC exit=0

### V4 checksums after the plain run
start 2026-11-04 11:23:04.202 UTC
c6452a5d5c87e3ef  target/debug/torrust-tracker
7742439af594b43f  /tmp/row4/shipped/torrust-tracker
end 2026-11-04 11:23:04.395 UTC exit=0

### V4 cli-configuration tests with TORRUST_TRACKER_BINARY set to the preserved copy
start 2026-11-04 11:23:04.398 UTC
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-11-04 11:23:05.634 UTC exit=0

### V4 checksum of the preserved copy after the override run
start 2026-11-04 11:23:05.637 UTC
7742439af594b43f
end 2026-11-04 11:23:05.740 UTC exit=0

### V5 cargo deny check bans (committed deny.toml)
start 2026-11-04 11:23:05.742 UTC
bans ok
end 2026-11-04 11:23:06.316 UTC exit=0

### V5 red: cargo deny check bans without the self wrapper
start 2026-11-04 11:23:06.328 UTC
error[banned]: crate 'torrust-tracker-udp-server = 0.1.0' is explicitly banned
bans FAILED
end 2026-11-04 11:23:06.897 UTC exit=2

### V5 restore: deny.toml has no change
start 2026-11-04 11:23:06.900 UTC
clean-exit=0
end 2026-11-04 11:23:06.910 UTC exit=0

### V5 cargo deny check bans after the restore
start 2026-11-04 11:23:06.912 UTC
bans ok
end 2026-11-04 11:23:07.472 UTC exit=0

### V5 cargo machete --with-metadata
start 2026-11-04 11:23:07.474 UTC
cargo-machete didn't find any unused dependencies in this directory. Good job!
end 2026-11-04 11:23:11.348 UTC exit=0

### V5 no required-features on any target of the workspace
start 2026-11-04 11:23:11.351 UTC
0
end 2026-11-04 11:23:11.366 UTC exit=0

### V5 cargo +stable package --workspace --no-verify
(34 "Packaged" lines and 9 license/homepage manifest warnings, all unrelated to this change, reduced to these counts)
start 2026-11-04 11:23:11.369 UTC
end 2026-11-04 11:23:40.471 UTC exit=0

### V6 import lines of the testing module: new example, then existing example
start 2026-11-04 11:23:40.473 UTC
0
1
end 2026-11-04 11:23:40.480 UTC exit=0

### V6 cargo build -p torrust-tracker-udp-server --examples
start 2026-11-04 11:23:40.483 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.94s
end 2026-11-04 11:23:41.470 UTC exit=0

### V6 cargo test -p torrust-tracker-udp-server --examples
start 2026-11-04 11:23:41.472 UTC
     Running unittests examples/udp_only_public_tracker.rs (target/debug/examples/udp_only_public_tracker-9a80d3aab20954cf)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests examples/udp_only_public_tracker_production_style.rs (target/debug/examples/udp_only_public_tracker_production_style-9ce169b9b31ef80c)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:23:41.874 UTC exit=0

### V6 plain cargo run of the production-style example (killed by timeout after the listening line)
start 2026-11-04 11:23:41.876 UTC
Listening on 127.0.0.1:33512
end 2026-11-04 11:24:41.889 UTC exit=124

### V6 plain cargo run of the testing-style example (killed by timeout after the listening line)
start 2026-11-04 11:24:41.891 UTC
Listening on 127.0.0.1:41123
end 2026-11-04 11:25:41.902 UTC exit=124

### V6 bench builds
start 2026-11-04 11:25:41.905 UTC
    Finished `bench` profile [optimized + debuginfo] target(s) in 0.27s
end 2026-11-04 11:25:42.206 UTC exit=0

### V6 nightly check of every target with all features
start 2026-11-04 11:25:42.209 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.54s
end 2026-11-04 11:25:42.816 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server (plain, before)
start 2026-11-04 11:25:42.818 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.22s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
end 2026-11-04 11:25:52.860 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-features (before)
start 2026-11-04 11:25:52.862 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.20s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
end 2026-11-04 11:26:00.930 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-targets --all-features (before)
start 2026-11-04 11:26:00.933 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.24s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:08.995 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server (plain, after)
start 2026-11-04 11:26:08.997 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.15s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-11-04 11:26:18.175 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-features (after)
start 2026-11-04 11:26:18.177 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.19s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
end 2026-11-04 11:26:26.183 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-targets --all-features (after)
start 2026-11-04 11:26:26.186 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.18s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.09s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:34.482 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server (plain, before)
start 2026-11-04 11:26:34.506 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.03s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:40.260 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-features (before)
start 2026-11-04 11:26:40.262 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:44.516 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-targets --all-features (before)
start 2026-11-04 11:26:44.518 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
end 2026-11-04 11:26:48.722 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server (plain, after)
start 2026-11-04 11:26:48.724 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:53.484 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-features (after)
start 2026-11-04 11:26:53.487 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:26:57.747 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-targets --all-features (after)
start 2026-11-04 11:26:57.749 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
end 2026-11-04 11:27:01.951 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server (plain, before)
start 2026-11-04 11:27:01.953 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.30s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:07.250 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-features (before)
start 2026-11-04 11:27:07.252 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.15s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.29s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:11.823 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-targets --all-features (before)
start 2026-11-04 11:27:11.106 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.12s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.29s
end 2026-11-04 11:27:14.627 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server (plain, after)
start 2026-11-04 11:27:14.630 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.15s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.27s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:19.169 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-features (after)
start 2026-11-04 11:27:19.172 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.15s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:22.895 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-targets --all-features (after)
start 2026-11-04 11:27:22.898 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.16s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.30s
end 2026-11-04 11:27:26.563 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter (plain, before)
start 2026-11-04 11:27:26.566 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:28.257 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-features (before)
start 2026-11-04 11:27:28.260 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:28.465 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-targets --all-features (before)
start 2026-11-04 11:27:28.468 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:28.620 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter (plain, after)
start 2026-11-04 11:27:28.623 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:29.113 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-features (after)
start 2026-11-04 11:27:29.115 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:29.325 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-targets --all-features (after)
start 2026-11-04 11:27:29.328 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:27:29.483 UTC exit=0

RUN END 2026-11-04 11:27:29.511 UTC
final manifests status: 0 changed files
```

## V7 - Both examples answer a real UDP request and stop on a signal

Each example is built, started in the background, sent a UDP connect request (BEP 15, connection id request, transaction id 12345) through a short Python snippet, then stopped with SIGTERM. A connect response has action 0, the same transaction id and 16 bytes.

```text
### example udp_only_public_tracker_production_style
start 2026-11-04 11:27:29 UTC
connect response: action=0 transaction_id=12345 length=16
exit=0
Listening on 127.0.0.1:40042
Press Ctrl-C (or send SIGTERM on Unix) to stop.

Received SIGTERM. Shutting down...
Stopped.
end 2026-11-04 11:27:32 UTC

### example udp_only_public_tracker
start 2026-11-04 11:27:32 UTC
connect response: action=0 transaction_id=12345 length=16
exit=0
Types from torrust-tracker-configuration used by this binary:
  Core       — tracker domain settings
  UdpTracker — bind address, cookie lifetime
  (Configuration aggregate and idle types are NOT compiled in)

Listening on 127.0.0.1:40425
Press Ctrl-C (or send SIGTERM on Unix) to stop.

Received SIGTERM. Shutting down...
Stopped.
end 2026-11-04 11:27:35 UTC
```

## V8 - Pre-push checks and the independent pre-PR review

Pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`) on the final code; `git diff --quiet HEAD -- packages src tests deny.toml Cargo.toml Cargo.lock` exits 0 first, so the tested working tree has the code of the branch head. An earlier pre-push run (2026-11-04 11:15:46 to 11:18:05 UTC, exit 0) was on the code before the example changes below and is superseded by this one.

```text
start 2026-11-04 11:31:58 UTC
code-diff-vs-HEAD-exit=0
Running pre-push checks...
Note: these checks can take several minutes. If Git reports a closed SSH connection after they pass, see <repository root>/docs/git-hooks.md.
[Step 1/6] Checking format with nightly toolchain ... PASS (1s)
[Step 2/6] Checking workspace with nightly toolchain ... PASS (0s)
[Step 3/6] Building documentation with nightly toolchain ... PASS (2s)
[Step 4/6] Building library documentation with nightly toolchain ... PASS (4s)
[Step 5/6] Checking that every workspace crate can be packaged ... PASS (27s)
[Step 6/6] Running all tests ... PASS (44s)
==========================================
SUCCESS: All pre-push checks passed! (1m 18s)
==========================================
You can now safely push your changes.
exit=0
end 2026-11-04 11:33:16 UTC
```

`linter all` runs in the pre-commit gate of every commit of the branch (each commit of this branch passed it, the one with the example refactor after the `too_many_lines` clippy finding of its first attempt was fixed).

Independent pre-PR review (read-only Task Reviewer, 2026-11-04, while the first pre-push ran): **PASS WITH FINDINGS**, no Blocker or Major. It verified the dependency audit directly (the registry crate is used only by `src/testing/environment.rs`, `Cargo.toml` and the new example; every other direct dependency has a production use) and the gate shape. It did not run clippy, `cargo doc` or tests because the first pre-push held the target lock. Its findings and their disposition:

| # | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| 1 | Minor | The new example says it does not use the `testing` feature, but it imports the registry crate, which reaches it through the self dev-dependency | Fixed in `refactor(udp-server): [#2699] follow the application's listener policy and loop error handling in the production-style example`: the header says it does not use the `testing` module and explains the self dev-dependency. The T4 decision (no explicit dev-dependency) is recorded in the spec. |
| 2 | Minor | The example drops an `io::Error` of the receive loop | Fixed in the same commit: the three outcomes of the join are reported. |
| 3 | Minor | The metrics policy was hard-coded `true` and the drain was a constant | Fixed in the same commit: the policy comes from `UdpTracker::tracker_usage_statistics` and the drain from `Shutdown::event_listener_drain()`, like `src/bootstrap/jobs/udp_tracker_core.rs`. The header says the example spawns the listeners itself. |
| 4 | Minor | The V6 grep step printed `1` and `2` for the "does not use the testing module" check | Fixed: the script now counts `^use .*udp_server::testing` lines, as its own step (0 and 1). |
| 5 | Minor | The V7 commands were not shown | Fixed: V7 points at its script, which is in the last section. |
| 6 | Minor | M3 named a file that the filtered output does not show | Fixed: the claim says "two `E0433` errors in the integration test of that package". |
| 7 | Nit | The rustdoc step proves "absent from rustdoc output", not "not compiled" | Kept; the M1 row says "absent from the default `cargo doc` output", and the build steps next to it carry the compile claim. |
| 8 | Nit | After the gating commit the health check tests do not compile until the next commit | Kept: the spec's Commit Points table asks for separate commits, and the gate runs no test compilation. |
| 9 | Nit | EPIC row 4 notes and the ADR "does not decide" list | The EPIC row 4 note now says the new example exists. No ADR amendment was made: the decision is recorded in the spec and is a candidate for the coordinator. |

## V9 - The container's third-party recipe cooks with the fixed `torrust-cargo-chef` (M8)

The `Test (Docker) (release)` job of PR #2711 failed on its first push because `cargo chef prepare --external-only` strips workspace path dependencies but, in `torrust-cargo-chef` 0.1.78, not the `[features]` entries that name them (see the decision in the [spec](ISSUE.md)). V1 to V8 did not build the `Containerfile`, so they could not see it. This step reproduces the two `Containerfile` commands (`cargo chef prepare --external-only` in the planner stage, `cargo chef cook --tests --workspace --all-features` in the cook stage, debug profile here; the release profile of the job does not change how manifests are read) on this tree with the pinned version (red control) and with the build of the fix from the `torrust/cargo-chef` checkout (green). The fix is in https://github.com/torrust/cargo-chef/pull/1, branch `fix/external-only-prune-features` (its fix commit is `fix(external-only): prune feature entries that name a stripped path dependency`); the version under test is the head of that branch, `chore: bump torrust-cargo-chef to 0.1.79`, built with `cargo install --path ... --locked`. The tracker tree is checked unchanged (`git diff --quiet HEAD -- packages src tests deny.toml Cargo.toml Cargo.lock Containerfile` is the first line of the transcript). The `Containerfile` pin bump is a separate commit that is not pushed until 0.1.79 is published, so the job itself has not run with it yet: AC6 stays open until it passes.

What the transcript shows: with 0.1.78, the skeleton of `udp-server` has `testing = ["dep:torrust-tracker-swarm-coordination-registry"]` and no registry dependency, and `cook` exits 101 with the message of the CI job; with 0.1.79 the skeleton has `testing = []` and `cook` exits 0 (it built the third-party crates of the workspace, 34 s).

```text
RUN START 2026-11-04 12:12:08 UTC
tracker tree head subject: docs(issues): [#2699] record the cargo-chef problem, the options and the decision
tracker tree code paths unchanged against HEAD: yes
fork head subject: chore: bump torrust-cargo-chef to 0.1.79
fork working tree clean: yes
### install the patched torrust-cargo-chef from the fork checkout
start 2026-11-04 12:12:08 UTC
   Compiling thiserror v2.0.18
   Compiling thiserror-impl v2.0.18
   Installed package `torrust-cargo-chef v0.1.79 (<cargo-chef checkout>)` (executable `cargo-chef`)
end 2026-11-04 12:12:19 UTC

### [old] torrust-cargo-chef-chef 0.1.78
start prepare 2026-11-04 12:12:19 UTC
prepare exit=0  end 2026-11-04 12:12:19 UTC
udp-server skeleton [features]: [features] | default = [] | testing = ["dep:torrust-tracker-swarm-coordination-registry"]
registry crate listed as dependency: False
start cook 2026-11-04 12:12:19 UTC
error: failed to load manifest for workspace member `/tmp/row4/cook-old/packages/udp-server`
  feature `testing` includes `dep:torrust-tracker-swarm-coordination-registry`, but `torrust-tracker-swarm-coordination-registry` is not listed as a dependency
thread 'main' (712222) panicked at /home/josecelano/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/torrust-cargo-chef-0.1.78/src/recipe.rs:243:27:
Exited with status code: 101
cook exit=101
end cook 2026-11-04 12:12:19 UTC

### [new] torrust-cargo-chef-chef 0.1.79
start prepare 2026-11-04 12:12:19 UTC
prepare exit=0  end 2026-11-04 12:12:19 UTC
udp-server skeleton [features]: [features] | default = [] | testing = []
registry crate listed as dependency: False
start cook 2026-11-04 12:12:19 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 34.27s
cook exit=0
end cook 2026-11-04 12:12:54 UTC
RUN END 2026-11-04 12:12:54 UTC
```

The script, as it ran (`/tmp/row4/recipe-old.json` and `recipe-new.json` are the two recipes; the old version was installed earlier with `cargo install torrust-cargo-chef --version 0.1.78 --locked --root /tmp/row4/chef`):

```bash
#!/usr/bin/env bash
set -u
R=<repo>
FORK=<cargo-chef checkout>
ts() { date -u '+%Y-%m-%d %H:%M:%S'; }

echo "RUN START $(ts) UTC"
echo "tracker tree head subject: $(git -C "$R" log -1 --format=%s)"
echo "tracker tree code paths unchanged against HEAD: $(git -C "$R" diff --quiet HEAD -- packages src tests deny.toml Cargo.toml Cargo.lock Containerfile && echo yes || echo NO)"
echo "fork head subject: $(git -C "$FORK" log -1 --format=%s)"
echo "fork working tree clean: $(git -C "$FORK" diff --quiet HEAD && echo yes || echo NO)"

echo "### install the patched torrust-cargo-chef from the fork checkout"
echo "start $(ts) UTC"
cargo install --path "$FORK" --locked --root /tmp/row4/chef-new 2>&1 | grep -E "Installed|error"
echo "end $(ts) UTC"

for v in old new; do
  if [ "$v" = old ]; then BIN=/tmp/row4/chef/bin; else BIN=/tmp/row4/chef-new/bin; fi
  echo
  echo "### [$v] $(PATH=$BIN:$PATH cargo chef --version)"
  echo "start prepare $(ts) UTC"
  (cd "$R" && PATH=$BIN:$PATH cargo chef prepare --external-only --recipe-path /tmp/row4/recipe-$v.json) 2>&1 | tail -3
  echo "prepare exit=$?  end $(ts) UTC"
  python3 - "$v" <<'PY'
import json, sys
r = json.load(open(f"/tmp/row4/recipe-{sys.argv[1]}.json"))
for m in r["skeleton"]["manifests"]:
    if m["relative_path"] == "packages/udp-server/Cargo.toml":
        text = m["contents"]
        start = text.index("[features]")
        print("udp-server skeleton [features]:", text[start:].split("\n\n")[0].replace("\n", " | "))
        print("registry crate listed as dependency:", "swarm-coordination-registry =" in text or "[dependencies.torrust-tracker-swarm" in text)
PY
  rm -rf /tmp/row4/cook-$v && mkdir -p /tmp/row4/cook-$v
  echo "start cook $(ts) UTC"
  (cd /tmp/row4/cook-$v && CARGO_TARGET_DIR=/tmp/row4/cook-target PATH=$BIN:$PATH cargo chef cook --tests --workspace --all-features --recipe-path /tmp/row4/recipe-$v.json 2>&1 | grep -E "^error|feature .testing. includes|Finished|panicked|Exited" | head -8; echo "cook exit=${PIPESTATUS[0]}")
  echo "end cook $(ts) UTC"
done
echo "RUN END $(ts) UTC"
```

### V9 follow-up - the published 0.1.79

The maintainer merged https://github.com/torrust/cargo-chef/pull/1 on 2026-11-04 at 12:23:33 UTC, and `torrust-cargo-chef` 0.1.79 was published to crates.io on 2026-11-04 at 12:25:09 UTC (the crates.io API lists it, not yanked) and tagged `v0.1.79-torrust` on the merge commit. `cargo-binstall` is not installed on this host, so the published crate was installed from crates.io with `cargo install`, which is what `cargo binstall` builds when no prebuilt artifact exists, and the same two `Containerfile` commands were run on this tree with it (2026-11-04, 12:25:45 to 12:25:55 UTC; the tracker tree had no change, `git status --short` empty):

```text
$ cargo install torrust-cargo-chef --version 0.1.79 --locked --root /tmp/row4/crates-io
   Installed package `torrust-cargo-chef v0.1.79` (executable `cargo-chef`)
$ cargo chef --version
torrust-cargo-chef-chef 0.1.79
$ cargo chef prepare --external-only --recipe-path /tmp/row4/recipe-published.json
$ cd <scratch> && cargo chef cook --tests --workspace --all-features --recipe-path /tmp/row4/recipe-published.json
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.90s
cook exit=0
```

The 0.90 s is a cache hit: the third-party crates were already built in the same target directory by the run above.

## Script

The script of V1 to V6 and the matrix, as it ran (the pre-push run of V8 is the hook itself). `run LABEL FILTER COMMAND...` prints the `date -u` start stamp, runs the command, prints the lines of its output that match the filter, and prints the end stamp and the command's exit code. The restore steps compare the working tree with `HEAD` through `git diff --quiet` on the named paths after a `git checkout -q --` of the files the red step edited.

```bash
#!/usr/bin/env bash
set -u
R=<repo>
B=/tmp/row4/base
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
echo "tested head subject: $(git log -1 --format=%s)"
run "tree is clean for the code paths" '.' bash -c "git diff --quiet HEAD -- packages src tests deny.toml Cargo.toml Cargo.lock; echo diff-exit=\$?"

git worktree add -q --detach "$B" 197c7330d
echo "baseline worktree subject: $(git -C "$B" log -1 --format=%s)"
echo

# V1 direct normal dependencies
run "V1 before: direct normal dependencies of udp-server" '.' bash -c "cd $B && cargo tree -p torrust-tracker-udp-server -e normal --depth 1"
run "V1 after: direct normal dependencies of udp-server" '.' cargo tree -p torrust-tracker-udp-server -e normal --depth 1
run "V1 after, with the feature: direct normal dependencies" 'swarm|udp-server v' cargo tree -p torrust-tracker-udp-server -e normal --depth 1 -F testing
run "V1 transitive: where the registry crate stays reachable from udp-server (after, no feature)" '.' cargo tree -p torrust-tracker-udp-server -e normal -i torrust-tracker-swarm-coordination-registry
run "V1 builds without and with the feature" 'Finished|^error' bash -c "cargo build -p torrust-tracker-udp-server && cargo build -p torrust-tracker-udp-server --features testing"
run "V1 the module is not compiled without the feature (rustdoc item count)" '.' bash -c "cargo doc -p torrust-tracker-udp-server --no-deps >/dev/null 2>&1; echo testing-dir-default=\$(ls -d target/doc/torrust_tracker_udp_server/testing 2>/dev/null | wc -l); cargo doc -p torrust-tracker-udp-server --no-deps --features testing >/dev/null 2>&1; echo testing-dir-with-feature=\$(ls -d target/doc/torrust_tracker_udp_server/testing 2>/dev/null | wc -l)"

# V3 consumer: red without the feature, green with it
sed -i 's#^\(torrust-tracker-udp-server = { version = "0.1.0", path = "../udp-server"\), features = \[ "testing" \] }#\1 }#' packages/axum-health-check-api-server/Cargo.toml
run "V3 red: health check package without its dev-dependency feature" '^error' cargo test -p torrust-tracker-axum-health-check-api-server --no-run
git checkout -q -- packages/axum-health-check-api-server/Cargo.toml Cargo.lock
run "V3 restore: no change in manifests" '.' bash -c "git diff --quiet -- Cargo.toml Cargo.lock packages deny.toml; echo clean-exit=\$?"
run "V3 dev edge of the health check package to udp-server, with its features" 'udp-server' bash -c "cargo tree -p torrust-tracker-axum-health-check-api-server -e dev --depth 1 -f '{p} [{f}]'"
run "V3 green: health check package with its dev-dependency feature" '^test result' cargo test -p torrust-tracker-axum-health-check-api-server

# V4 shipped features
run "V4 green: real tree" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
sed -i 's#^torrust-tracker-udp-server = { version = "0.1.0", path = "packages/udp-server" }#torrust-tracker-udp-server = { version = "0.1.0", path = "packages/udp-server", features = ["testing"] }#' Cargo.toml
run "V4 red 1: features testing on the root [dependencies] entry of udp-server" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
git checkout -q -- Cargo.toml Cargo.lock
sed -i 's#^default = \[  \]#default = [ "testing" ]#' packages/udp-server/Cargo.toml
run "V4 red 2: testing in the default chain of udp-server" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker
git checkout -q -- packages/udp-server/Cargo.toml Cargo.lock
run "V4 restore: no change in manifests" '.' bash -c "git diff --quiet -- Cargo.toml Cargo.lock packages deny.toml; echo clean-exit=\$?"
run "V4 green again after the restore" '.' cargo run -q -p shipped-features-check -- --package torrust-tracker

# V4 preserved shipped-profile binary
run "V4 build the shipped-profile binary" 'Finished|^error' cargo build -p torrust-tracker --bin torrust-tracker
mkdir -p /tmp/row4/shipped && cp target/debug/torrust-tracker /tmp/row4/shipped/torrust-tracker
run "V4 checksum of the build and of the preserved copy (first 16 hex digits)" '.' bash -c "sha256sum target/debug/torrust-tracker /tmp/row4/shipped/torrust-tracker | cut -c1-16,65-"
run "V4 test environment strings in the preserved copy" '.' bash -c "echo environment=\$(strings /tmp/row4/shipped/torrust-tracker | grep -c 'UDP test environment') udp_server_testing_module=\$(strings /tmp/row4/shipped/torrust-tracker | grep -c 'udp-server/src/testing')"
run "V4 plain cli-configuration tests" '^test result|^error' cargo test -p torrust-tracker --test cli-configuration
run "V4 checksums after the plain run" '.' bash -c "sha256sum target/debug/torrust-tracker /tmp/row4/shipped/torrust-tracker | cut -c1-16,65-"
run "V4 cli-configuration tests with TORRUST_TRACKER_BINARY set to the preserved copy" '^test result|^error' env TORRUST_TRACKER_BINARY=/tmp/row4/shipped/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
run "V4 checksum of the preserved copy after the override run" '.' bash -c "sha256sum /tmp/row4/shipped/torrust-tracker | cut -c1-16"

# V5
run "V5 cargo deny check bans (committed deny.toml)" 'bans (ok|FAILED)' cargo deny check bans
cp deny.toml /tmp/row4/deny.keep
python3 - <<'PY'
lines = open("deny.toml").read().split("\n")
out = []
inside = False
for l in lines:
    if 'crate = "torrust-tracker-udp-server", wrappers' in l:
        inside = True
    if inside and l.strip() == '"torrust-tracker-udp-server",':
        continue
    if inside and l.strip() == "] },":
        inside = False
    out.append(l)
open("deny.toml", "w").write("\n".join(out))
PY
run "V5 red: cargo deny check bans without the self wrapper" '^error\[|bans (ok|FAILED)' cargo deny check bans
cp /tmp/row4/deny.keep deny.toml
run "V5 restore: deny.toml has no change" '.' bash -c "git diff --quiet -- deny.toml; echo clean-exit=\$?"
run "V5 cargo deny check bans after the restore" 'bans (ok|FAILED)' cargo deny check bans
run "V5 cargo machete --with-metadata" 'didn.t find|found' cargo machete --with-metadata
run "V5 no required-features on any target of the workspace" '.' bash -c "grep -rn 'required-features' --include=Cargo.toml . --exclude-dir=target --exclude-dir=.git | wc -l"
run "V5 cargo +stable package --workspace --no-verify" 'Packaged|^error|warning: ' cargo +stable package --workspace --no-verify --allow-dirty

# V6 examples
run "V6 import lines of the testing module: new example, then existing example" '.' bash -c "grep -c '^use .*udp_server::testing' packages/udp-server/examples/udp_only_public_tracker_production_style.rs; grep -c '^use .*udp_server::testing' packages/udp-server/examples/udp_only_public_tracker.rs"
run "V6 cargo build -p torrust-tracker-udp-server --examples" 'Finished|^error' cargo build -p torrust-tracker-udp-server --examples
run "V6 cargo test -p torrust-tracker-udp-server --examples" '^test result|Running|^error' cargo test -p torrust-tracker-udp-server --examples
run "V6 plain cargo run of the production-style example (killed by timeout after the listening line)" 'Listening|^error' timeout 60 cargo run -q -p torrust-tracker-udp-server --example udp_only_public_tracker_production_style
run "V6 plain cargo run of the testing-style example (killed by timeout after the listening line)" 'Listening|^error' timeout 60 cargo run -q -p torrust-tracker-udp-server --example udp_only_public_tracker
run "V6 bench builds" 'Finished|^error' cargo bench -p torrust-tracker-udp-server --no-run
run "V6 nightly check of every target with all features" 'Finished|^error|^warning' cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features

# V2 matrix, before then after
mat() {
  local where="$1" pkg="$2" label="$3"
  shift 3
  run "V2 cargo test -p $pkg $label" '^test result|^error' bash -c "cd $where && cargo test -p $pkg $*"
}
for p in torrust-tracker-udp-server torrust-tracker-axum-health-check-api-server torrust-tracker-axum-rest-api-server torrust-tracker-rest-api-runtime-adapter; do
  for w in "$B:before" "$R:after"; do
    dir=${w%%:*}
    tag=${w##*:}
    mat "$dir" "$p" "(plain, $tag)" ""
    mat "$dir" "$p" "--all-features ($tag)" "--all-features"
    mat "$dir" "$p" "--all-targets --all-features ($tag)" "--all-targets --all-features"
  done
done

git worktree remove --force "$B"
echo "RUN END $(ts) UTC"
echo "final manifests status: $(git status --short -- Cargo.toml Cargo.lock packages deny.toml | wc -l) changed files"
```

The script of V7, as it ran:

```bash
#!/usr/bin/env bash
set -u
R=<repo>
cd "$R" || exit 1
ts() { date -u '+%Y-%m-%d %H:%M:%S'; }

for ex in udp_only_public_tracker_production_style udp_only_public_tracker; do
  echo "### example $ex"
  echo "start $(ts) UTC"
  cargo build -q -p torrust-tracker-udp-server --example "$ex" 2>&1 | tail -2
  target/debug/examples/$ex > /tmp/row4/$ex.out 2>&1 &
  pid=$!
  sleep 3
  addr=$(grep -oE '127\.0\.0\.1:[0-9]+' /tmp/row4/$ex.out | head -1)
  python3 - "$addr" <<'PY'
import socket, struct, sys
host, port = sys.argv[1].split(":")
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(3)
s.sendto(struct.pack(">QII", 0x41727101980, 0, 12345), (host, int(port)))
data, _ = s.recvfrom(64)
action, tx, _conn = struct.unpack(">IIQ", data)
print(f"connect response: action={action} transaction_id={tx} length={len(data)}")
PY
  kill -TERM "$pid"
  wait "$pid"
  echo "exit=$?"
  cat /tmp/row4/$ex.out
  echo "end $(ts) UTC"
  echo
done
```
