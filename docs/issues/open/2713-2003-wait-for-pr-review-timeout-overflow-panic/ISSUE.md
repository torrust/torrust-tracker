---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: 2003
github-issue: 2713
spec-path: docs/issues/open/2713-2003-wait-for-pr-review-timeout-overflow-panic/ISSUE.md
branch: "2712-wait-for-pr-review-bug-and-adr-specs-spec"
related-pr: 2802
last-updated-utc: "2026-11-10 13:28"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
    - docs/issues/open/2713-2003-wait-for-pr-review-timeout-overflow-panic/manual-verification-evidence.md
---

<!-- skill-link: create-issue -->

# Issue #2713 - `wait-for-pr-review` panics when `--timeout-seconds` is too large for the deadline

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make every accepted value of `--timeout-seconds` run without a panic, and report a value that cannot be
represented as a usage error in the documented JSON form, with exit code `2`.

## Background

`wait-for-pr-review` (issue #2581) accepts every positive 64-bit integer for `--timeout-seconds` (clap's
`value_parser!(u64).range(1..)`). In `main()` it computes the absolute deadline of the whole wait as
`started + config.timeout`, an `Instant` plus a `Duration`, at
`.github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs:132` (on `develop` at `f7fecdf14`).

What happens, step by step:

1. A caller passes a very large value, for example `--timeout-seconds 18446744073709551615`.
2. The value passes argument parsing, because it is a positive `u64`.
3. The progress record `wait_started` is written to stderr.
4. `started + config.timeout` cannot be represented as an `Instant`, and `Instant`'s `+` panics on overflow.
5. The process exits with code `101` and writes the panic message to stderr as plain text.

This violates the global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`):
exit codes are `0`, `1` and `2`, usage errors are exit `2`, and everything on stderr is JSON, one record per
line. It also bypasses the `usage_error` record every other invalid argument produces.

Impact: a nonsensical argument produces a crash instead of a diagnostic. No data is lost and no request is made,
because the panic happens before the first call. The tool is used by AI agents that read the exit code and the
JSON records, so a plain-text panic and an exit code outside the contract are the visible harm.

The threshold depends on the platform's `Instant` range, so the largest value that works is not the same
everywhere. On the Linux machine used for the reproduction, `4611686018427387904` (2^62) ran and
`9223372036854775807` (the largest `i64`) panicked; see `manual-verification-evidence.md`.

## Scope

### In Scope

- Decide how an oversized `--timeout-seconds` is rejected (see the design question).
- A maintained regression test that makes no API request.
- Update the option table of the `wait-for-pr-review` skill if a maximum is introduced.

### Out of Scope

- Changing the default timeout or the poll interval.
- Other numeric options: `--poll-interval-seconds` is bounded by the remaining timeout in the wait loop and
  cannot overflow the deadline on its own. This was checked (V3 in `manual-verification-evidence.md`): with
  `--poll-interval-seconds 18446744073709551615` and a 20 s timeout the run ended at 20.002 s with a `timeout`
  result, and with both options at 2^62 the run was still waiting after 4 s and ended on `SIGTERM`.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (exit codes `0`/`1`/`2`, JSON
  diagnostics, the `usage_error` record).
- ADRs to create: `None known`.

## Design and Ownership Review

Not applicable beyond the question below: this is argument validation in `main()`; no child process, I/O or
resource lifetime changes.

Design question for the maintainer, to settle at implementation:

- **Option 1, a documented parse-time maximum.** Bound `--timeout-seconds` with the clap range (for example
  one year in seconds) and state the maximum in `--help` and in the skill. Behavior is the same on every
  platform, because the bound sits far below any platform's `Instant` limit, and the error is the existing
  `usage_error`.
- **Option 2, a run-time `checked_add`.** Keep accepting every `u64` and turn `started.checked_add(timeout)`
  returning nothing into a `usage_error`. No arbitrary limit, but the accepted range then depends on the platform
  and on the machine's uptime, and the same command can succeed on one machine and fail on another.

Recommendation: option 1, because a deterministic, documented limit is easier to test and to explain to an agent
than a platform-dependent one. The maximum is the maintainer's choice.

**Decision (maintainer, 2026-11-10):** option 1, with a maximum of **86400 seconds (one day)**, answered by the
maintainer directly in the implementation session (the choices offered were one day, one week and one year,
the last being the spec's example); there is no GitHub comment for it, and this paragraph is the record. The bound is the
clap range `1..=86400` on `--timeout-seconds`; the rejection is the existing `usage_error` (exit `2`).

## Bug-Fix Process

Use [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md). The sequence:

- Analysis: the `Instant` overflow in `main()` described above; the hypothesis is that it is the only overflow
  site on the path from the CLI. The wait loop's own additions use durations bounded by the remaining timeout.
- Real-artifact reproduction: done while drafting; see `manual-verification-evidence.md` section V1. Outcome:
  **Reproduced** with the real binary and no fake `gh`, since the panic happens before any call.
- Regression test boundary and red run: T1.
- Production fix: T2.
- Green run and like-for-like recheck of V1, after the last production change: T3.

## Regression Test Strategy

The smallest deterministic maintained boundary that fails for the defect is an integration test of the binary in
`tests/cli.rs`: run it with `--timeout-seconds 18446744073709551615` and assert exit `2`, empty stdout, and a
single `usage_error` record on stderr that parses as JSON. It needs no fake `gh` response because the argument is
rejected before the first call, and it asserts the observable contract (exit code and record), which is exactly
what the panic breaks. If option 1 is chosen, add one test at the largest accepted value that runs to a result
using the existing fake `gh` and a merged pull request, and one at the smallest rejected value. A unit test of
the parser is welcome if the bound lives in a function.

The test must be run red against the unfixed code (expected: exit `101`) and green after.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                          | Notes / Expected Output                                                    |
| --- | ------ | ------------------------------------------------------------- | -------------------------------------------------------------------------- |
| T1  | DONE   | Add the integration test for the oversized value; run it red  | Red output (exit `101`) recorded in `manual-verification-evidence.md`.     |
| T2  | DONE   | Implement the chosen option; add boundary tests for option 1  | Oversized value is a `usage_error` with exit `2`.                          |
| T3  | DONE   | Run the tests green, then repeat V1 on the final build        | Green output and the recheck recorded, naming the commit subject it ran on.|
| T4  | DONE   | Update the skill option table and `--help` text               | The maximum, if any, is documented once.                                   |

## Commit Points

| Task | Coherent change set                                   | Commit policy                                        |
| ---- | ----------------------------------------------------- | ---------------------------------------------------- |
| T1   | The failing test (fix applied in the next commit)     | Commit with the fix if the repository requires green commits, otherwise commit red first and say so. |
| T2   | The validation and its boundary tests                 | Commit after focused validation and required review. |
| T3, T4 | Evidence and documentation                          | Commit after focused validation.                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2713-2003-wait-for-pr-review-timeout-overflow-panic/ISSUE.md`
- [x] Reproduction attempted and recorded before review (outcome: Reproduced)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#2713) and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Final like-for-like recheck of V1 recorded after the last production change
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-04 12:10 UTC - AI assistant (Copilot SDK in VS Code) - Drafted the specification and reproduced the
  defect on the real binary: `--timeout-seconds 18446744073709551615` and `9223372036854775807` exit `101` with
  a plain-text panic; `4611686018427387904` runs - `manual-verification-evidence.md` V1.
- 2026-11-04 12:12 UTC - AI assistant (Copilot SDK in VS Code) - The maintainer approved the specification; GitHub issue #2713 created, labelled `Bug` and `Automation`, and linked as a sub-issue of EPIC #2003; the specification moved from `docs/issues/drafts/` to `docs/issues/open/` for a spec-only pull request.
- 2026-11-04 13:07 UTC - AI assistant (Copilot SDK in VS Code) - Correction: the drafting entry above (12:10) was stamped before the commands it
  describes ran. The reproduction and lint commands ran between 11:49 and 11:56 UTC, as the command output and
  `manual-verification-evidence.md` record, so the drafting stamp is wrong. The earlier entries are kept unchanged,
  and `last-updated-utc` now holds the time of this edit.
- 2026-11-10 - AI assistant (Claude Code) - The maintainer chose option 1 with a maximum of 86400 s. T1: the new
  tests failed on the unfixed code (oversized value exit `101`, 86401 exit `0`). T2/T4: added
  `MAX_TIMEOUT_SECONDS = 86_400` as the clap range upper bound, updated the `--help` text and the skill table.
  T3: `cargo test -p wait-for-pr-review` and clippy pass; V2 recorded.
- 2026-11-10 13:28 UTC - AI assistant (Claude Code) - Review round 1 (da2ce7): moved `MAX_TIMEOUT_SECONDS` above the
  `Cli` doc comment so `--help` keeps its description, and added a help-text assertion; renamed `gh_calls` to
  `pull_calls`; V2 rerun on the new build and recorded with the green output; this bookkeeping updated.

## Acceptance Criteria

- [x] AC1: `--timeout-seconds 18446744073709551615` exits `2` with empty stdout and one `usage_error` JSON record
      on stderr, and makes no API request.
- [x] AC2: No value the CLI accepts can make `main()` panic; the accepted range and the rejection behavior are
      the same on every supported platform (option 1) or the rejection is reported as a usage error on the
      platform where it happens (option 2).
- [x] AC3: The skill and `--help` state the maximum, if one is introduced.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] The final recheck of V1 is recorded in `manual-verification-evidence.md`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p wait-for-pr-review`
- `cargo clippy -p wait-for-pr-review --all-targets`
- Pre-push checks

### Manual Verification Scenarios

| ID | Scenario                          | Human-oriented command/steps                                                           | Expected Result                                  | Status | Evidence                                     |
| -- | --------------------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------ | ------ | -------------------------------------------- |
| M1 | Oversized timeout (before)        | `wait-for-pr-review --pr-number 2584 --timeout-seconds 18446744073709551615`           | Exit `101`, plain-text panic on stderr           | DONE   | `manual-verification-evidence.md` section V1 |
| M2 | The same, after the fix           | The same command on the final build                                                    | Exit `2`, one `usage_error` record, no panic     | DONE   | `manual-verification-evidence.md` section V2 |

Every recorded command result names the toolchain that produced it.

### Disposable Verification Scripts

None. The reproduction is a single command on the real binary.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | Test `it_should_exit_two_with_a_usage_error_record_when_the_timeout_is_too_large_for_any_deadline` (asserts no `gh` call); V2 |
| AC2   | DONE                   | The bound `1..=86400` is far below any `Instant` range and platform independent; boundary tests at 86400 and 86401; V2 |
| AC3   | DONE                   | `--help` text and the skill option table state the maximum |

## Risks and Trade-offs

- A maximum rejects values that some platform could have run. Mitigation: pick a maximum far above any
  realistic wait (the longest wait an agent needs is hours); a wait of years is not a use case.
- A run-time check without a maximum keeps every value but makes behavior platform-dependent (option 2).

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Related issues: #2581 (closed, implemented the poller), #2003 (parent EPIC)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
