---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2724-2482-gate-axum-http-server-test-environment/ISSUE.md
last-updated-utc: "2026-11-05 08:49"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md). The output below is the real output of one script run, reduced to the `test result:` lines, exit codes and error lines (the unit and integration test names are dropped).

## Environment and Prerequisites

- Host: Linux; stable Rust toolchain, `cargo +nightly` for the all-targets check. `cargo-chef` 0.1.79 (`torrust-cargo-chef`, the version pinned in the `Containerfile`) was installed with `cargo install --locked` into a scratch directory; the globally installed `cargo-chef` 0.1.71 has no `--external-only`.
- Artifact under test: the branch head `docs(axum-http-server): [#2724] add a production-style example and label the testing-style one`; the script checks that `packages src tests deny.toml Cargo.toml Cargo.lock` have no change against `HEAD` (`clean-code-paths exit=0`).
- Baseline (T1, `torrust/develop` at `4e23c4356`): http-server 49 unit + 62 integration + 0 doc, health check 15 + 11 + 0, 26 direct normal dependencies. The implementation base `58a3434b1` (merge of PR #2725) has no difference from `4e23c4356` in `packages`, `src`, `tests`, `Cargo.toml`, `Cargo.lock` or `deny.toml` (`git diff --stat` is empty), so the T1 baseline holds for it; no separate run on the unchanged base was made.

## Results

| Scenario | Result | Where |
| --- | --- | --- |
| M1 No test-only dependency | Direct normal dependencies of `axum-http-server` drop from 26 to 23: `torrust-clock`, `torrust-info-hash` and `torrust-tracker-swarm-coordination-registry` are no longer direct. `torrust-clock` stays reachable through `torrust-metrics` and `torrust-tracker-core`. | V1 |
| M2 Plain package tests | 49 + 62 + 0, equal to the baseline, with and without `--all-features`. | V2 |
| M3 The consumer | 15 + 11 + 0, equal to the baseline, with and without `--all-features`. | V3 |
| M4 Shipped features | Exit 0 on the real tree; exit 1 (`validation_error`) with `features = ["testing"]` on the root `[dependencies]` entry; restored (`git diff --quiet`, exit 0). | V4 |
| M5 Examples | Both build (`cargo test --examples`, V5) and both ran under plain `cargo run`: each printed `Listening on 127.0.0.1:<port>`, answered `/health_check` with 200 and an announce with 200, and stopped on SIGTERM printing `Stopped.` (V7). Only the existing one imports `testing` (0 and 1 import lines). | V5, V7 |
| M6 Deny, machete, packaging, nightly | `cargo deny check bans` ok, and `explicitly banned` (exit 2) without the self wrapper; `cargo machete --with-metadata` exit 0; `cargo +stable package --workspace --no-verify` exit 0; `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features` exit 0; no `required-features` in any manifest. | V6 |
| M7 Container recipe | With `torrust-cargo-chef` 0.1.79, `prepare --external-only` exit 0 then `cook --tests --workspace --all-features` exit 0 in a scratch copy of the tree. | V8 |
| Shipped binary | The binary built by `cargo build -p torrust-tracker --bin torrust-tracker` has no `axum-http-server/src/testing` string (count 0); the 35 `cli-configuration` tests pass with `TORRUST_TRACKER_BINARY` set to a copy of it, whose checksum is unchanged by the run. | V9 |
| Dependent | `axum-rest-api-server` runs 156 + 58 + 0 and passes (no pre-change count was recorded for this package, which does not depend on the gated module). | V2 |

## Design-review checkpoint (production-style example)

Compared with `src/bootstrap/jobs/http_tracker.rs` and `src/testing/environment.rs` after the example first ran, as the spec requires:

- Same start call as the application: `HttpServer::new(Launcher::new(bind, tls, ipv6_v6only, drain)).start_with_cancellation(container, form, metadata, token)`, so it uses the same default health check as `start_job`. Differences by design: no TLS (the application loads it with `make_rust_tls`), one instance only, and the example joins the server task and the drain controller itself instead of handing them to the supervisor (`supervise_token_aware_server`) of the application.
- Ownership: two tokens held through drop guards (disarmed on the normal path), the listener is spawned only after the start succeeded, the event receiver is subscribed before the server can publish. Stop order is server, drain controller, then listener, as in the test environment.
- Deadlines: one absolute startup deadline (`timeout_at`) around the container composition and the start; a fresh absolute deadline computed when the signal arrives bounds the stop sequence; waiting for the signal is not bounded.
- Failure path: a start error or timeout returns before any listener exists, prints the error and exits with a non-zero code; every join failure of the stop sequence is reported and the exit code is non-zero.
- Not exercised here: the failure path was not run against a taken port; it follows from the code path above and from the unit tests of the environment.

## Transcript

```text
head: docs(axum-http-server): [#2724] add a production-style example and label the testing-style one
clean-code-paths exit=0
### V1 after: direct normal deps
start 2026-11-05 08:46:27 UTC
exit=0
end 2026-11-05 08:46:27 UTC
### V2 http-server plain
start 2026-11-05 08:46:27 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 29.42s
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 62 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:47:00 UTC
### V2 http-server all-features
start 2026-11-05 08:47:00 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 0.40s
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 62 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:47:04 UTC
### V3 health-check plain
start 2026-11-05 08:47:04 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 27.95s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:47:36 UTC
### V3 health-check all-features
start 2026-11-05 08:47:36 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 0.24s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:47:41 UTC
### V2 rest-api-server (dependent)
start 2026-11-05 08:47:41 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 8.78s
test result: ok. 156 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:47:53 UTC
### V4 shipped features (real tree)
start 2026-11-05 08:47:53 UTC
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
exit=0
end 2026-11-05 08:47:55 UTC
109:torrust-tracker-axum-http-server = { version = "0.1.0", path = "packages/axum-http-server" }
### V4 shipped features (testing on root dependency)
start 2026-11-05 08:47:55 UTC
{"kind":"validation_error","message":"workspace package `torrust-tracker-axum-http-server` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-
exit=1
end 2026-11-05 08:47:56 UTC
Updated 1 path from the index
restored clean exit=0
### V6 deny without self wrapper
start 2026-11-05 08:47:56 UTC
error[banned]: crate 'torrust-tracker-axum-http-server = 0.1.0' is explicitly banned
exit=2
end 2026-11-05 08:47:56 UTC
Updated 1 path from the index
restored clean exit=0
### V6 deny
start 2026-11-05 08:47:56 UTC
bans ok
exit=0
end 2026-11-05 08:47:57 UTC
### V6 machete
start 2026-11-05 08:47:57 UTC
exit=0
end 2026-11-05 08:48:01 UTC
### V5 examples build and test
start 2026-11-05 08:48:01 UTC
    Finished `test` profile [optimized + debuginfo] target(s) in 0.43s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
exit=0
end 2026-11-05 08:48:01 UTC
no required-features: 0
testing imports: prod-style=0 existing=1
```

V1 after (the same script, the first line shows `exit=0` because the helper's count was lost to the `bash -c` quoting); rerun by hand:

```text
$ cargo tree -p torrust-tracker-axum-http-server -e normal --depth 1 | sed 's/ (.*//' | tail -n +2 | grep -c .
23
```

V7, the examples, run by hand for both (build, `cargo run`, `curl /health_check` then an announce, `pkill -TERM`):

```text
http_only_public_tracker_production_style: Listening on 127.0.0.1:51089 / health_check: 200 / announce: 200 / Received SIGTERM. Shutting down... / Stopped.
http_only_public_tracker:                  Listening on 127.0.0.1:59549 / health_check: 200 / announce: 200 / Received SIGTERM. Shutting down... / Stopped.
```

V8, container recipe:

```text
$ cargo chef --version
torrust-cargo-chef-chef 0.1.79
$ cargo chef prepare --external-only --recipe-path <file>      # exit 0
$ cargo chef cook --tests --workspace --all-features --recipe-path <file>   # exit 0, Finished `dev` profile
```

V9, shipped binary:

```text
$ strings <binary> | grep -c 'axum-http-server/src/testing'
0
$ TORRUST_TRACKER_BINARY=<copy> cargo test -p torrust-tracker --test cli-configuration
test result: ok. 35 passed; 0 failed
```
