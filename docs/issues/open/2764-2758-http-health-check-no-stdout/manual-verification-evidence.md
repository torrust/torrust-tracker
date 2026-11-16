---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md
last-updated-utc: 2026-11-16 18:54
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13, and 13:20 for the pseudo-terminal probe recorded under Observed Result
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

The sections are in the order they ran. R0 reproduces the defect, R1 and R2 are the red tests and the mutation checks, and G1, V1 to V3 checked the first implementation. Review rounds 1 to 3 of PR 2889 changed the binary, so the sections "Round 1 of PR 2889" to "Round 3 of PR 2889" at the end hold the results for the final binary.

### R0 - Reproduction of the deviation before the change (http_health_check)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction before the change; the red test, the fix and the recheck are R1, V1 to V3 below)

#### Steps Performed

```text
$ target/debug/http_health_check
exit=1
stdout (0 bytes):
(empty)
stderr (134 bytes):
Usage:   cargo run --bin http_health_check <HEALTH_URL>
Example: cargo run --bin http_health_check http://127.0.0.1:1313/health_check
```

```text
$ target/debug/http_health_check http://127.0.0.1:9/health_check > out 2> err
exit=1
stdout (88 bytes):
Health check ...
ERROR: error sending request for url (http://127.0.0.1:9/health_check)
stderr (0 bytes):
(empty)
```

#### Observed Result

Usage error: plain text on stderr and exit `1` (the contract requires JSON and exit `2`). Unreachable endpoint: plain text on stdout (`Health check ...`, `ERROR: ...`), nothing on stderr, exit `1`. The healthy case was not run here (it needs a running tracker). The same probe inside a pseudo-terminal, run at 13:20 UTC, gives the same output and `rc=1`: `script -qec 'target/debug/http_health_check http://127.0.0.1:9/health_check; rc=$?; echo "rc=$rc"; exit "$rc"' /dev/null` printed `Health check ...`, `ERROR: error sending request for url (http://127.0.0.1:9/health_check)` and `rc=1`, and `script` exited `1`.

### R1 - Red regression tests against the old binary

- Goal: see the tests fail against the unchanged binary before the fix.
- Status: `DONE`

First run on 2026-11-11 (before the rebase onto the newer `torrust/develop`), with the commit `test(health-check): [#2764] add contract tests for http_health_check` alone: `cargo test --test http_health_check` gave `5 failed`.

Re-run on 2026-11-16 at about 17:38 UTC (review round 1 of PR 2889, finding F5 asked for every case): the tests of the current tree against the binary of `torrust/develop` (`git show torrust/develop:src/bin/http_health_check.rs > src/bin/http_health_check.rs`, restored afterwards; see R2 for the restore check). `cargo test --test http_health_check` gave `0 passed; 10 failed`. The assertion that fired, per test:

```text
it_should_exit_0_and_report_healthy_when_the_endpoint_answers_2xx                  stdout must be empty, but was: Health check ...
it_should_exit_1_and_report_unhealthy_when_the_endpoint_answers_a_non_2xx_status   stdout must be empty, but was: Health check ...
it_should_exit_1_and_report_unreachable_when_nothing_listens_on_the_port           stdout must be empty, but was: Health check ...
it_should_exit_2_with_a_usage_error_record_when_the_url_argument_is_missing        unexpected exit code; stderr was: Usage:   cargo run --bin http_health_check <HEALTH_URL> ... left: Some(1) right: Some(2)
it_should_exit_2_with_a_usage_error_record_when_there_are_extra_arguments          unexpected exit code; stderr was: Usage:   cargo run --bin http_health_check <HEALTH_URL> ... left: Some(1) right: Some(2)
it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_a_url          unexpected exit code; stderr was: (empty) left: Some(1) right: Some(2)
it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_valid_utf8     unexpected exit code; stderr was: thread 'main' panicked at .../library/std/src/env.rs:878:51 ... left: Some(101) right: Some(2)
it_should_not_expose_url_credentials_when_the_endpoint_is_healthy                  stdout must be empty, but was: Health check ...
it_should_not_expose_url_credentials_when_the_endpoint_is_unhealthy                stdout must be empty, but was: Health check ...
it_should_not_expose_url_credentials_when_the_endpoint_is_unreachable              stdout must be empty, but was: Health check ...
```

The tests added in round 1 were also run against the binary of the previous round (the commit `test(health-check): [#2764] hold the unreachable port bound without listening`): `5 passed; 5 failed`. The malformed URL gave exit 1 with an `unreachable` record, the non-UTF-8 argument gave exit 101 with the `env.rs` panic, and each credential test failed on the credential check. Excerpts:

```text
it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_a_url: left: Some(1) right: Some(2); stderr was: {"kind":"unreachable","message":"could not probe `not-a-url-path-secret`: builder error","url":"not-a-url-path-secret","exit_code":1}
it_should_not_expose_url_credentials_when_the_endpoint_is_healthy: stderr exposes `probe-user`: {"kind":"healthy","message":"`http://probe-user:userinfo-secret@127.0.0.1:42237/path-secret/health_check?token=query-secret` answered 200 OK","url":"http://probe-user:userinfo-secret@127.0.0.1:42237/path-secret/health_check?token=query-secret","status":200,"exit_code":0}
```

### R2 - Mutation checks

- Goal: show that the tests detect a break of each clause of AC2 and AC3.
- Status: `DONE`

The rows ran at different times. The rows for exit `1` on a non-2xx answer and for JSON on stderr ran on 2026-11-16 at about 17:33 UTC against the round-1 binary, before it was committed (the working-tree file had the same content); the row for exit `2` on a usage error ran at about 17:40 UTC after that commit; the two rows marked "review round 2" ran later, as the paragraph after the table says. Before each mutation the file was copied to the scratchpad; afterwards it was copied back.

| Clause | Mutation | Result of `cargo test --test http_health_check` |
| ------ | -------- | ----------------------------------------------- |
| Exit `1` on a non-2xx answer | the `unhealthy` branch returns `EXIT_HEALTHY` | `8 passed; 2 failed`: the two 503 tests, `left: Some(0)`, `right: Some(1)` |
| Exit `2` on a usage error | `EXIT_USAGE_ERROR` set to `1` | `6 passed; 4 failed`: the four usage-error tests |
| Exit `0` on a 2xx answer | `EXIT_HEALTHY` set to `1` (review round 2) | `9 passed; 2 failed`: the two healthy tests, `left: Some(1)`, `right: Some(0)` |
| Exit `1` on a failed request | the `unreachable` record returns `EXIT_HEALTHY` (review round 2) | `9 passed; 2 failed`: the two unreachable tests, `left: Some(0)`, `right: Some(1)` |
| One JSON object per stderr line | `main` writes `kind=<kind>` as plain text | `0 passed; 10 failed`: `stderr line is not JSON (expected value at line 1 column 1): kind="..."` |

The two rows marked "review round 2" ran on 2026-11-16 at about 18:14 UTC on the round-2 binary (with the scheme check), with no image build running. A first attempt at the exit-0 row, changing the `healthy` branch to return `EXIT_FAILURE`, did not compile, because `EXIT_HEALTHY` was then unused and warnings are errors; that attempt is not counted. After those two rows, `diff -q` of the saved copy printed `identical`.

After the last mutation of round 1, `diff -q` of the saved copy and `src/bin/http_health_check.rs` printed nothing (`identical`), and `git status --short` listed no change under `src/` (only the spec and the new audit folder of this round).

### G1 - Green tests, linters and image test stage

> Historical: this section ran on the first implementation, before review round 1 of PR 2889. Its records still carried a `url` field and the probed URL in `message`, and the test file had 5 tests. The final binary writes no URL, the test file has 11 tests, and the current results are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889" below. The output is kept as it was recorded.

- Status: `DONE`

On the fixed tree (rebased onto `torrust/develop` at the merge of PR 2878, plus the three commits of this issue): `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` both gave `5 passed`; `cargo clippy --bin http_health_check --tests` gave no warnings; `linter all` exited `0` (2026-11-16 16:57 UTC). The release image build (see V2) ran the whole workspace suite in its test stage: `1883 tests run: 1883 passed, 0 skipped`.

The first image build failed in that test stage: the five new tests could not find the binary (`No such file or directory`), because nextest runs from an archive built in another stage and the path compiled into `CARGO_BIN_EXE_http_health_check` does not exist there. The test now reads `NEXTEST_BIN_EXE_http_health_check` first (commit `test(health-check): [#2764] find the binary through the nextest variable in the image test stages`), and the second build passed.

### V1, V3 - Healthy, unhealthy, unreachable and usage error with a real tracker

> Historical: this section ran on the first implementation, before review round 1 of PR 2889. Its records still carried a `url` field and the probed URL in `message`, and the test file had 5 tests. The final binary writes no URL, the test file has 11 tests, and the current results are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889" below. The output is kept as it was recorded.

- Goal: M1 and M3 on the fixed binary, like-for-like with R0.
- Status: `DONE`
- Toolchain: `rustc 1.99.1`; debug build of the rebased tree; the tracker started from `share/default/config/tracker.container.no-persistence.toml` (health API on port 1313), stdout and stderr redirected to files. Run on 2026-11-16 at about 16:56 UTC.

```text
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=0
stdout (0 bytes):
stderr:
{"kind":"healthy","message":"`http://127.0.0.1:1313/health_check` answered 200 OK","url":"http://127.0.0.1:1313/health_check","status":200,"exit_code":0}

$ target/debug/http_health_check http://127.0.0.1:1313/nope
exit=1
stdout (0 bytes):
stderr:
{"kind":"unhealthy","message":"`http://127.0.0.1:1313/nope` answered the non-success status 404 Not Found","url":"http://127.0.0.1:1313/nope","status":404,"exit_code":1}

$ target/debug/http_health_check http://127.0.0.1:9/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe `http://127.0.0.1:9/health_check`: error sending request for url (http://127.0.0.1:9/health_check)","url":"http://127.0.0.1:9/health_check","exit_code":1}

$ target/debug/http_health_check
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: expected exactly one argument; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

(tracker stopped)
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe `http://127.0.0.1:1313/health_check`: error sending request for url (http://127.0.0.1:1313/health_check)","url":"http://127.0.0.1:1313/health_check","exit_code":1}
```

Observed: stdout is empty in every case, each stderr line is one JSON object with a `kind`, and the exit codes are `0`, `1`, `1`, `2`, `1`. The pseudo-terminal probe of R0 (`script -qec ...`) was not repeated: this tool has no TTY refusal (it is `no-stdout-result`), and stdout stays empty regardless.

### V2 - Container HEALTHCHECK

> Historical: this section ran on the first implementation, before review round 1 of PR 2889. Its records still carried a `url` field and the probed URL in `message`, and the test file had 5 tests. The final binary writes no URL, the test file has 11 tests, and the current results are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889" below. The output is kept as it was recorded.

- Goal: M2, the image still reports `unhealthy` when the endpoint fails.
- Status: `DONE`
- Setup: `docker build -f Containerfile --target release -t torrust-tracker-2764:test .` (docker exit `0`, default `CARGO_BUILD_JOBS=4`), run on 2026-11-16 (16:27 to 16:55 UTC).

Healthy container, the shipped no-persistence config mounted at `/etc/torrust/tracker/tracker.toml`: `docker inspect` reported `starting` at 5 s, then `healthy` at 10, 15 and 20 s. The last probe recorded in `.State.Health.Log`:

```text
ExitCode 0, Output: {"kind":"healthy","message":"`http://localhost:1313/health_check` answered 200 OK","url":"http://localhost:1313/health_check","status":200,"exit_code":0}
```

Failing endpoint: a second container with `HEALTH_CHECK_API_PORT=1314` (the tracker serves its health API on 1313, so the probe finds nothing listening). `docker inspect` reported `starting` with `FailingStreak` 0, 1, 2, then `unhealthy` from the 20 s mark (`FailingStreak` 3 to 7 afterwards). The last probe:

```text
ExitCode 1, Output: {"kind":"unreachable","message":"could not probe `http://localhost:1314/health_check`: error sending request for url (http://localhost:1314/health_check)","url":"http://localhost:1314/health_check","exit_code":1}
```

Not used as evidence: `docker pause` of the healthy container. Docker marked it `unhealthy` with `FailingStreak` 0 and the log held only the earlier `healthy` probe, so the pause made Docker flag the container without running the binary. The `docker stop` of the health endpoint alone is not possible (it shares the tracker process), which is why the port mismatch was used. Both containers were removed afterwards.

### Round 1 of PR 2889 - recheck of G1, V1, V2 and V3

- Goal: repeat the checks on the binary changed by review round 1 (no URL in the records, non-UTF-8 and malformed arguments as usage errors, exit code kept when the record cannot be written).
- Status: `DONE`

Tests: `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` report 10 passed; clippy gives no warning.

Image: `docker build -f Containerfile --target release -t torrust-tracker-2764:round1 .` exited `0`, and its test stage reported `1888 tests run: 1888 passed, 0 skipped`. While the build ran, `src/bin/http_health_check.rs` was briefly mutated for R2, so the image's own binary was checked: `docker run --rm --entrypoint /usr/bin/http_health_check torrust-tracker-2764:round1` printed the `usage_error` record with `"exit_code":2` and exited `2`, so the image was not built from the mutated file (that mutation set the usage exit code to 1). That check rules out only the usage-code mutation in the shipped binary. The stage result covers all three round-1 mutations: the test stage builds its tests from the same copied source, and each R2 mutation makes at least two of the `http_health_check` tests fail, while the stage ran 1888 tests with none failing. The evidence template asks to discard a run whose dependent steps ran concurrently; this run is kept on that argument, as review finding `review-finding:pr-2889-f13` suggested, and is not repeated.

Real tracker (2026-11-16, about 17:36 UTC, debug build, same config as V1):

```text
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=0
stdout (0 bytes):
stderr:
{"kind":"healthy","message":"the health endpoint answered 200 OK","status":200,"exit_code":0}

$ target/debug/http_health_check http://probe-user:userinfo-secret@127.0.0.1:1313/health_check?token=query-secret
exit=0
stdout (0 bytes):
stderr:
{"kind":"healthy","message":"the health endpoint answered 200 OK","status":200,"exit_code":0}

$ target/debug/http_health_check http://127.0.0.1:1313/nope
exit=1
stdout (0 bytes):
stderr:
{"kind":"unhealthy","message":"the health endpoint answered the non-success status 404 Not Found","status":404,"exit_code":1}

$ target/debug/http_health_check http://127.0.0.1:9/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe the health endpoint: error sending request: client error (Connect): tcp connect error: Connection refused (os error 111)","exit_code":1}

$ target/debug/http_health_check
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: expected exactly one argument; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

$ target/debug/http_health_check not-a-url
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: the argument is not a valid URL; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

(tracker stopped)
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe the health endpoint: error sending request: client error (Connect): tcp connect error: Connection refused (os error 111)","exit_code":1}
```

Container (2026-11-16, 17:46 to 17:47 UTC, image `torrust-tracker-2764:round1`): a container with the shipped no-persistence config was `healthy` from the 5 s mark; a second one with `HEALTH_CHECK_API_PORT=1314` was `starting` with `FailingStreak` 0, 1, 2, then `unhealthy` from the 20 s mark. A second pair, started to read the probe records, gave:

```text
tt-r1a: healthy failing=0
ExitCode 0 Output: {"kind":"healthy","message":"the health endpoint answered 200 OK","status":200,"exit_code":0}
tt-r1b: unhealthy failing=4
ExitCode 1 Output: {"kind":"unreachable","message":"could not probe the health endpoint: error sending request: client error (Connect): tcp connect error: Connection refused (os error 111)","exit_code":1}
```

All four containers were removed afterwards (`docker ps -a --filter name=tt-r1 -q` printed nothing).

### Round 2 of PR 2889 - URL scheme

- Goal: a URL whose scheme is not `http` or `https` is a usage error (review finding `review-finding:pr-2889-f15`).
- Status: `DONE`

Red, on 2026-11-16 at about 18:13 UTC: the new test `it_should_exit_2_with_a_usage_error_record_when_the_url_scheme_is_not_http_or_https` (it probes `ftp://127.0.0.1/health_check` and `mailto:probe@example.com`) against the round-1 binary gave `10 passed; 1 failed`, with `unexpected exit code; stderr was: {"kind":"unreachable","message":"could not probe the health endpoint: builder error: URL scheme is not allowed","exit_code":1}`, `left: Some(1)`, `right: Some(2)`. Green after the fix: `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` report 11 passed; clippy gives no warning. The image was not rebuilt for this change: it touches only argument parsing, and the container `HEALTHCHECK` (V2) passes an `http` URL, which the scheme check accepts.

### Round 3 of PR 2889 - recheck of V1 and V3 on the final binary

- Goal: repeat the like-for-like real-tracker runs after the last production change (review finding `review-finding:pr-2889-f16`).
- Status: `DONE`
- Build: debug build of the tree whose last production change is the commit `fix(health-check): [#2764] report a URL scheme other than http or https as a usage error` (no change to `src/` after it); the tracker started from `share/default/config/tracker.container.no-persistence.toml`. Run on 2026-11-16 at about 18:29 UTC.

```text
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=0
stdout (0 bytes):
stderr:
{"kind":"healthy","message":"the health endpoint answered 200 OK","status":200,"exit_code":0}

$ target/debug/http_health_check http://probe-user:userinfo-secret@127.0.0.1:1313/health_check?token=query-secret
exit=0
stdout (0 bytes):
stderr:
{"kind":"healthy","message":"the health endpoint answered 200 OK","status":200,"exit_code":0}

$ target/debug/http_health_check http://127.0.0.1:1313/nope
exit=1
stdout (0 bytes):
stderr:
{"kind":"unhealthy","message":"the health endpoint answered the non-success status 404 Not Found","status":404,"exit_code":1}

$ target/debug/http_health_check http://127.0.0.1:9/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe the health endpoint: error sending request: client error (Connect): tcp connect error: Connection refused (os error 111)","exit_code":1}

$ target/debug/http_health_check
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: expected exactly one argument; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

$ target/debug/http_health_check not-a-url
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: the argument is not a valid URL; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

$ target/debug/http_health_check ftp://127.0.0.1:1313/health_check
exit=2
stdout (0 bytes):
stderr:
{"kind":"usage_error","message":"usage error: the URL scheme is not `http` or `https`; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`","exit_code":2}

(tracker stopped)
$ target/debug/http_health_check http://127.0.0.1:1313/health_check
exit=1
stdout (0 bytes):
stderr:
{"kind":"unreachable","message":"could not probe the health endpoint: error sending request: client error (Connect): tcp connect error: Connection refused (os error 111)","exit_code":1}
```

Observed: stdout is empty in every case, each stderr line is one JSON object with a `kind`, no credential of the second URL appears, and the exit codes are `0`, `0`, `1`, `1`, `2`, `2`, `2`, `1`. The new `ftp://` case is a `usage_error` with exit `2`. The image (V2) was not rebuilt: the scheme check runs before any probe and accepts the `http` URL the `HEALTHCHECK` passes.

### Round 4 of PR 2889 - test server accept errors

- Goal: the test server must not hide an `accept` error as `WouldBlock` (review finding `review-finding:pr-2889-f22`).
- Status: `DONE`

The accept loop now sleeps only on `WouldBlock`, retries on `Interrupted`, and panics on any other error; the server's `Drop` re-raises that panic in the test. Green: `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` report 11 passed; clippy gives no warning. Failure path, run on 2026-11-16 at about 18:53 UTC: with the `WouldBlock` guard temporarily changed to `ErrorKind::Unsupported`, so that the listener's `WouldBlock` reaches the panic arm, `cargo test --test http_health_check` gave `7 passed; 4 failed`. The four tests that start a server failed with `the test server could not accept a connection: Resource temporarily unavailable (os error 11)`. The file was restored from a saved copy, and `diff -q` printed `identical`. This change touches only the tests; the binary and its rechecks are unchanged.
