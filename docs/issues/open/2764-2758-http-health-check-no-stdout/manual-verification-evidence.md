---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md
last-updated-utc: 2026-11-16 16:58
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13, and 13:20 for the pseudo-terminal probe recorded under Observed Result
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

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

- Goal: see the new tests fail against the unchanged binary before the fix.
- Status: `DONE`

Run on 2026-11-11 (before the rebase onto the newer `torrust/develop`) with the tests added and the old `src/bin/http_health_check.rs` (the commit `test(health-check): [#2764] add contract tests for http_health_check` alone): `cargo test --test http_health_check` gave `5 failed`. The assertions that fired: stdout was `Health check ...` (not empty) for the healthy and unreachable cases, and the usage cases returned exit `1` where `2` is required (`left: Some(1)`, `right: Some(2)`).

### R2 - Mutation check

- Goal: show the tests detect a wrong exit code.
- Status: `DONE`

Changing `EXIT_USAGE_ERROR` from `2` to `1` in the fixed binary makes `it_should_exit_2_with_a_usage_error_record_when_the_url_argument_is_missing` and `..._when_there_are_extra_arguments` fail (`3 passed; 2 failed`); the change was reverted (`git checkout`), and the tree was clean afterwards.

### G1 - Green tests, linters and image test stage

- Status: `DONE`

On the fixed tree (rebased onto `torrust/develop` at the merge of PR 2878, plus the three commits of this issue): `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` both gave `5 passed`; `cargo clippy --bin http_health_check --tests` gave no warnings; `linter all` exited `0` (2026-11-16 16:57 UTC). The release image build (see V2) ran the whole workspace suite in its test stage: `1883 tests run: 1883 passed, 0 skipped`.

The first image build failed in that test stage: the five new tests could not find the binary (`No such file or directory`), because nextest runs from an archive built in another stage and the path compiled into `CARGO_BIN_EXE_http_health_check` does not exist there. The test now reads `NEXTEST_BIN_EXE_http_health_check` first (commit `test(health-check): [#2764] find the binary through the nextest variable in the image test stages`), and the second build passed.

### V1, V3 - Healthy, unhealthy, unreachable and usage error with a real tracker

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
