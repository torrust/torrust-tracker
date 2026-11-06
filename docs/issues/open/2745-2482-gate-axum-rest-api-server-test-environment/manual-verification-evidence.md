---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2745-2482-gate-axum-rest-api-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-06 10:20"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). The transcript below is the real output of one script run (`/tmp/row6/verify.sh`, ANSI codes stripped), followed by two control runs and the container recipe check.

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain, `cargo +nightly` for the all-targets check. `cargo-chef` 0.1.79 (`torrust-cargo-chef`, the version pinned in the `Containerfile`) was installed with `cargo install --locked` into a scratch directory; the globally installed `cargo-chef` is 0.1.71 and has no `--external-only`.
- Artifact under test: the branch head `docs(axum-rest-api-server): [#2745] replace the stale test module note and resolve the inventory row`, on top of `torrust/develop` at `6da1883c3` (merge of PR #2746).
- Baseline (T1, `torrust/develop` at `da8bd15d1`, 2026-11-06 09:13 UTC): axum-rest-api-server 162 unit + 58 integration + 0 doc; axum-health-check-api-server 15 + 11 + 0; 35 direct normal dependencies of the REST API package (the spec first said 34: a miscount, corrected in the spec). `git diff da8bd15d1 6da1883c3` has no change in `packages/axum-rest-api-server`, `packages/axum-health-check-api-server` or `deny.toml`, so the baseline holds on the implementation base. All counts below are measured, not derived.

## Results

| Scenario | Result | Where |
| --- | --- | --- |
| M1 No test-only dependency | Direct normal dependencies drop from 35 to 28: `torrust-tracker-core`, `-http-core`, `-udp-core`, `-udp-server`, `-swarm-coordination-registry`, `-rest-api-client` and `torrust-clock` are no longer direct (grep of the depth-1 tree: no match). `cargo check -p torrust-tracker-axum-rest-api-server --lib` with no features exits 0. | V1 |
| M2 Plain package tests | 162 + 58 + 0, equal to the baseline, with and without `--all-features`. | V2 |
| M3 The consumer | 15 + 11 + 0, equal to the baseline, with and without `--all-features`. Without the dev-dependency feature the health check tests fail to compile (`cannot find testing in torrust_tracker_axum_rest_api_server`, observed before the T3 commit). | V3 |
| M4 Shipped features | Exit 0 on the real tree; exit 1 (`validation_error`) with `features = ["testing"]` on the root `[dependencies]` entry; restored (`git diff --quiet`, exit 0). | V4, controls |
| M5 Deny, machete, packaging, nightly | `cargo deny check bans` exit 0, and `explicitly banned` (exit 2) without the self wrapper; `cargo machete --with-metadata` exit 0; `cargo +stable package --workspace --no-verify` exit 0; `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features` exit 0; no `required-features` in any manifest. | V5 |
| M6 Container recipe | With `torrust-cargo-chef` 0.1.79, `prepare --external-only` exit 0 then `cook --tests --workspace --all-features` exit 0 in a scratch directory (only the recipe file was copied there). | V6 |

## Transcript

```text
head: docs(axum-rest-api-server): [#2745] replace the stale test module note and resolve the inventory row
### V1 after: direct normal deps
start 2026-11-06 10:18:40 UTC
torrust-tracker-axum-rest-api-server v0.1.0 (<worktree>/packages/axum-rest-api-server)
├── axum v0.8.9
├── axum-extra v0.12.6
├── axum-server v0.8.0
├── derive_more v2.1.1
├── futures v0.3.34
├── hyper v1.12.0
├── reqwest v0.13.5
├── secrecy v0.10.3
├── serde v1.0.229
├── serde_json v1.0.154
├── subtle v2.6.1
├── thiserror v2.0.21
├── tokio v1.54.1
├── tokio-util v0.7.20
├── torrust-info-hash v0.2.0
├── torrust-metrics v0.2.0
├── torrust-net-primitives v0.1.0
├── torrust-server-lib v0.4.0
├── torrust-tracker-axum-server v0.1.0 (<worktree>/packages/axum-server)
├── torrust-tracker-configuration v3.0.0 (<worktree>/packages/configuration)
├── torrust-tracker-primitives v3.1.0 (<worktree>/packages/primitives)
├── torrust-tracker-rest-api-application v0.1.0 (<worktree>/packages/rest-api-application)
├── torrust-tracker-rest-api-protocol v0.1.0 (<worktree>/packages/rest-api-protocol)
├── torrust-tracker-rest-api-runtime-adapter v0.1.0 (<worktree>/packages/rest-api-runtime-adapter)
├── tower v0.5.3
├── tower-http v0.7.1
├── tracing v0.1.44
└── url v2.5.8
exit=0
end 2026-11-06 10:18:41 UTC
### V1 check: none of the seven is direct
start 2026-11-06 10:18:41 UTC
matches-exit=1 (1 means none)
### V1 lib no features
start 2026-11-06 10:18:41 UTC
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.32s
exit=0
end 2026-11-06 10:18:41 UTC
### V2 rest-api plain
start 2026-11-06 10:18:41 UTC
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_rest_api_server-d996a93232aa2d80)
test result: ok. 162 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.24s
     Running tests/integration.rs (target/debug/deps/integration-5dd195449edd96df)
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.33s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
exit=0
end 2026-11-06 10:18:48 UTC
### V2 rest-api all-features
start 2026-11-06 10:18:48 UTC
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_rest_api_server-d996a93232aa2d80)
test result: ok. 162 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.15s
     Running tests/integration.rs (target/debug/deps/integration-5dd195449edd96df)
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.33s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
exit=0
end 2026-11-06 10:18:52 UTC
### V3 health plain
start 2026-11-06 10:18:52 UTC
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-352cececbccc5068)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
     Running tests/integration.rs (target/debug/deps/integration-7c4b497d2a7407be)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.06s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
exit=0
end 2026-11-06 10:18:58 UTC
### V3 health all-features
start 2026-11-06 10:18:58 UTC
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-352cececbccc5068)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s
     Running tests/integration.rs (target/debug/deps/integration-7c4b497d2a7407be)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
exit=0
end 2026-11-06 10:19:02 UTC
### V4 shipped features
start 2026-11-06 10:19:02 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
exit=0
end 2026-11-06 10:19:02 UTC
### V5 deny
start 2026-11-06 10:19:02 UTC
(warning[duplicate] lines omitted: 33; pre-existing, not part of this change)
(the `warning[unused-wrapper]` below is for the health check crate's own wrapper at deny.toml:35; it is pre-existing: `torrust/develop`'s `deny.toml` prints the same warning)
warning[unused-wrapper]: wrapper for banned crate was not encountered
bans ok
exit=0
end 2026-11-06 10:19:03 UTC
### V5 machete
start 2026-11-06 10:19:03 UTC
Analyzing dependencies of crates in this directory...
cargo-machete didn't find any unused dependencies in this directory. Good job!
Done!
exit=0
end 2026-11-06 10:19:07 UTC
### V5 package
start 2026-11-06 10:19:07 UTC
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging agent-review-report-contract v0.1.0 (<worktree>/contrib/dev-tools/checks/agent-review-report-contract)
    Packaged 5 files, 31.8KiB (5.7KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging ci-gate v0.1.0 (<worktree>/contrib/dev-tools/checks/ci-gate)
    Packaged 11 files, 73.3KiB (17.0KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging clippy-allow-reasons v0.1.0 (<worktree>/contrib/dev-tools/checks/clippy-allow-reasons)
    Packaged 7 files, 38.9KiB (10.0KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging frontmatter-validator v0.1.0 (<worktree>/contrib/dev-tools/checks/frontmatter-validator)
    Packaged 23 files, 190.3KiB (43.4KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging github-review-threads v0.1.0 (<worktree>/contrib/dev-tools/github/github-review-threads)
    Packaged 17 files, 62.6KiB (14.4KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging package-coverage-check v0.1.0 (<worktree>/contrib/dev-tools/checks/package-coverage-check)
    Packaged 7 files, 47.1KiB (10.6KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging shipped-features-check v0.1.0 (<worktree>/contrib/dev-tools/checks/shipped-features-check)
    Packaged 7 files, 34.2KiB (9.6KiB compressed)
   Packaging torrust-tracker-primitives v3.1.0 (<worktree>/packages/primitives)
    Packaged 22 files, 124.8KiB (34.4KiB compressed)
   Packaging torrust-tracker-configuration v3.0.0 (<worktree>/packages/configuration)
    Packaged 35 files, 346.6KiB (79.4KiB compressed)
   Packaging torrust-tracker-axum-server v0.1.0 (<worktree>/packages/axum-server)
    Packaged 11 files, 133.4KiB (37.8KiB compressed)
   Packaging torrust-tracker-http-protocol v0.1.0 (<worktree>/packages/http-protocol)
    Packaged 27 files, 159.9KiB (39.3KiB compressed)
   Packaging torrust-tracker-udp-protocol v0.1.0 (<worktree>/packages/udp-protocol)
    Packaged 14 files, 99.9KiB (26.2KiB compressed)
   Packaging torrust-tracker-client-lib v0.1.0 (<worktree>/packages/tracker-client)
    Packaged 14 files, 84.5KiB (22.7KiB compressed)
   Packaging torrust-tracker-events v0.1.0 (<worktree>/packages/events)
    Packaged 14 files, 80.0KiB (24.3KiB compressed)
   Packaging torrust-tracker-swarm-coordination-registry v0.1.0 (<worktree>/packages/swarm-coordination-registry)
    Packaged 20 files, 207.1KiB (38.0KiB compressed)
   Packaging torrust-tracker-test-helpers v3.0.0 (<worktree>/packages/test-helpers)
    Packaged 10 files, 95.5KiB (29.5KiB compressed)
   Packaging torrust-tracker-core v0.1.0 (<worktree>/packages/tracker-core)
    Packaged 103 files, 649.4KiB (131.6KiB compressed)
   Packaging torrust-tracker-http-core v0.1.0 (<worktree>/packages/http-core)
    Packaged 25 files, 378.1KiB (78.7KiB compressed)
   Packaging torrust-tracker-axum-http-server v0.1.0 (<worktree>/packages/axum-http-server)
    Packaged 41 files, 419.4KiB (87.1KiB compressed)
   Packaging torrust-tracker-rest-api-protocol v0.1.0 (<worktree>/packages/rest-api-protocol)
    Packaged 28 files, 87.3KiB (27.5KiB compressed)
   Packaging torrust-tracker-rest-api-application v0.1.0 (<worktree>/packages/rest-api-application)
    Packaged 18 files, 87.9KiB (26.3KiB compressed)
   Packaging torrust-tracker-rest-api-client v0.1.0 (<worktree>/packages/rest-api-client)
    Packaged 12 files, 89.5KiB (22.9KiB compressed)
   Packaging torrust-tracker-udp-core v0.1.0 (<worktree>/packages/udp-core)
    Packaged 35 files, 317.4KiB (76.6KiB compressed)
   Packaging torrust-tracker-udp-server v0.1.0 (<worktree>/packages/udp-server)
    Packaged 56 files, 780.0KiB (148.8KiB compressed)
   Packaging torrust-tracker-rest-api-runtime-adapter v0.1.0 (<worktree>/packages/rest-api-runtime-adapter)
    Packaged 15 files, 175.4KiB (46.7KiB compressed)
   Packaging torrust-tracker-axum-rest-api-server v0.1.0 (<worktree>/packages/axum-rest-api-server)
    Packaged 56 files, 439.8KiB (95.6KiB compressed)
   Packaging torrust-tracker-axum-health-check-api-server v0.2.0 (<worktree>/packages/axum-health-check-api-server)
    Packaged 18 files, 227.5KiB (58.4KiB compressed)
   Packaging torrust-tracker v3.0.0-develop (<worktree>)
    Packaged 1676 files, 23.8MiB (6.6MiB compressed)
   Packaging torrust-tracker-client v0.1.0 (<worktree>/console/tracker-client)
    Packaged 56 files, 241.3KiB (58.5KiB compressed)
   Packaging torrust-tracker-e2e-tools v0.1.0 (<worktree>/packages/e2e-tools)
    Packaged 8 files, 124.5KiB (33.4KiB compressed)
   Packaging torrust-tracker-persistence-benchmark v0.1.0 (<worktree>/packages/persistence-benchmark)
    Packaged 24 files, 156.1KiB (38.8KiB compressed)
   Packaging torrust-tracker-torrent-repository-benchmarking v0.1.0 (<worktree>/packages/torrent-repository-benchmarking)
    Packaged 34 files, 187.7KiB (30.8KiB compressed)
warning: manifest has no license, license-file, documentation, homepage or repository
   Packaging wait-for-pr-review v0.1.0 (<worktree>/.github/skills/dev/pr-reviews/wait-for-pr-review/tool)
    Packaged 13 files, 105.0KiB (24.9KiB compressed)
warning: manifest has no documentation, homepage or repository
   Packaging workspace-coupling v0.1.0 (<worktree>/contrib/dev-tools/analysis/workspace-coupling)
    Packaged 9 files, 57.9KiB (14.4KiB compressed)
exit=0
end 2026-11-06 10:19:41 UTC
### V5 required-features
start 2026-11-06 10:19:41 UTC
grep-exit=1 (1 means none)
### V5 nightly all-targets
start 2026-11-06 10:19:41 UTC
    Checking torrust-tracker-axum-rest-api-server v0.1.0 (<worktree>/packages/axum-rest-api-server)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.32s
exit=0
end 2026-11-06 10:19:43 UTC
```

Controls (V4 control and V5 control), run after the script:

```text
### V4 control: testing on the root [dependencies] entry
start 2026-11-06 10:19:52 UTC
 1 file changed, 1 insertion(+), 1 deletion(-)
{"kind":"validation_error","message":"workspace package `torrust-tracker-axum-rest-api-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","package":"torrust-tracker-axum-rest-api-server","feature":"testing","exit_code":1}
exit=1
Updated 1 path from the index
restored exit=0
### V5 control: deny without the self wrapper
error[banned]: crate 'torrust-tracker-axum-rest-api-server = 0.1.0' is explicitly banned
exit=2
Updated 1 path from the index
restored exit=0
end 2026-11-06 10:19:53 UTC
```

V6, container recipe:

```text
$ cargo chef --version
torrust-cargo-chef-chef 0.1.79
$ cargo chef prepare --external-only --recipe-path /tmp/row6/recipe.json      # exit 0
$ cd /tmp/row6/scratch && cargo chef cook --tests --workspace --all-features --recipe-path /tmp/row6/recipe.json
    Finished `dev` profile [optimized + debuginfo] target(s) in 29.89s       # exit 0
```
