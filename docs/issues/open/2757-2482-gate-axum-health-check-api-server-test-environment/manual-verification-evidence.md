---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-09 15:53"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). The transcript below is the real output of one script run (`/tmp/row7/verify.sh`, ANSI codes stripped), followed by the container recipe check and the AC8 check.

## Environment

- Host: Linux; stable Rust toolchain, `cargo +nightly` for the all-targets check. `torrust-cargo-chef` 0.1.79 (the version pinned in the `Containerfile`) was installed with `cargo install --locked` into a scratch directory (`/tmp/chef079`); the globally installed `cargo-chef` is 0.1.71 and has no `--external-only`.
- Base: `torrust/develop` at `ca8efcd4d` (the merge of PR #2759), implementation commit `1f2c2852f`.
- Baseline (T1), measured before the change on that base, with and without `--all-features`: 15 unit + 11 integration + 0 doc tests; 17 direct normal dependencies (`cargo tree --depth 1`).

## Scenarios

| Scenario | Result | Evidence |
| --- | --- | --- |
| M1 The production build has no test-only dependency | 16 direct normal dependencies, no `torrust-tracker-configuration`; with `-F testing` it is 17 | V1 |
| M2 Plain package tests | 15 + 11 + 0, equal to the baseline, with and without `--all-features`; `cargo check --lib` with no features passes | V2 |
| M3 Shipped features | Exit 0 | V3 |
| M4 Deny, machete, packaging, nightly check | All exit 0; `deny` fails without the self wrapper (control); the lib tests do not compile without the self dev-dependency (control) | V4 |
| M5 Container recipe | `prepare --external-only` exit 0, `cook --tests --workspace --all-features` exit 0 | V5 |
| M6 AC8 on the whole workspace | No test-only runtime edge in the regenerated report or the (corrected) diagram | V6 |

## Transcript

V1 to V4:

```text
start 2026-11-09 15:48:08 UTC
1f2c2852f refactor(health-check-api): [#2757] gate the test environment behind a testing feature
--- V1 direct normal deps
16
configuration lines exit=1
--- V1b with feature
17
--- V2 lib check no features
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.16s
--- V2 plain tests
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-e71ff352f93b278c)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.02s
     Running tests/integration.rs (target/debug/deps/integration-ee0387f8999212e5)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.06s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
--- V2 all-features tests
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-e71ff352f93b278c)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
     Running tests/integration.rs (target/debug/deps/integration-ee0387f8999212e5)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
--- V3 shipped features
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
exit=0
--- V4 deny/machete/package
bans ok
cargo-machete didn't find any unused dependencies in this directory. Good job!
Done!
    Packaged 9 files, 57.9KiB (14.4KiB compressed)
package exit=0
--- V4 nightly all-targets
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.76s
--- control: deny without self wrapper (RAYON_NUM_THREADS=1; without it cargo deny panics in this sandbox when it prints errors)
warning[unmatched-wrapper]: direct parent 'torrust-tracker-axum-health-check-api-server = 0.2.0' of banned crate 'torrust-tracker-axum-health-check-api-server = 0.2.0' was not marked as a wrapper
error[banned]: crate 'torrust-tracker-axum-health-check-api-server = 0.2.0' is explicitly banned
deny exit=2
--- control: testing feature without self dev-dep (lib tests vanish)
error[E0433]: cannot find module or crate `torrust_tracker_configuration` in this scope
error: could not compile `torrust-tracker-axum-health-check-api-server` (lib test) due to 1 previous error
end 2026-11-09 15:49:11 UTC
```

The tree-clean checks after each control are implied by the restored files: `git status` showed only the intended changes afterwards (a control run rewrote `Cargo.lock` once and it was restored with `git checkout Cargo.lock`).

V5, container recipe:

```text
$ cargo chef --version
torrust-cargo-chef-chef 0.1.79
$ cargo chef prepare --external-only --recipe-path /tmp/row7/recipe.json      # exit 0
$ cd /tmp/row7/scratch && cargo chef cook --tests --workspace --all-features --recipe-path /tmp/row7/recipe.json
    Finished `dev` profile [optimized + debuginfo] target(s) in 31.76s       # exit 0
```

V6, AC8 (the coupling report is [`workspace-coupling-report-2026-11-09.md`](../../open/1669-overhaul-packages/workspace-coupling-report-2026-11-09.md), generated by `cargo run -q -p workspace-coupling -- <file>` on the implementation branch):

- 34 workspace packages. Nine `optional (testing)` edges: `axum-health-check-api-server` to `configuration`; `axum-http-server` to `swarm-coordination-registry`; `axum-rest-api-server` to `tracker-core`, `http-core`, `rest-api-client`, `swarm-coordination-registry`, `udp-core` and `udp-server`; `udp-server` to `swarm-coordination-registry`. That is the nine forced edges the inventory counted.
- A scan of `cargo metadata`: no `testing` feature is in a `default` chain; no normal edge enables `testing` on a workspace dependency; the only normal test-ish crates are `mockall` (optional, `tracker-core`'s `testing`), `tempfile` in the root crate (out of scope by AC8) and in `frontmatter-validator`, and `testcontainers` in `persistence-benchmark` (both unpublished tools whose binaries use them; out of AC8's library scope).
- Every library `pub mod testing;` (or `environment`) is behind `#[cfg(feature = "testing")]`: `axum-health-check-api-server`, `axum-http-server`, `axum-rest-api-server` and `udp-server`.
- The diagram still drew four test-only edges that rows 4 and 5 had missed (`axum-http --> clock`, `axum-http --> info-hash`, `axum-http --> swarm`, `udp-srv --> swarm`) and the health check's `axum-health --> config`. All five are removed in this change and its note lists them.
- Result: AC8's conditions hold after this change. The criterion is ticked by the #2482 close-out, not here.
