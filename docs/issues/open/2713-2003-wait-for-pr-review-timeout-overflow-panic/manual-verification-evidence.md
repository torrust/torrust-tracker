---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2713-2003-wait-for-pr-review-timeout-overflow-panic/ISSUE.md
last-updated-utc: "2026-11-10 13:28"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-04 11:49 to 11:55
- Artifact under test: `wait-for-pr-review` as built from `develop` at `f7fecdf14`
- Operating system / environment: Linux 7.0.0-38-generic
- Rust toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: `cargo build -p wait-for-pr-review`. The runs use the real `gh` and the
  already merged pull request #2584, so that values that do not panic return `pr_merged` at once. The panic
  happens before the first API call, so the pull request does not matter for the failing values.

## Verification Processes

### V1 - Oversized `--timeout-seconds` (reproduction, before the fix)

- Goal: find out whether, and from which value, `--timeout-seconds` makes the poller panic.
- Initial state: unfixed code.
- Status: `DONE`
- Outcome: **Reproduced**. The wrong outcome itself was observed: exit code `101` and a plain-text panic.

#### Steps Performed

For each value, `target/debug/wait-for-pr-review --pr-number 2584 --timeout-seconds <value>` with stdout and
stderr redirected to files, then the exit code, the size of stdout, and the number of stderr lines that do not
parse as JSON.

#### Observed Result

```text
--timeout-seconds 18446744073709551615 -> exit 101 | stdout=0 bytes  | 5 stderr lines | non-JSON stderr lines: 4
--timeout-seconds 9223372036854775807  -> exit 101 | stdout=0 bytes  | 5 stderr lines | non-JSON stderr lines: 4
--timeout-seconds 4611686018427387904  -> exit 0   | stdout=75 bytes | 1 stderr lines | non-JSON stderr lines: 0
--timeout-seconds 1000000000000000     -> exit 0   | stdout=75 bytes | 1 stderr lines | non-JSON stderr lines: 0
--timeout-seconds 1000000000000        -> exit 0   | stdout=75 bytes | 1 stderr lines | non-JSON stderr lines: 0
--timeout-seconds 100000000000         -> exit 0   | stdout=75 bytes | 1 stderr lines | non-JSON stderr lines: 0
```

Full stderr of the largest value:

```text
{"kind":"wait_started","pr_number":2584,"reviewers":["da2ce7","copilot-pull-request-reviewer[bot]"],"timeout_seconds":18446744073709551615,"poll_interval_seconds":120}

thread 'main' (577270) panicked at .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs:132:57:
overflow when adding duration to instant
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Conclusion

On this machine the panic starts somewhere between 2^62 (`4611686018427387904`, which runs) and the largest
`i64` (`9223372036854775807`, which panics); the exact threshold was not searched for, and it depends on the
platform's `Instant` range. A panic is exit `101` with four plain-text stderr lines, which breaks the documented
`0`/`1`/`2` exit codes and the JSON-only stderr. The panic site is `started + config.timeout` at
`main.rs:132:57`.

### V2 - The same scenario after the fix

- Goal: repeat V1 on the final build.
- Status: `DONE`
- Date and time (UTC): 2026-11-10 13:27
- Build: `cargo build -p wait-for-pr-review` from the working tree on top of the commit with subject
  "fix(wait-for-pr-review): reject --timeout-seconds above one day instead of panicking (#2713)", plus the
  review-round fixes (the `--help` description and the test rename); toolchain `rustc 1.99.1 (58cf8f9c3
  2026-10-13)`; real `gh`, merged pull request #2584.

```text
--timeout-seconds 18446744073709551615 -> exit 2 | stdout=0 bytes  | 1 stderr line (usage_error)
--timeout-seconds 9223372036854775807  -> exit 2 | stdout=0 bytes  | 1 stderr line (usage_error)
--timeout-seconds 86401                -> exit 2 | stdout=0 bytes  | 1 stderr line (usage_error)
--timeout-seconds 86400                -> exit 0 | stdout=75 bytes | 1 stderr line (wait_started)
```

Stderr of the largest value (the one record, shown on one line):

```text
{"kind":"usage_error","message":"error: invalid value '18446744073709551615' for '--timeout-seconds <TIMEOUT_SECONDS>': 18446744073709551615 is not in 1..=86400 ..."}
```

Red run before the fix (the new tests in `tests/cli.rs`, `cargo test -p wait-for-pr-review --test cli timeout_is`):
`it_should_exit_two_with_a_usage_error_record_when_the_timeout_is_too_large_for_any_deadline` failed with
`left: Some(101) right: Some(2)`; `it_should_exit_two_when_the_timeout_is_one_second_above_the_maximum` failed with
`left: Some(0) right: Some(2)`; `it_should_run_to_a_result_when_the_timeout_is_the_maximum` passed (it guards the
inclusive end and passes with or without the fix). `test result: FAILED. 1 passed; 2 failed`.

Green run after the fix (`cargo test -p wait-for-pr-review`, 2026-11-10 13:27 UTC): unit tests `62 passed; 0 failed`,
integration tests `27 passed; 0 failed`; `cargo clippy -p wait-for-pr-review --all-targets` clean. The help test now also
asserts that `--help` keeps the program description and states the maximum.

### V3 - A huge `--poll-interval-seconds` alone does not overflow (scope check)

- Goal: check the claim in the issue specification that the poll interval cannot overflow the deadline.
- Initial state: unfixed code, the real `gh`, and open pull request #2711, so that the wait loop is reached.
- Status: `DONE`

#### Observed Result

```text
--poll-interval-seconds 18446744073709551615 --timeout-seconds 20
  -> {"kind":"timeout","waited_seconds":20,...}  exit 0, real 0m20,002s
--poll-interval-seconds 4611686018427387904 --timeout-seconds 4611686018427387904, SIGTERM sent after 4 s
  -> still running at 4 s: True | exit 1 | non-JSON stderr lines: 0
```

With a 3 s budget the same options ended in `deadline_exceeded` and exit `1`. That is not the poll interval: the
pull request is large and its first snapshot needs more than 3 s, and a normal `--poll-interval-seconds 1` gave
the same record.

#### Conclusion

A huge poll interval is cut to the remaining timeout, so it does not overflow on its own, and with a huge
timeout the wait still honors a signal. The claim in the specification holds.

## Failures and Follow-up

None yet.
