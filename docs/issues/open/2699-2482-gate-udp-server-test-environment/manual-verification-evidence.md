---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2699-2482-gate-udp-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-04 11:14"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). Every output below is the real output of the command shown; paths under the repository are replaced by `<repo>`, and output that only lists unrelated dependencies or packaging is reduced as each section says.

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain (`cargo 1.99.1`), `cargo +nightly` (`cargo 1.101.0-nightly`) for the all-targets check. No container build is needed: the `Containerfile` change of #2670 already builds the shipped binary without test features, and V4 checks the shipped-profile binary directly.
- Date and time (UTC): V1 to V6 and the matrix ran as one script on 2026-11-04 from 10:59:31 to 11:08:08 UTC; the examples run (V7) on 2026-11-04 at 11:13 UTC. The sub-second part of the stamps is not monotonic in a few steps (an end stamp a few hundred milliseconds before its start stamp, for example in "V3 red"), so the order of the steps is the order of the script, and only the seconds are meaningful.
- Artifact under test: the branch head whose subject is `feat(udp-server): [#2699] add a production-style public tracker example`, when the script started; the script checks that the code paths (`packages`, `src`, `tests`, `deny.toml`, `Cargo.toml`, `Cargo.lock`) have no change against `HEAD` (`git diff --quiet HEAD -- <paths>`, exit 0).
- Baseline tree: `develop` at the merge of PR #2700 (`docs(issues): [#2699] specify gating the udp-server test environment`), before any change of this issue, built in a detached worktree (`git worktree add --detach`) and removed at the end.
- Test counts come from the `test result:` lines of the cargo runs; the transcript below keeps only those lines. The raw logs were kept under `/tmp` during the work and are not part of the repository. Benches run in test mode under `--all-targets` and print no `test result:` line.

## Results

| Scenario | Result | Where |
| --- | --- | --- |
| M1 The production build has no test-only dependency | The direct normal dependencies of `udp-server` drop from 27 to 26; `torrust-tracker-swarm-coordination-registry` is no longer a direct dependency. It stays reachable transitively through `torrust-tracker-core` and `torrust-tracker-udp-core` (the `-i` tree below). With `-F testing` it is the only added direct dependency. `src/testing/` is absent from the default `cargo doc` output and present with the feature. | V1 |
| M2 Plain package tests | `cargo test -p torrust-tracker-udp-server` runs 233 + 16 + 1 (1 ignored) + 2 tests, equal to the baseline, with and without `--all-features`; `--all-targets --all-features` has one more empty test binary, the new example. | V2 |
| M3 The consumer still works | `cargo test -p torrust-tracker-axum-health-check-api-server` runs 15 + 8 tests, equal to the baseline. Without its dev-dependency feature the same command fails to compile (`E0433` at `tests/server/contract.rs`, two sites), so the feature entry is needed. | V2, V3 |
| M4 Shipped features | green on the real tree; red with `features = ["testing"]` on the root `[dependencies]` entry of `udp-server`; red with `default = [ "testing" ]`; green again after restoring (clean `git diff --quiet`). | V4 |
| M5 Dependents | `axum-rest-api-server` (106 + 58) and `rest-api-runtime-adapter` (3 + 0), which depend on `udp-server` through `[dependencies]`, have identical counts before and after, plain and with `--all-features`. | V2 |
| M6 Examples | Both build under `cargo build --examples` and `cargo test --examples`; both run under plain `cargo run` (they print `Listening on ...` until the timeout kills them); both answer a real UDP connect request and stop on SIGTERM (V7). The new example has no `use` of `udp_server::testing`; the existing one has exactly one. | V6, V7 |
| M7 Deny, machete, packaging, nightly, bench | `cargo deny check bans` ok, and failing (`explicitly banned`) without the self wrapper; `cargo machete --with-metadata` finds nothing; `cargo +stable package --workspace --no-verify` exits 0; `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features` exits 0; `cargo bench -p torrust-tracker-udp-server --no-run` exits 0; no `required-features` anywhere in the workspace manifests. | V5, V6 |
| M8 The shipped binary | The binary built by `cargo build -p torrust-tracker --bin torrust-tracker` has no trace of the UDP test environment (strings count 0 for both markers); the 35 `cli-configuration` tests pass with `TORRUST_TRACKER_BINARY` set to the preserved copy, whose checksum is unchanged by the plain run (the plain run rebuilt `target/debug/torrust-tracker`, whose checksum differs). | V4 |

Dependency audit against rule A.2 of the ADR (done for the spec on 2026-11-03 and checked again by the "V1 before" and "V1 after" lists): `torrust-tracker-swarm-coordination-registry` was the only direct normal dependency of `udp-server` that only `src/testing/environment.rs` uses. Every other direct normal dependency is also used by non-test code, and `mockall`, `rand`, `criterion`, `torrust-tracker-test-helpers` are dev-dependencies already. The new production-style example imports the registry crate; it compiles because the self dev-dependency enables `testing` for the package's own targets, so no explicit dev-dependency on the registry crate was added (T4). `cargo build -p torrust-tracker-udp-server --examples` (V6) succeeded; that build runs with the package's dev-dependencies, so it shows the example compiles under the self dev-dependency, not that it would compile without the feature.

## V1 to V6 and the matrix - transcript

The script below ran once; its filtered output follows. "before" runs `cd <baseline worktree> && cargo ...`, "after" runs in this tree.

```text
RUN START 2026-11-04 10:59:31.759 UTC
tested head subject: feat(udp-server): [#2699] add a production-style public tracker example
### tree is clean for the code paths
start 2026-11-04 10:59:31.763 UTC
diff-exit=0
end 2026-11-04 10:59:31.772 UTC exit=0

baseline worktree subject: Merge torrust/torrust-tracker#2700: docs(issues): [#2699] specify gating the udp-server test environment

### V1 before: direct normal dependencies of udp-server
start 2026-11-04 10:59:31.896 UTC
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
end 2026-11-04 10:59:32.238 UTC exit=0

### V1 after: direct normal dependencies of udp-server
start 2026-11-04 10:59:32.262 UTC
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
end 2026-11-04 10:59:32.150 UTC exit=0

### V1 after, with the feature: direct normal dependencies
start 2026-11-04 10:59:32.153 UTC
torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
├── torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
end 2026-11-04 10:59:32.276 UTC exit=0

### V1 transitive: where the registry crate stays reachable from udp-server (after, no feature)
start 2026-11-04 10:59:32.278 UTC
torrust-tracker-swarm-coordination-registry v0.1.0 (<repo>/packages/swarm-coordination-registry)
├── torrust-tracker-core v0.1.0 (<repo>/packages/tracker-core)
│   ├── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core)
│   │   └── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
│   └── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server)
└── torrust-tracker-udp-core v0.1.0 (<repo>/packages/udp-core) (*)
end 2026-11-04 10:59:32.402 UTC exit=0

### V1 builds without and with the feature
start 2026-11-04 10:59:32.404 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.14s
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.26s
end 2026-11-04 10:59:34.867 UTC exit=0

### V1 the module is not compiled without the feature (rustdoc item count)
start 2026-11-04 10:59:34.869 UTC
testing-dir-default=0
testing-dir-with-feature=1
end 2026-11-04 10:59:41.729 UTC exit=0

### V3 red: health check package without its dev-dependency feature
start 2026-11-04 10:59:41.764 UTC
error[E0433]: cannot find `testing` in `torrust_tracker_udp_server`
error[E0433]: cannot find `testing` in `torrust_tracker_udp_server`
error: could not compile `torrust-tracker-axum-health-check-api-server` (test "integration") due to 2 previous errors
end 2026-11-04 10:59:41.412 UTC exit=101

### V3 restore: no change in manifests
start 2026-11-04 10:59:41.418 UTC
clean-exit=0
end 2026-11-04 10:59:41.426 UTC exit=0

### V3 dev edge of the health check package to udp-server, with its features
start 2026-11-04 10:59:41.428 UTC
torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server) []
├── torrust-tracker-axum-http-server v0.1.0 (<repo>/packages/axum-http-server) []
├── torrust-tracker-axum-rest-api-server v0.1.0 (<repo>/packages/axum-rest-api-server) []
├── torrust-tracker-test-helpers v3.0.0 (<repo>/packages/test-helpers) []
└── torrust-tracker-udp-server v0.1.0 (<repo>/packages/udp-server) [default,testing]
end 2026-11-04 10:59:41.557 UTC exit=0

### V3 green: health check package with its dev-dependency feature
start 2026-11-04 10:59:41.559 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 10:59:45.822 UTC exit=0

### V4 green: real tree
start 2026-11-04 10:59:45.824 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-11-04 10:59:46.190 UTC exit=0

### V4 red 1: features testing on the root [dependencies] entry of udp-server
start 2026-11-04 10:59:46.194 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-udp-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-udp-server","feature":"testing","exit_code":1}
end 2026-11-04 10:59:46.386 UTC exit=1

### V4 red 2: testing in the default chain of udp-server
start 2026-11-04 10:59:46.393 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-udp-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-udp-server","feature":"testing","exit_code":1}
end 2026-11-04 10:59:46.585 UTC exit=1

### V4 restore: no change in manifests
start 2026-11-04 10:59:46.590 UTC
clean-exit=0
end 2026-11-04 10:59:46.598 UTC exit=0

### V4 green again after the restore
start 2026-11-04 10:59:46.600 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
end 2026-11-04 10:59:46.813 UTC exit=0

### V4 build the shipped-profile binary
start 2026-11-04 10:59:46.815 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 12.86s
end 2026-11-04 10:59:59.722 UTC exit=0

### V4 checksum of the build and of the preserved copy (first 16 hex digits)
start 2026-11-04 10:59:59.799 UTC
3267c410074c8956  target/debug/torrust-tracker
3267c410074c8956  /tmp/row4/shipped/torrust-tracker
end 2026-11-04 10:59:59.991 UTC exit=0

### V4 test environment strings in the preserved copy
start 2026-11-04 10:59:59.993 UTC
environment=0 udp_server_testing_module=0
end 2026-11-04 11:00:01.342 UTC exit=0

### V4 plain cli-configuration tests
start 2026-11-04 11:00:01.344 UTC
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-11-04 11:00:15.327 UTC exit=0

### V4 checksums after the plain run
start 2026-11-04 11:00:15.329 UTC
524f68579576b3ed  target/debug/torrust-tracker
3267c410074c8956  /tmp/row4/shipped/torrust-tracker
end 2026-11-04 11:00:15.531 UTC exit=0

### V4 cli-configuration tests with TORRUST_TRACKER_BINARY set to the preserved copy
start 2026-11-04 11:00:15.533 UTC
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
end 2026-11-04 11:00:16.768 UTC exit=0

### V4 checksum of the preserved copy after the override run
start 2026-11-04 11:00:16.771 UTC
3267c410074c8956
end 2026-11-04 11:00:16.873 UTC exit=0

### V5 cargo deny check bans (committed deny.toml)
start 2026-11-04 11:00:16.876 UTC
bans ok
end 2026-11-04 11:00:17.437 UTC exit=0

### V5 red: cargo deny check bans without the self wrapper
start 2026-11-04 11:00:17.448 UTC
error[banned]: crate 'torrust-tracker-udp-server = 0.1.0' is explicitly banned
bans FAILED
end 2026-11-04 11:00:18.483 UTC exit=2

### V5 restore: deny.toml has no change
start 2026-11-04 11:00:18.827 UTC
clean-exit=0
end 2026-11-04 11:00:18.180 UTC exit=0

### V5 cargo deny check bans after the restore
start 2026-11-04 11:00:18.202 UTC
bans ok
end 2026-11-04 11:00:18.576 UTC exit=0

### V5 cargo machete --with-metadata
start 2026-11-04 11:00:18.578 UTC
cargo-machete didn't find any unused dependencies in this directory. Good job!
end 2026-11-04 11:00:22.344 UTC exit=0

### V5 no required-features on any target of the workspace
start 2026-11-04 11:00:22.346 UTC
0
end 2026-11-04 11:00:22.362 UTC exit=0

### V5 cargo +stable package --workspace --no-verify
(34 "Packaged" lines and 9 license/homepage manifest warnings, all unrelated to this change, reduced to these counts)
start 2026-11-04 11:00:22.364 UTC
end 2026-11-04 11:00:59.694 UTC exit=0

### V6 the new example does not use the testing module
start 2026-11-04 11:00:59.696 UTC
1
2
end 2026-11-04 11:00:59.703 UTC exit=0

### V6 cargo build -p torrust-tracker-udp-server --examples
start 2026-11-04 11:00:59.704 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.71s
end 2026-11-04 11:01:00.457 UTC exit=0

### V6 cargo test -p torrust-tracker-udp-server --examples
start 2026-11-04 11:01:00.459 UTC
     Running unittests examples/udp_only_public_tracker.rs (target/debug/examples/udp_only_public_tracker-9a80d3aab20954cf)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests examples/udp_only_public_tracker_production_style.rs (target/debug/examples/udp_only_public_tracker_production_style-9ce169b9b31ef80c)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:01:00.633 UTC exit=0

### V6 plain cargo run of the production-style example (killed by timeout after the listening line)
start 2026-11-04 11:01:00.635 UTC
Listening on 127.0.0.1:40012
end 2026-11-04 11:02:00.648 UTC exit=124

### V6 plain cargo run of the testing-style example (killed by timeout after the listening line)
start 2026-11-04 11:02:00.650 UTC
Listening on 127.0.0.1:45217
end 2026-11-04 11:03:00.674 UTC exit=124

### V6 bench builds
start 2026-11-04 11:03:00.676 UTC
    Finished `bench` profile [optimized + debuginfo] target(s) in 1m 36s
end 2026-11-04 11:04:36.983 UTC exit=0

### V6 nightly check of every target with all features
start 2026-11-04 11:04:36.991 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 20.85s
end 2026-11-04 11:04:57.954 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server (plain, before)
start 2026-11-04 11:04:57.956 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.21s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.07s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
end 2026-11-04 11:05:21.789 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-features (before)
start 2026-11-04 11:05:21.793 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.29s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.17s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
end 2026-11-04 11:05:31.244 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-targets --all-features (before)
start 2026-11-04 11:05:31.251 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.24s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.11s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:05:41.966 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server (plain, after)
start 2026-11-04 11:05:41.969 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.20s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.13s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
end 2026-11-04 11:05:58.592 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-features (after)
start 2026-11-04 11:05:58.644 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.36s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.17s
test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
end 2026-11-04 11:06:07.982 UTC exit=0

### V2 cargo test -p torrust-tracker-udp-server --all-targets --all-features (after)
start 2026-11-04 11:06:07.102 UTC
test result: ok. 233 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.28s
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.08s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:06:15.997 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server (plain, before)
start 2026-11-04 11:06:16.579 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:06:34.325 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-features (before)
start 2026-11-04 11:06:34.348 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:06:38.320 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-targets --all-features (before)
start 2026-11-04 11:06:38.323 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
end 2026-11-04 11:06:42.550 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server (plain, after)
start 2026-11-04 11:06:42.553 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:06:50.682 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-features (after)
start 2026-11-04 11:06:50.684 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:06:55.891 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-health-check-api-server --all-targets --all-features (after)
start 2026-11-04 11:06:55.975 UTC
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.05s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.13s
end 2026-11-04 11:07:00.169 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server (plain, before)
start 2026-11-04 11:07:00.181 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.37s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:07:24.158 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-features (before)
start 2026-11-04 11:07:24.162 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.32s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:07:28.277 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-targets --all-features (before)
start 2026-11-04 11:07:28.280 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.14s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.38s
end 2026-11-04 11:07:32.919 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server (plain, after)
start 2026-11-04 11:07:32.944 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.34s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:07:43.973 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-features (after)
start 2026-11-04 11:07:43.976 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.34s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:07:47.921 UTC exit=0

### V2 cargo test -p torrust-tracker-axum-rest-api-server --all-targets --all-features (after)
start 2026-11-04 11:07:47.923 UTC
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.31s
end 2026-11-04 11:07:51.676 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter (plain, before)
start 2026-11-04 11:07:51.678 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:01.330 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-features (before)
start 2026-11-04 11:08:01.332 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:03.518 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-targets --all-features (before)
start 2026-11-04 11:08:03.521 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:03.702 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter (plain, after)
start 2026-11-04 11:08:03.705 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:06.863 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-features (after)
start 2026-11-04 11:08:06.865 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:07.961 UTC exit=0

### V2 cargo test -p torrust-tracker-rest-api-runtime-adapter --all-targets --all-features (after)
start 2026-11-04 11:08:07.964 UTC
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
end 2026-11-04 11:08:08.478 UTC exit=0

RUN END 2026-11-04 11:08:08.508 UTC
final manifests status: 0 changed files
```

## V7 - Both examples answer a real UDP request and stop on a signal

Each example is built, started in the background, sent a UDP connect request (BEP 15, connection id request, transaction id 12345) through a short Python snippet, then stopped with SIGTERM. A connect response has action 0, the same transaction id and 16 bytes.

```text
### example udp_only_public_tracker_production_style
start 2026-11-04 11:13:00 UTC
connect response: action=0 transaction_id=12345 length=16
exit=0
Listening on 127.0.0.1:38474
Press Ctrl-C (or send SIGTERM on Unix) to stop.

Received SIGTERM. Shutting down...
Stopped.
end 2026-11-04 11:13:03 UTC

### example udp_only_public_tracker
start 2026-11-04 11:13:03 UTC
connect response: action=0 transaction_id=12345 length=16
exit=0
Types from torrust-tracker-configuration used by this binary:
  Core       — tracker domain settings
  UdpTracker — bind address, cookie lifetime
  (Configuration aggregate and idle types are NOT compiled in)

Listening on 127.0.0.1:37471
Press Ctrl-C (or send SIGTERM on Unix) to stop.

Received SIGTERM. Shutting down...
Stopped.
end 2026-11-04 11:13:06 UTC
```

## Script

The script of V1 to V6 and the matrix, as it ran. `run LABEL FILTER COMMAND...` prints the `date -u` start stamp, runs the command, prints the lines of its output that match the filter, and prints the end stamp and the command's exit code. The restore steps compare the working tree with `HEAD` through `git diff --quiet` on the named paths after a `git checkout -q --` of the files the red step edited. The check "V6 the new example does not use the testing module" matched the module name in the doc header of the new example (prose, not code), so the 1 it printed is not an import; the precise check is `grep -c '^use .*udp_server::testing'` on each example, which gave 0 for the new example and 1 for the existing one on 2026-11-04 at 11:12 UTC.

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
run "V6 the new example does not use the testing module" '.' bash -c "grep -c 'udp_server::testing' packages/udp-server/examples/udp_only_public_tracker_production_style.rs; grep -c 'udp_server::testing' packages/udp-server/examples/udp_only_public_tracker.rs"
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
