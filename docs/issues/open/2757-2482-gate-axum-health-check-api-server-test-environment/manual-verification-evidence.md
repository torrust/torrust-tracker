---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-09 16:45"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). Every command is in [`evidence/verify.sh`](evidence/verify.sh) (the diagram comparison is [`evidence/check_diagram.py`](evidence/check_diagram.py)); the transcript below is the output of one run of that script on a clean tree, ANSI codes stripped and the checkout path shown as `<repo>`. A line `[exit N]` is the exit code of the whole command, with `pipefail` set; the exit 1 of a `grep -c` that counts zero is the expected result, and exits 2 and 101 mark the failing controls and the merge-base check.

## Environment

- Host: Linux; stable Rust toolchain, `cargo +nightly` for the all-targets check. `torrust-cargo-chef` 0.1.79 (the version pinned in the `Containerfile`) was installed with `cargo install --locked` into a scratch directory; the globally installed `cargo-chef` is 0.1.71 and has no `--external-only`.
- Base: `torrust/develop` at `ca8efcd4d` (the merge of PR #2759). The run is on the branch head that carries the commit `docs(issues): [#2757] print the commit subject in the verification script` (named by subject, as the evidence template asks).
- Baseline (T1), measured at the base by the first block of the transcript: 17 direct normal dependencies; 15 unit + 11 integration + 0 doc tests; and `cargo check --lib` without features **already fails** there with E0277 (`Url: Serialize` is not satisfied), because `url` lacked `serde`; plain `cargo test` passes because crates reached through `[dev-dependencies]` request the feature.

## Scenarios

| Scenario | Result | Evidence |
| --- | --- | --- |
| M1 The production build has no test-only dependency | 16 direct normal dependencies, no `torrust-tracker-configuration`; with `-F testing` it is 17 | V1 |
| M2 Plain package tests | 15 + 11 + 0, equal to the baseline, with and without `--all-features`; `cargo check --lib` with no features now passes | V2 |
| M3 Shipped features | Exit 0 | V3 |
| M4 Deny, machete, packaging, nightly check | All exit 0; `deny` fails without the self wrapper entry and the lib tests do not compile without the self dev-dependency (controls) | V4 |
| M5 Container recipe | `prepare --external-only` exit 0, `cook --tests --workspace --all-features` exit 0 | V5 |
| M6 AC8 on the whole workspace | No test-only runtime edge in the regenerated report or the diagram | V6 |

## Transcript (V1 to V4 and V6)

```text
start 2026-11-09 16:44:53 UTC

$ git log -1 --format=%s
docs(issues): [#2757] print the commit subject in the verification script
[exit 0]

=== Baseline at the merge base (T1, F1): the library without features

$ cd /tmp/row7-base-check && git log --oneline -1 && cargo check -p torrust-tracker-axum-health-check-api-server --lib 2>&1 | grep -E '^error|could not compile' | head -3
ca8efcd4d Merge torrust/torrust-tracker#2759: docs(issues): [#2757] add the axum-health-check-api-server test environment gating spec
error[E0277]: the trait bound `Url: serde::Serialize` is not satisfied
error[E0277]: the trait bound `Url: serde::Deserialize<'de>` is not satisfied
error[E0277]: the trait bound `Url: serde::Deserialize<'de>` is not satisfied
[exit 101]

$ cd /tmp/row7-base-check && cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1 | grep -c '^[├└]'
17
[exit 0]

$ cd /tmp/row7-base-check && cargo test -p torrust-tracker-axum-health-check-api-server 2>&1 | grep -E 'test result|Running'
     Running unittests src/lib.rs (<repo>/target/debug/deps/torrust_tracker_axum_health_check_api_server-352cececbccc5068)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
     Running tests/integration.rs (<repo>/target/debug/deps/integration-7c4b497d2a7407be)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
[exit 0]

=== V1 direct normal dependencies

$ cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1 | grep -c '^[├└]'
16
[exit 0]

$ cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1 | grep -c configuration
0
[exit 1]

$ cargo tree -p torrust-tracker-axum-health-check-api-server -e normal --depth 1 -F testing | grep -c '^[├└]'
17
[exit 0]

=== V2 tests and the library without features

$ cargo check -p torrust-tracker-axum-health-check-api-server --lib 2>&1 | tail -1
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.10s
[exit 0]

$ cargo test -p torrust-tracker-axum-health-check-api-server 2>&1 | grep -E 'test result|Running'
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-e71ff352f93b278c)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
     Running tests/integration.rs (target/debug/deps/integration-ee0387f8999212e5)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.06s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
[exit 0]

$ cargo test -p torrust-tracker-axum-health-check-api-server --all-features 2>&1 | grep -E 'test result|Running'
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-e71ff352f93b278c)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
     Running tests/integration.rs (target/debug/deps/integration-ee0387f8999212e5)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
[exit 0]

=== V3 shipped features

$ cargo run -q --package shipped-features-check -- --package torrust-tracker
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
[exit 0]

=== V4 deny, machete, packaging, nightly

$ cargo deny check bans 2>&1 | tail -1
bans ok
[exit 0]

$ cargo machete --with-metadata 2>&1 | tail -2
cargo-machete didn't find any unused dependencies in this directory. Good job!
Done!
[exit 0]

$ cargo +stable package --workspace --no-verify 2>&1 | tail -1
    Packaged 9 files, 57.9KiB (14.4KiB compressed)
[exit 0]

$ cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features 2>&1 | tail -1
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.21s
[exit 0]

=== Control: deny without the self wrapper entry

$ RAYON_NUM_THREADS=1 cargo deny check bans 2>&1 | grep -E 'unmatched-wrapper|error\[banned\]'
warning[unmatched-wrapper]: direct parent 'torrust-tracker-axum-health-check-api-server = 0.2.0' of banned crate 'torrust-tracker-axum-health-check-api-server = 0.2.0' was not marked as a wrapper
error[banned]: crate 'torrust-tracker-axum-health-check-api-server = 0.2.0' is explicitly banned
[exit 2]

=== Control: no self dev-dependency (the lib tests do not compile)

$ cargo test -p torrust-tracker-axum-health-check-api-server 2>&1 | grep -E '^error|could not compile' | head -3
error[E0433]: cannot find module or crate `torrust_tracker_configuration` in this scope
error[E0432]: unresolved import `torrust_tracker_axum_health_check_api_server::environment`
error[E0432]: unresolved import `torrust_tracker_axum_health_check_api_server::environment`
[exit 101]

$ git status --short deny.toml Cargo.lock packages/axum-health-check-api-server/Cargo.toml
[exit 0]

=== V6 AC8: coupling report labels and the diagram

$ grep -c 'optional (testing)' docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-11-09.md
10
[exit 0]

$ python3 docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/evidence/check_diagram.py
nodes 33, arrows drawn 136, declared non-test-only edges 135
drawn but not declared: [('server-lib', 'net-prim')]
declared but not drawn: []
[exit 0]

end 2026-11-09 16:45:36 UTC
```

V5, container recipe (run on the same tree, in a scratch directory; only the recipe file is copied there):

```text
$ cargo chef --version
torrust-cargo-chef-chef 0.1.79
$ cargo chef prepare --external-only --recipe-path <scratch>/recipe.json
[exit 0]
$ cd <scratch> && cargo chef cook --tests --workspace --all-features --recipe-path <scratch>/recipe.json
    Finished `dev` profile [optimized + debuginfo] target(s) in 30.57s
[exit 0]
```

## V6, AC8

The coupling report is [`workspace-coupling-report-2026-11-09.md`](../../open/1669-overhaul-packages/workspace-coupling-report-2026-11-09.md), generated by `cargo run -q -p workspace-coupling -- <file>` on the implementation branch. Its legend line plus the nine edge headings explain the count of ten matching lines in the transcript.

- 34 workspace packages. Nine `optional (testing)` edges: `axum-health-check-api-server` to `configuration`; `axum-http-server` to `swarm-coordination-registry`; `axum-rest-api-server` to `tracker-core`, `http-core`, `rest-api-client`, `swarm-coordination-registry`, `udp-core` and `udp-server`; `udp-server` to `swarm-coordination-registry`. That is the nine forced edges the inventory counted.
- A scan of `cargo metadata` (run once during the work, not part of the script): no `testing` feature is in a `default` chain; no normal edge enables `testing` on a workspace dependency; the only normal test-ish crates are `mockall` (optional, `tracker-core`'s `testing`), `tempfile` in the root crate (out of scope by AC8) and in `frontmatter-validator`, and `testcontainers` in `persistence-benchmark` (both unpublished tools whose binaries use them; outside AC8's library scope). Every library `pub mod testing;` (or `environment`) is behind `#[cfg(feature = "testing")]`: `axum-health-check-api-server`, `axum-http-server`, `axum-rest-api-server` and `udp-server`.
- The diagram comparison (`check_diagram.py`, last command of the transcript) finds every drawn arrow declared by a manifest except the one documented external-to-external edge (`server-lib` to `net-prim`), and every declared non-test-only edge drawn. Before this change it also found the health check's `axum-health --> config`, four test-only edges that rows 4 and 5 had missed (`axum-http --> clock`, `info-hash`, `swarm`; `udp-srv --> swarm`) and six `test-helpers` arrows no manifest declares; all eleven are removed and the page states 135 edges.
- Result: AC8's conditions hold after this change. The criterion is ticked by the #2482 close-out, not here.
