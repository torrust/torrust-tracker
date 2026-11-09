---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2712-2003-wait-for-pr-review-pipe-wait-after-exit/ISSUE.md
last-updated-utc: "2026-11-09 10:48"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-04 11:49 to 11:50
- Artifact under test: `wait-for-pr-review` as built from `develop` at `f7fecdf14` (the binary was built
  after the merge of issue #2581's pull request and is unchanged by later commits)
- Operating system / environment: Linux 7.0.0-38-generic
- Rust toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: `cargo build -p wait-for-pr-review`. The fake `gh` is
  `evidence/fake-gh-leaves-pipe-open.sh`, copied as `gh` into a temporary directory placed first on `PATH`. It
  answers only `gh auth status`, the first call the poller makes, so no run reaches the GitHub API.

## Verification Processes

### V1 - A descendant holds the pipes after `gh` exits (reproduction, before the fix)

- Goal: show whether the timeout and the signal handling hold when `gh` exits successfully but a descendant keeps
  its stdout and stderr open.
- Initial state: unfixed code; the fake `gh` exits at once and leaves a background `sleep 25` holding the pipes.
- Status: `DONE`
- Outcome: **Reproduced**. The wrong outcome itself was observed: the run outlived its budget.

#### Steps Performed

1. Control: the fake `gh` run alone, with its output sent to `/dev/null`, to show that the script itself exits at
   once and does not make the poller wait by itself.
2. Run 1: `wait-for-pr-review --pr-number 7 --timeout-seconds 3`, with the fake `gh` first on `PATH`.
3. Run 2: `wait-for-pr-review --pr-number 7 --timeout-seconds 900`, `SIGTERM` sent after 3 s.

#### Observed Result

```text
control: gh auth status > /dev/null 2>&1    real 0m0,002s
run 1: exit 1 after 25.0 s (budget 3 s)
stdout bytes: 0 | last stderr record: {"kind":"deadline_exceeded","message":"the wait budget ended while a `gh` call was still running"}
run 2: SIGTERM sent at 3.0 s; exit 1 after 25.0 s
stdout bytes: 0 | last stderr record: {"kind":"interrupted","message":"interrupted by a signal"}
```

#### Conclusion

The fake `gh` exits in 2 ms, yet the poller took 25.0 s for a 3 s budget, which is the lifetime of the
background `sleep`, and honored a `SIGTERM` sent at 3.0 s only at 25.0 s. The records it finally wrote
(`deadline_exceeded`, `interrupted`) are the correct ones; they arrive too late. The wait is in `collect(reader,
None)` on the success path of `run()`, which the error path avoids with its 200 ms grace period. This was not
observed with the real `gh`.

### V2 - The same scenario after the fix

- Goal: repeat V1 on the final build.
- Status: `DONE`
- Outcome: **Fixed**. Both runs end at their bound.

#### Steps Performed

Same as V1, with `target/debug/wait-for-pr-review` built from the branch `2712-wait-for-pr-review-pipe-wait`
on top of commit `docs(wait-for-pr-review): [#2712] record the partial-output design decision` plus the
uncommitted fix (committed afterwards as `fix(wait-for-pr-review): [#2712] bound the reader wait by the
deadline and signals`). Toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, Linux 7.0.0-38-generic.

#### Observed Result

```text
run 1: exit 1 after 3.0 s (budget 3 s)
stdout bytes: 0 | last stderr record: {"kind":"deadline_exceeded","message":"the wait budget ended while a `gh` call was still running"}
run 2: SIGTERM sent at 3.0 s; exit 1 after 3.0 s
stdout bytes: 0 | last stderr record: {"kind":"interrupted","message":"interrupted by a signal"}
```

#### Conclusion

The run that took 25.0 s for a 3 s budget now ends at 3.0 s, and the `SIGTERM` sent at 3.0 s is honored at
3.0 s, with the same records as before.

### T1 and T2 - Regression tests, red then green

Fake `gh` leaves `sleep 20` holding its pipes (`DESCENDANT_LIFETIME`); the assertions require the run to end
in under 10 s. `cargo test -p wait-for-pr-review --test cli descendant`:

```text
Red (unfixed code):
test it_should_stop_on_sigterm_when_gh_exits_but_a_descendant_keeps_its_pipes_open ... FAILED
test it_should_end_at_the_deadline_when_gh_exits_but_a_descendant_keeps_its_pipes_open ... FAILED
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 22 filtered out; finished in 20.01s

Green (fixed code): cargo test -p wait-for-pr-review
test result: ok. 24 passed; 0 failed (cli)   plus the unit tests of `collect`, all passing
```

### V3 - Like-for-like recheck on the final build

- Goal: repeat V1 after the last production change, which is pull request #2756.
- Status: `DONE`
- Outcome: **Fixed**. Same result as V2.

#### Steps Performed

Same as V1 and V2, with `target/debug/wait-for-pr-review` built from `develop` at `92665ae17` (`Merge
torrust/torrust-tracker#2756`), the fix commit of #2750 and the follow-up of #2756 both included. Toolchain: stable
`rustc 1.99.1 (58cf8f9c3 2026-10-13)`, Linux 7.0.0-43-generic.

#### Observed Result

```text
run 1: exit 1 after 3.0 s (budget 3 s)
stdout bytes: 0 | last stderr record: {"kind":"deadline_exceeded","message":"the wait budget ended while a `gh` call was still running"}
run 2: SIGTERM sent at 3.0 s; exit 1 after 3.0 s
stdout bytes: 0 | last stderr record: {"kind":"interrupted","message":"interrupted by a signal"}
cargo test -p wait-for-pr-review: 62 unit and 24 integration tests passed, 0 failed
```

## Failures and Follow-up

None. Review findings of #2750 and #2756 are recorded in `docs/pr-reviews/pr-2750-review/PR-REVIEW.md` and `docs/pr-reviews/pr-2756-review/PR-REVIEW.md`.
